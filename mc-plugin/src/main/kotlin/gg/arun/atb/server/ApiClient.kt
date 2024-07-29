package gg.arun.atb.server

import gg.arun.atb.Atb.Companion.config
import io.jsonwebtoken.Jwts
import io.jsonwebtoken.io.Decoders
import io.jsonwebtoken.security.Keys
import io.ktor.client.*
import io.ktor.client.engine.cio.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.serialization.kotlinx.json.*
import java.security.Key
import java.util.*

//@Serializable
//data class LoginPayload(val uuid: String)

val serverUrl = config["server_url"].toString()
val base64_key = System.getenv("JWT_ED25519_KEY")

val client = HttpClient(CIO) {
    install(ContentNegotiation) {
        json()
    }
}

/// returns JWT
fun signmessage(payload: String): String {
    println("KEY: $base64_key")
    val key: Key = Keys.hmacShaKeyFor(
        Decoders.BASE64.decode(base64_key)
    )
    val now = Date()
    val expiryDate = Date(now.time + 1000) // 1 minute

    return Jwts.builder()
//        .issuer("Arun's Treasury Branches minecraft plugin")
//        .subject("linking minecraft uuid to account on $serverUrl")
//        .audience().add(serverUrl).and()
//        .expiration(expiryDate)
//        .notBefore(now)
        .content(payload)
        .signWith(key)
        .compact()
}




