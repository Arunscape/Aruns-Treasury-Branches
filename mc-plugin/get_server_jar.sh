#!/usr/bin/env bash

rm -rf test/ &

VERSION=$(curl "https://api.purpurmc.org/v2/purpur" | jq -r .versions[-1])

DOWNLOAD_URL="https://api.purpurmc.org/v2/purpur/$VERSION/latest/download"

wait 

mkdir test

curl -Lo test/server.jar $DOWNLOAD_URL

echo "eula=true" > test/eula.txt
