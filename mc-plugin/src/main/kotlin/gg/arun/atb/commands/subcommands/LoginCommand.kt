package gg.arun.atb.commands.subcommands


import gg.arun.atb.Atb.Companion.config
import gg.arun.atb.commands.SubCommand
import gg.arun.atb.server.client
import gg.arun.atb.server.serverUrl
import gg.arun.atb.server.signmessage
import io.ktor.client.request.*
import io.ktor.http.*
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import org.bukkit.entity.Player


class LoginCommand : SubCommand() {

    private val websiteUrl: String = config["website_url"].toString()
    override fun getName(): String {
        return "login"
    }

    override fun getDescription(): String {
        return "Generates a URL you can paste into your browser to login on atb.arun.gg"
    }

    override fun getSyntax(): String {
        return "/atb login"
    }

//    override fun perform(player: Player, args: Array<out String>) {
//
//        val uuid = player.uniqueId
//        val username = player.name
//
//        println("$username is attempting to login. Their uuid is $uuid ")
//
//        try {
//            val token = getToken(uuid)
//            val loginUrl =
//                URLBuilder(websiteUrl).run {
//                    path("login")
//                    parameters.append("token", token)
//                    parameters.append("username", username)
//                    buildString()
//                }
//
//            val component = Component.text(loginUrl)
//                .clickEvent(ClickEvent.openUrl(loginUrl))
//                .hoverEvent(HoverEvent.showText(Component.text("Click me to login!")))
//
//            player.sendMessage(component)
//        } catch (e: Exception) {
//            println("Error: $e")
//            player.sendMessage("login error: ask Arun: $e")
//        }
//
//
//    }

    override suspend fun perform(player: Player, args: Array<out String>) {
        val uuid = player.uniqueId
        val username = player.name

        @Serializable
        data class Payload(val uuid: String, val username: String)

        val p = Payload(uuid.toString(), username)

        val x = signmessage(Json.encodeToString(p))

        println(x)
        println(serverUrl)

        client.post(serverUrl) {
            url {
                appendPathSegments("mcplugin", "link_account")
            }
            headers {
                append(HttpHeaders.Authorization, x)
            }
        }
        player.sendMessage(x)
    }


}
