#!/usr/bin/env bash
#mvn clean package
mvn package
mv target/atb-1.0-SNAPSHOT.jar test/ATB.jar
cd test
java -jar server.jar -nogui

