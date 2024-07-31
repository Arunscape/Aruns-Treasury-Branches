
.DEFAULT_GOAL:= run

clean:
	rm -rf test &
	mvn clean
	wait

package: update
	mvn package

downloadserverjar:
	./get_server_jar.sh

run: package downloadserverjar
	mkdir -p test/plugins/
	mv target/atb-1.0-SNAPSHOT.jar test/plugins/ATB.jar
	cd test && java -jar server.jar -nogui

update:
	#mvn versions:update-parent
	mvn versions:update-properties
	mvn versions:use-latest-releases
	#mvn versions:update-child-modules


ifndef MCAUTH_CLIENT_SECRET
    $(error Error: Environment variable 'MCAUTH_CLIENT_SECRET' is not set)
endif

# Continue with the rest of your Makefile code
