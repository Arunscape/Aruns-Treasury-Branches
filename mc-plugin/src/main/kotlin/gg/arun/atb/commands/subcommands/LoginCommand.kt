package gg.arun.atb.commands.subcommands


import co.aikar.timings.TimingsManager.url
import gg.arun.atb.Atb
import gg.arun.atb.commands.SubCommand
import gg.arun.atb.server.apiclient.getToken
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.engine.cio.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import net.md_5.bungee.api.chat.ClickEvent
import net.md_5.bungee.api.chat.ComponentBuilder
import net.md_5.bungee.api.chat.HoverEvent
import org.bukkit.command.Command
import org.bukkit.command.CommandSender
import org.bukkit.entity.Player
import java.lang.Exception
import java.util.*


val websiteUrl = Atb.config.getString("website_url")!!

class LoginCommand : SubCommand() {
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
        val username = player.name

        println("$username is attempting to login. Their uuid is $uuid ")

        try {
            val token = getToken(uuid)
            val loginUrl =
                URLBuilder(websiteUrl).run {
                    path("login")
                    parameters.append("token", token)
                    parameters.append("username", username)
                    buildString()
                }
            val textComponent = net.md_5.bungee.api.chat.TextComponent(loginUrl)
            textComponent.setClickEvent(ClickEvent(ClickEvent.Action.OPEN_URL, loginUrl))
            textComponent.setHoverEvent(
                HoverEvent(
                    HoverEvent.Action.SHOW_TEXT,
                    ComponentBuilder("Click me to login!").create()
                )
            )
            player.sendMessage(textComponent)
        } catch (e: Exception) {
            println("Error: $e")
            player.sendMessage("login error: ask Arun: $e")
        }


    }

    override fun onTabComplete(
        sender: CommandSender, command: Command, label: String, args: Array<out String>
    ): MutableList<String> {
        return mutableListOf()
    }


}
