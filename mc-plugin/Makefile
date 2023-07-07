
.DEFAULT_GOAL:= run

clean:
	rm -rf test &
	mvn clean
	wait

package:
	mvn package

downloadserverjar:
	./get_server_jar.sh

run: package downloadserverjar
	mkdir -p test/plugins/
	mv target/atb-1.0-SNAPSHOT.jar test/plugins/ATB.jar
	cd test && java -jar server.jar -nogui



ifndef MCAUTH_CLIENT_SECRET
    $(error Error: Environment variable 'MCAUTH_CLIENT_SECRET' is not set)
endif

# Continue with the rest of your Makefile code
