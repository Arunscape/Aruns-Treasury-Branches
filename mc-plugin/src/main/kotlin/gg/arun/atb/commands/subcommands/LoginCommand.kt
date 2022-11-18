package gg.arun.atb.commands.subcommands


import gg.arun.atb.Atb
import gg.arun.atb.commands.SubCommand
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.engine.cio.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import org.bukkit.command.Command
import org.bukkit.command.CommandSender
import org.bukkit.entity.Player
import java.util.*

@Serializable
data class LoginPayload(val uuid: String)

class LoginCommand : SubCommand() {

    val client = HttpClient(CIO) {
        install(ContentNegotiation) {
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

        try {
            val token = getToken(uuid)

            player.sendMessage("your token is $token")
        } catch (_: Error) {
            player.sendMessage("login error: ask Arun")
        }


    }

    fun getToken(uuid: UUID): String = runBlocking {

        val serverUrl = Atb.config.getString("server_url")

        val requestUrl = "$serverUrl/login?uuid=$uuid"
        println(requestUrl)
        val response = client.get(requestUrl)
        return@runBlocking response.body()
    }

    override fun onTabComplete(
        sender: CommandSender, command: Command, label: String, args: Array<out String>
    ): MutableList<String> {
        return mutableListOf()
    }


}
