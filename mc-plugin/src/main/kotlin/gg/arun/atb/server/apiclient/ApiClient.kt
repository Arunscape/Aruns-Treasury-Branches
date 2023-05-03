package gg.arun.atb.server.apiclient

import gg.arun.atb.Atb
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.engine.cio.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Serializable
import java.util.*

@Serializable
data class LoginPayload(val uuid: String)

val serverUrl = Atb.config.getString("server_url")!!

val client = HttpClient(CIO) {
    install(ContentNegotiation) {
        json()
    }
}


fun getToken(uuid: UUID): String = runBlocking {


    val response = client.get(serverUrl) {
        url {
            appendPathSegments("login")
            parameters.append("uuid", "$uuid")
        }

    }

    println("AAAAAAAAAAAAAAAAAAAAAAAAAA" + response)

    return@runBlocking response.body()
}

class ApiClient {
}