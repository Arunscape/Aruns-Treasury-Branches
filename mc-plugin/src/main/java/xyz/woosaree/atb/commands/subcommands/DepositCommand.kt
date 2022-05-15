package xyz.woosaree.atb.commands.subcommands

import org.bukkit.Bukkit
import org.bukkit.ChatColor
import org.bukkit.entity.Player
import xyz.woosaree.atb.commands.SubCommand

class DepositCommand : SubCommand() {
    override fun getName(): String {
        return "deposit"
    }
    override fun getDescription(): String {
        return "Deposit an item into ATB"
    }

    override fun getSyntax(): String {
        return "/atb deposit <item> <amount>"
    }


        override fun perform(player: Player, args: Array<out String>) {
        if (args.size != 3){
            sendError(player)
            return
        }

        val item = args[1]
        val amount = args[2]

        val inventory = Bukkit.createInventory(player, 9, "${ChatColor.GREEN}Deposit: Drag items here to deposit them")
        player.openInventory(inventory)

    }


}