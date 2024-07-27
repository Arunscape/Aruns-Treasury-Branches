package gg.arun.atb.commands.subcommands


import gg.arun.atb.Atb.Companion.config
import gg.arun.atb.commands.SubCommand
import gg.arun.atb.server.signmessage
import kotlinx.serialization.Serializable
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

    override fun perform(player: Player, args: Array<out String>) {
        val uuid = player.uniqueId
        val username = player.name

        @Serializable
        data class Payload(val uuid: String, val username: String)

        val p = Payload(uuid.toString(), username)

        val x = signmessage(p.toString())

        println(x)
        player.sendMessage(x)
    }


}
