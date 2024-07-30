package gg.arun.atb.server

//import io.jsonwebtoken.Jwts
//import io.jsonwebtoken.SignatureAlgorithm
//import io.jsonwebtoken.io.Decoders
//import io.jsonwebtoken.security.Keys
import gg.arun.atb.Atb.Companion.config
import io.jsonwebtoken.Jwts
import io.ktor.client.*
import io.ktor.client.engine.cio.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.serialization.kotlinx.json.*
import java.security.KeyFactory
import java.security.spec.PKCS8EncodedKeySpec
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
fun signmessage(claims: Map<String, Any>): String {

    val pem_private_key = String(Base64.getDecoder().decode(base64_key))


    val stripped_private_key = pem_private_key.replace("\n", "")
        .replace("-----BEGIN PRIVATE KEY-----", "")
        .replace("-----END PRIVATE KEY-----", "")

    val private_key_bytes = Base64.getDecoder().decode(stripped_private_key)
    val factory = KeyFactory.getInstance("EdDSA")
    val key = factory.generatePrivate(PKCS8EncodedKeySpec(private_key_bytes))

    val now = Date()
    val expiryDate = Date(now.time + 1000) // 1 minute

    return Jwts.builder()
        .issuer("mc.arun.gg")
        .audience().add("atb.arun.gg").and()
        .expiration(expiryDate)
        .notBefore(now)
        .claims(claims)
        .signWith(key, Jwts.SIG.EdDSA)
        .compact()

}




