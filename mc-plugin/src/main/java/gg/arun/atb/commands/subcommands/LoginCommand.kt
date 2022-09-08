package gg.arun.atb.commands.subcommands

import org.bukkit.Material
import org.bukkit.entity.Player
import org.bukkit.inventory.ItemStack
import gg.arun.atb.commands.SubCommand

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

        player.sendMessage("your id is " + uuid)

    }


}
