package gg.arun.atb.commands.subcommands

import org.bukkit.Material
import org.bukkit.entity.Player
import org.bukkit.inventory.ItemStack
import gg.arun.atb.commands.SubCommand
import org.bukkit.command.Command
import org.bukkit.command.CommandSender

class EmailCommand : ArunSubCommand() {
    override fun getName(): String {
        return "email"
    }

    override fun getDescription(): String {
        return "Associates an email address with your account." +
                "This has no benefit at the moment." +
                "In the future, I might add social logins"
    }

    override fun getSyntax(): String {
        return "/atb email <your_email_address>"
    }


    override fun perform(player: Player, args: Array<out String>) {

        // todo
        player.sendMessage(getDescription())

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
