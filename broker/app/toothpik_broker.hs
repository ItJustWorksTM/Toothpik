--------------------------------------------------------------------------------
-- |
-- Module      :  Main
-- Copyright   :  (c) AeroStun (for ItJustWorksTM) 2020
-- License     :  Apache-2.0
--
-- Stability   :  experimental
--------------------------------------------------------------------------------
module Main where

import           Data.Version                    (showVersion)

import           Hummingbird
import           ToothpikBroker.ActiveAuthenticator

import           Paths_hummingbird               (version)

main :: IO ()
main =
  runWithVendorSettings settings
  where
    settings = VendorSettings {
      vendorVersionName = showVersion version
    } :: VendorSettings ActiveAuthenticator

