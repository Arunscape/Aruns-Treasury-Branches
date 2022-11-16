package gg.arun.atb.commands.subcommands

import gg.arun.atb.commands.SubCommand
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.engine.cio.*
import org.bukkit.Bukkit
import org.bukkit.command.Command
import org.bukkit.command.CommandSender
import org.bukkit.entity.Player
import io.ktor.client.request.*
import io.ktor.client.statement.*
import kotlinx.coroutines.runBlocking
import java.lang.Error
import java.util.*
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.engine.cio.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.request.*
import io.ktor.client.statement.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.*
import kotlinx.serialization.*
import kotlinx.serialization.json.*

@Serializable
data class LoginPayload(val uuid: String)

class LoginCommand : SubCommand() {

    val client = HttpClient(CIO) {
        install(ContentNegotiation) {
//            gson()
            json()
        }
    }

    override fun getName(): String {
        return "login"
    }

    override fun getDescription(): String {
        return "Generates a URL you can paste into your browser to login on atb.arun.gg"
    }

    override fun getSyntax(): String {
        return "/atb login"
    }


    override fun perform(player: Player, args: Array<out String>) {

        val uuid = player.uniqueId

        player.sendMessage("your id is $uuid ")

        val token = getToken(uuid)

        player.sendMessage("your token is $token")

    }

    fun getToken(uuid: UUID): String = runBlocking {

//        old java way
//        val values = mapOf("uuid" to uuid.toString())
//
//
//        val client = HttpClient.newBuilder().build();
//        val request = HttpRequest.newBuilder()
//            .uri(URI.create("http://localhost:8081/login"))
//            .POST(HttpRequest.BodyPublishers.ofString("\"uuid\": \"$uuid\""))
//            .build()
//        val response = client.send(request, HttpResponse.BodyHandlers.ofString());
//        println(response.body())
//

        // khttp
//        return response.body()
//        val url = "http://localhost:8081/login"
//        val body = mapOf("uuid" to uuid.toString())
//        val res = post(url, body)
//
//        return res.text

        try {
            val response = client.post("http://[::1]:8081/login") {
                val body = LoginPayload(uuid.toString())
                println(Json.encodeToString(body))
                setBody(body)
                contentType(ContentType.Application.Json)
            }
            return@runBlocking response.body()

        } catch (e: Error) {
            return@runBlocking e.toString()
        }


    }

    override fun onTabComplete(
        sender: CommandSender,
        command: Command,
        label: String,
        args: Array<out String>
    ): MutableList<String> {
        return mutableListOf()
    }


}
