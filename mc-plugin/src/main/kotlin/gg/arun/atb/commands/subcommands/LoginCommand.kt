package gg.arun.atb.commands.subcommands

import gg.arun.atb.commands.SubCommand
import org.bukkit.Bukkit
import org.bukkit.command.Command
import org.bukkit.command.CommandSender
import org.bukkit.entity.Player
import java.util.*

//import khttp.post


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

        player.sendMessage("your id is $uuid ")

        val token = getToken(uuid)

        player.sendMessage("your token is $token")

    }

    fun getToken(uuid: UUID): String {

        return ""
//        val url = "http://localhost:8081/login"
//        val body = mapOf("uuid" to uuid.toString())
//        val res = post(url, body)
//
//        return res.text
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
