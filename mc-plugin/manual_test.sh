#!/usr/bin/env bash
set -e 

mvn clean package
mv target/atb-1.0-SNAPSHOT.jar test/plugins/ATB.jar

cd test
java -jar server.jar -nogui
