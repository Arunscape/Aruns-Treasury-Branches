#!/usr/bin/env bash

# rm -rf test/ &

VERSION=$(<test/version.txt) 2> /dev/null

LATEST_VERSION=$(curl "https://api.purpurmc.org/v2/purpur" | jq -r .versions[-1])


if [[ $VERSION == $LATEST_VERSION ]]; then
    echo "Already got the latest version"
    exit 0
fi

DOWNLOAD_URL="https://api.purpurmc.org/v2/purpur/$LATEST_VERSION/latest/download"

wait 

mkdir test

curl -Lo test/server.jar $DOWNLOAD_URL

echo "eula=true" > test/eula.txt

echo $LATEST_VERSION > test/version.txt
