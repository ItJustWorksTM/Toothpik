# Toothpik Broker

This component is the broker part of the Toothpik system. It consists of a customized version of the Hummingbird broker.

Building requires Haskell Stack.

On Un*x, `./setup.sh` can be used to setup the build environment, followed by `cd build; stack build` to build the broker, and finally `stack run -- broker --config=./config-dev.yml` to run the broker.

Hummingbird only supports MQTTv3, which is plenty for this system, but supports MQTT over both TCP and WebSockets.
