{-# LANGUAGE FlexibleInstances   #-}
{-# LANGUAGE LambdaCase          #-}
{-# LANGUAGE MultiWayIf          #-}
{-# LANGUAGE OverloadedStrings   #-}
{-# LANGUAGE TupleSections       #-}
{-# LANGUAGE TypeFamilies        #-}
{-# LANGUAGE ScopedTypeVariables #-}
module ToothpikBroker.ActiveAuthenticator where
--------------------------------------------------------------------------------
-- |
-- Module      :  ToothpikBroker.ActiveAuthenticator
-- Copyright   :  (c) AeroStun (for ItJustWorksTM) 2020
-- License     :  Apache-2.0
--
-- Stability   :  experimental
--------------------------------------------------------------------------------

import           Codec.CBOR.Decoding
import           Codec.CBOR.Encoding
import           Codec.CBOR.Read
import           Codec.CBOR.Write
import           Control.Exception
import           Control.Concurrent.STM
import           Control.Monad                                 (when)
import           Control.Monad.Catch                           (bracket, throwM)
import           Control.Monad.STM
import           Crypto.OTP
import           Data.Aeson.Types
import qualified Data.Attoparsec.ByteString         as ABS
import qualified Data.ByteString                    as BS
import qualified Data.ByteString.Builder            as BSB     (lazyByteStringHex, toLazyByteString)
import qualified Data.ByteString.Char8              as C8
import qualified Data.ByteString.Lazy               as BL
import           Data.ByteString.Base32
import qualified Data.Cache                         as C
import           Data.Coerce
import           Data.Either
import           Data.Maybe
import           Data.String.Conversions
import           Data.Text                          as Txt     (Text, pack, unpack, isPrefixOf, empty)
import qualified Data.Text.Lazy.Encoding            as E
import           Data.Text.Read
import           Data.Time.Clock.POSIX
import           Data.Typeable
import           Data.UUID
import qualified Data.UUID.V4                       as UUIDv4
import qualified System.Log.Logger                  as Log
import qualified StmContainers.Map                  as TM

import           Network.MQTT.Broker.Authentication
import           LocallyBounced.Network.MQTT.Client
import           Network.MQTT.Message
import qualified Network.MQTT.Topic                 as T
import qualified Network.MQTT.Trie                  as R
import           Network.URI

type ConnectUUID = UUID
type SystemUUID = UUID
type LeanUsername = Txt.Text
type Secret = BL.ByteString

data Clearance
    = AuthClearance    { authcl_uuid :: SystemUUID }
    | StoreClearance   { st_username :: LeanUsername }
    | ClientClearance  { cl2_uuid :: SystemUUID, cl2_username :: LeanUsername }
    | AnonClearance
    | WatcherClearance { watch_username :: LeanUsername }

type AuthReplyPayload = Maybe (SystemUUID, Secret) 
data AuthReply
    = AuthReply
    { reply_username :: LeanUsername
    , reply_payload :: AuthReplyPayload
    }

    
type AuthCache = C.Cache LeanUsername (SystemUUID, Secret)
type HandleMap = TM.Map LeanUsername (AuthReplyPayload -> IO())

data ActiveAuthenticator
    = ActiveAuthenticator
    { cache :: AuthCache
    , clearances :: TM.Map ConnectUUID Clearance
    , lclient :: TVar (Maybe MQTTClient)
    , lclient_uuid :: ConnectUUID
    , lclient_config :: MQTTConfig
    , lclient_handlers :: HandleMap
    }

instance Authenticator ActiveAuthenticator where
  data AuthenticatorConfig ActiveAuthenticator
     = ActiveAuthenticatorConfig deriving (Show)

  data AuthenticationException ActiveAuthenticator
     = ActiveAuthenticationException String deriving (Eq, Ord, Show, Typeable)

  newAuthenticator _ = do
    Log.infoM "ActiveAuthenticator" "Creating new ActiveAuthenticator."
    cache' <- C.newCache Nothing
    client <- atomically $ newTVar (Nothing :: Maybe MQTTClient)
    uuid <- UUIDv4.nextRandom
    Log.infoM "ActiveAuthenticator" $ "We are " ++ toString uuid
    clearances' <- atomically $ TM.new
    hdls <- atomically $ TM.new
    return ActiveAuthenticator
        { cache = cache'
        , clearances = clearances'
        , lclient = client
        , lclient_uuid = uuid
        , lclient_config = mqttConfig
            { _msgCB = SimpleCallback (cb uuid hdls)
            , _protocol = Protocol311
            }
        , lclient_handlers = hdls
        }
    where
      cb :: UUID -> HandleMap -> MQTTClient -> T.Topic -> BL.ByteString -> [Property] -> IO ()
      cb uuid hdls _ topic body _ = do
        Log.infoM "ActiveAuthenticator" $ "LClient got packet from" ++ (show topic)
        let secret_reply_topic = "auth/" ++ (toString uuid) ++ "/reply/store/user/secret"
        if (unpack topic) == secret_reply_topic
          then case (deserialiseFromBytes decodeReply body) of
              Left _ -> Log.infoM "ActiveAuthenticator" $ "LClient got bad auth reply packet " ++ (show $ BSB.toLazyByteString $ BSB.lazyByteStringHex $ body)
              Right ("", reply) -> lookupUUID >>= applyOrNoop
                where
                  lookupUUID :: IO (Maybe (AuthReplyPayload -> IO ()))
                  lookupUUID = atomically $ TM.lookup (reply_username reply) hdls
                  applyOrNoop :: Maybe (AuthReplyPayload -> IO ()) -> IO ()
                  applyOrNoop m = fromMaybe (pure ()) $ m <*> (Just $ reply_payload reply)
              _ -> pure ()
            else pure ()


  authenticate auth req = do
      Log.infoM "ActiveAuthenticator" $ "Authenticating " ++ (show $ requestClientIdentifier req)
      let ident = coerce $ requestClientIdentifier req
      let in_creds = requestCredentials req
      let mb_uuid = fromText ident
      case (mb_uuid, in_creds) of
        (Nothing, _) -> pure Nothing
        (Just uuid, Nothing)
          | ((==) uuid $ lclient_uuid auth) -> setClearance uuid $ AuthClearance uuid
          | otherwise -> setClearance uuid AnonClearance
        (Just uuid, Just (username, Just password))
          | "watcher-" `Txt.isPrefixOf` uname -> setClearance uuid $ WatcherClearance uname
          | "store-" `Txt.isPrefixOf` uname -> setClearance uuid $ StoreClearance uname
          | otherwise -> do
            mb_clearance <- atomically $ TM.lookup uuid (clearances auth)
            case mb_clearance of
              Just AnonClearance -> do
                now <- getPOSIXTime >>= \t -> return (floor t :: OTPTime)
                creds <- pullCache >>= orQueryStore
                case creds of
                  Just (sys_uuid, secret) ->
                    case validateSecret secret (BL.fromStrict $ coerce password) now of
                      Right () -> do
                        Log.infoM "ActiveAuthenticator" $ "Switching " ++ (toString uuid) ++ " into " ++ (toString sys_uuid)
                        setClearance uuid $ ClientClearance sys_uuid uname
                      Left (err) -> do
                        Log.infoM "ActiveAuthenticator" $ "Login failed: " ++ err
                        pure Nothing
                  Nothing -> pure Nothing
              _ -> pure Nothing
          where
            uname :: LeanUsername
            uname = coerce username
            validateSecret :: Secret -> Secret -> OTPTime -> Either String ()
            validateSecret server_secret client_secret time =
              let ((srvPw, srvSecretEnc), (cltPw, cltCodeStr)) = (split server_secret, split client_secret)
                  cltCode = maybeParseCode cltCodeStr
                  srvSecret = decodeSecret <$> srvSecretEnc
              in if 
                | srvPw /= cltPw -> Left "Mismatched passwords"
                | isJust srvSecret /= isJust cltCode -> Left "Server has 2FA but not client"
                | isNothing srvSecret -> Right ()
                | totpVerify defaultTOTPParams (fromJust srvSecret) time (fromJust cltCode) -> Right ()
                | otherwise -> Left "Invalid 2FA code"
              where
                split :: Secret -> (Secret, Maybe Secret)
                split s =
                  case BL.elemIndex 0x0A s of
                    Nothing -> (s, Nothing)
                    Just i -> let (sec, code) = BL.splitAt i s in (sec, Just (BL.drop 1 code))
                decodeSecret :: Secret -> BS.ByteString
                decodeSecret s = let Right v = decodeBase32 $ BL.toStrict s in v
                maybeParseCode :: Maybe BL.ByteString -> Maybe OTP
                maybeParseCode Nothing = Nothing
                maybeParseCode (Just s) = 
                  case decimal $ cs s of
                      Left _ -> Nothing
                      Right (v, r)
                        | r == Txt.empty -> Just v
                        | otherwise -> Nothing
            pullCache ::  IO AuthReplyPayload
            pullCache = C.lookup (cache auth) uname
            orQueryStore :: AuthReplyPayload -> IO AuthReplyPayload
            orQueryStore (Just s) = pure $ Just s
            orQueryStore Nothing = rpcCreds topic $ body
              where
                topic :: Txt.Text
                topic = Txt.pack $ "store/user/" ++ (toString $ lclient_uuid auth) ++ "/secret"
                body :: BL.ByteString
                body = toLazyByteString $ encodeMapLen 1 <> encodeString "username" <> encodeString uname
                rpcCreds :: T.Topic -> BL.ByteString -> IO AuthReplyPayload
                rpcCreds outtop body = do
                  client <- getLClient
                  Control.Exception.bracket reg unreg (rt client)
                  where
                    hdls :: HandleMap
                    hdls = lclient_handlers auth
                    getLClient :: IO MQTTClient
                    getLClient = do
                      let lclient_uuid' = lclient_uuid auth
                      val <- readTVarIO $ lclient auth
                      case val of
                        Just v -> pure v
                        Nothing -> do
                          client <- connectURI (lclient_config auth) $ thisURI lclient_uuid'
                          () <- atomically $ writeTVar (lclient auth) $ Just client
                          _ <- subscribe client (Prelude.map (, subOptions) $ inbound_routes lclient_uuid') mempty
                          return client
                    reg :: IO (TChan AuthReplyPayload)
                    reg = newTChanIO >>= (\r -> ((atomically $ TM.insert (cb r) uname hdls) *> pure r))
                    unreg :: TChan AuthReplyPayload -> IO ()
                    unreg _ = atomically $ TM.delete uname hdls
                    cb :: TChan AuthReplyPayload -> AuthReplyPayload -> IO ()
                    cb r v = atomically $ writeTChan r v
                    rt client r = do
                      publishq client outtop body False LocallyBounced.Network.MQTT.Client.QoS1 mempty
                      atomically $ do
                        connd <- isConnectedSTM client
                        when (not connd) $ throwM (MQTTException "disconnected")
                        readTChan r
        _ -> pure Nothing
    where
      setClearance :: ConnectUUID -> Clearance -> IO (Maybe ConnectUUID)
      setClearance uuid cl = (pure $ Just uuid) <* (atomically $ TM.insert cl uuid $ clearances auth)
      

        
  getPrincipal auth pid = do
      Log.infoM "ActiveAuthenticator" $ "Acquiring principal for " ++ (toString pid)
      (toPrincipal pid) <$$> (atomically $ TM.lookup pid $ clearances auth)


  getLastException _ = pure Nothing


instance Exception (AuthenticationException ActiveAuthenticator)

instance FromJSON (AuthenticatorConfig ActiveAuthenticator) where
  parseJSON Null = pure ActiveAuthenticatorConfig
  parseJSON invalid = typeMismatch "ActiveAuthenticatorConfig" invalid

decodeReply :: Decoder s AuthReply
decodeReply = do
  mlen <- decodeMapLen
  uname_key <- decodeString
  uname <- decodeString
  next_key <- decodeString
  tok <- peekTokenType
  case (mlen, uname_key, next_key, tok) of
    (2, "username", "error", TypeNull) -> pure $ AuthReply uname Nothing
    (3, "username", "user_id", TypeString) -> do
      user_id <- decodeString
      let (Just uuid) = fromText user_id
      secret_key <- decodeString
      secret <- decodeString
      if (secret_key == "secret")
        then pure $ AuthReply uname $ Just (uuid, cs secret)
        else fail "Invalid reply"
    _ -> fail "Invalid reply"

toPrincipal :: ConnectUUID -> Clearance -> Principal
toPrincipal conn_uuid cl =
  case cl of
    WatcherClearance uname -> mkPrincipal (Just $ Username uname) ["#"] ["#"]
    AuthClearance uuid -> mkPrincipal Nothing ["auth/" ++ all_for uuid, "auth/cache/#"] ["store/user/" ++ (toString uuid) ++ "/secret"]
    StoreClearance uname -> mkPrincipal (Just $ Username uname) ["store/#", "client/#"] ["#"]
    ClientClearance uuid uname -> mkPrincipal (Just $ Username uname) ["client/" ++ all_for uuid, "store/+/public/realtime/#", "store/+/realtime/#", "client/" ++ (toString conn_uuid) ++ "/reply/store/user/my_id"] ["store/+/" ++ all_for uuid, "store/user/" ++ (toString conn_uuid) ++ "/my_id"]
    AnonClearance -> mkPrincipal Nothing ["client/" ++ (toString conn_uuid) ++ "/#", "store/+/public/realtime/#"] ["store/+/public/" ++ all_for conn_uuid]
  where
    all_for :: UUID -> String
    all_for uuid = (toString uuid) ++ "/#"
    mkPrincipal :: Maybe Username -> [String] -> [String] -> Principal
    mkPrincipal mb_uname sub pub = Principal
      { principalUsername = mb_uname
      , principalQuota    = quota
      , principalPublishPermissions = packTrie pub
      , principalSubscribePermissions = packTrie sub
      , principalRetainPermissions = R.empty
      }
    packTrie :: [String] -> R.Trie ()
    packTrie a = insertFoldable (map (\e -> (parseRule e, ())) a) R.empty
      where insertFoldable = flip $ foldr $ uncurry R.insert
    quota = Quota
      { quotaMaxSessions          = 1
      , quotaMaxIdleSessionTTL    = 60
      , quotaMaxPacketSize        = 65535
      , quotaMaxPacketIdentifiers = 10
      , quotaMaxQueueSizeQoS0     = 100
      , quotaMaxQueueSizeQoS1     = 100
      , quotaMaxQueueSizeQoS2     = 100
      }
    parseRule :: String -> Filter
    parseRule rule =
      let Right f = ABS.parseOnly (filterParser <* ABS.endOfInput) (C8.pack rule)
      in f

(<$$>) :: (Functor f, Functor g) => (a -> b) -> f (g a) -> f (g b)
(<$$>) m v = (fmap m) <$> v

thisURI :: UUID -> URI
thisURI uuid =
  let Just (uri) = parseURI $ "mqtt://127.0.0.1:1883#" ++ (toString uuid)
  in uri

inbound_routes :: UUID -> [Txt.Text]
inbound_routes uuid = Prelude.map Txt.pack
  [ "auth/" ++ (toString uuid) ++ "/reply/store/user/secret" 
  , "auth/cache/add"
  , "auth/cache/del"
  , "auth/cache/flush"
  ]
