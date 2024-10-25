package gg.arun.atb

import io.jsonwebtoken.Jwts
import net.minecraft.text.Text
import net.silkmc.silk.commands.command
import java.security.KeyFactory
import java.security.spec.PKCS8EncodedKeySpec
import java.util.Date
import kotlin.io.encoding.Base64
import kotlin.io.encoding.ExperimentalEncodingApi

fun init() {
    println("hello arun")

    command("mycommand") {
        runs {
            // inside the command handler
            source.sendMessage(Text.literal("why hello there"))
            source.sendMessage(Text.literal(signmessage(mapOf())))
        }
    }
}

// don't use in prod, this key is just for testing
val base64_key =
    "LS0tLS1CRUdJTiBQUklWQVRFIEtFWS0tLS0tCk1DNENBUUF3QlFZREsyVndCQ0lFSU1pMVdweGFqSHNZNEFnVTAvZVZsYzMwclNqOVBVckE1UXVWYkp0SWJVVzYKLS0tLS1FTkQgUFJJVkFURSBLRVktLS0tLQo="

@OptIn(ExperimentalEncodingApi::class)
fun signmessage(claims: Map<String, Any>): String {

    val pem_private_key = Base64.decode(base64_key).decodeToString()


    val stripped_private_key = pem_private_key.replace("\n", "")
        .replace("-----BEGIN PRIVATE KEY-----", "")
        .replace("-----END PRIVATE KEY-----", "")

    val private_key_bytes = Base64.decode(stripped_private_key)
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