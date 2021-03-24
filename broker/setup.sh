#!/bin/bash

cp -r upstream build
cp -Rf .replace.d/* build/
cp -r ./app ./src ./bounce build/
