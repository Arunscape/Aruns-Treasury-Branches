package gg.arun.atb.commands.subcommands

import org.bukkit.entity.Player
import gg.arun.atb.commands.SubCommand
import org.bukkit.command.Command
import org.bukkit.command.CommandSender

class LoginCommand : ArunSubCommand() {

    val login_secret: String = System.getenv("ATB_PLUGIN_SECRET") ?: "changeme"
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

        player.sendMessage("your id is " + uuid + " " + login_secret)


    }

    override fun onTabComplete(
        sender: CommandSender,
        command: Command,
        label: String,
        args: Array<out String>?
    ): MutableList<String>? {
        TODO("Not yet implemented")
    }


}
