{-# LANGUAGE FlexibleInstances #-}
{-# LANGUAGE LambdaCase        #-}
{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE TupleSections     #-}
{-# LANGUAGE TypeFamilies      #-}
module ToothpikBroker.NoopAuthenticator where
--------------------------------------------------------------------------------
-- |
-- Module      :  ToothpikBroker.NoopAuthenticator
-- Copyright   :  (c) AeroStun (for ItJustWorksTM) 2020
-- License     :  Apache-2.0
--
-- Stability   :  experimental
--------------------------------------------------------------------------------

import           Control.Exception
import           Data.Aeson.Types
import qualified Data.Attoparsec.ByteString         as ABS
import qualified Data.ByteString.Char8              as C
import           Data.Typeable
import qualified Data.UUID.V4                       as UUIDv4
import qualified System.Log.Logger                  as Log

import           Network.MQTT.Broker.Authentication
import           Network.MQTT.Message
import qualified Network.MQTT.Trie                  as R

data NoopAuthenticator = NoopAuthenticator

instance Authenticator NoopAuthenticator where
  data AuthenticatorConfig NoopAuthenticator
     = NoopAuthenticatorConfig deriving (Show)

  data AuthenticationException NoopAuthenticator
     = NoopAuthenticationException String deriving (Eq, Ord, Show, Typeable)

  newAuthenticator _ = do
    Log.infoM "NoopAuthenticator" "Creating new NoopAuthenticator."
    pure NoopAuthenticator

  authenticate _ _ = do
      Log.infoM "NoopAuthenticator" "Authenticating"
      v <- UUIDv4.nextRandom
      return $ Just v

  getPrincipal _ _ = do
      Log.infoM "NoopAuthenticator" "Acquiring principal"
      let f = ABS.parseOnly (filterParser <* ABS.endOfInput) (C.pack "#") 
      return $ case f of
        Left _ -> Nothing
        Right r -> Just Principal
          { principalUsername = Nothing
          , principalQuota    = Quota
            { quotaMaxSessions          = 1
            , quotaMaxIdleSessionTTL    = 60
            , quotaMaxPacketSize        = 65535
            , quotaMaxPacketIdentifiers = 10
            , quotaMaxQueueSizeQoS0     = 100
            , quotaMaxQueueSizeQoS1     = 100
            , quotaMaxQueueSizeQoS2     = 100
            }
          , principalPublishPermissions = all_trie
          , principalSubscribePermissions = all_trie
          , principalRetainPermissions = all_trie
          }
          where
            all_trie = R.insert r () R.empty
      
  getLastException _ = pure Nothing


instance Exception (AuthenticationException NoopAuthenticator)

instance FromJSON (AuthenticatorConfig NoopAuthenticator) where
  parseJSON Null = pure NoopAuthenticatorConfig
  parseJSON invalid = typeMismatch "NoopAuthenticatorConfig" invalid

      

