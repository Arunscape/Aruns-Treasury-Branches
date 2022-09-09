package gg.arun.atb.commands.subcommands

import gg.arun.atb.commands.SubCommand
import org.bukkit.Material
import org.bukkit.command.Command
import org.bukkit.command.CommandSender
import org.bukkit.entity.Player
import org.bukkit.inventory.ItemStack

class WithdrawCommand : SubCommand() {
    override fun getName(): String {
        return "withdraw"
    }

    override fun getDescription(): String {
        return "Withdraw an item from ATB"
    }

    override fun getSyntax(): String {
        return "/atb withdraw <item> <amount>"
        // return "/atb withdraw <item> <amount> <account_nickname>"
    }


    override fun perform(player: Player, args: Array<out String>) {
        if (args.size != 3) {
            sendError(player)
            return
        }

        val item = Material.matchMaterial(args[1])
        if (item == null) {
            sendError(player, "Item ${args[1]} does not exist")
            return
        }

        val amount = args[2].toIntOrNull()
        if (amount == null) {
            sendError(player, "The amount you entered was not a number")
            return
        }


        player.sendMessage("Contacting backend...")

        val addThese = ItemStack(item, amount)
        val failed = player.inventory.addItem(addThese)
        if (failed.isNotEmpty()) {
            player.sendMessage("Error: failed to withdraw these: $failed")
            val failedAmount = failed.values.map { it.amount }.reduce { acc, it -> acc + it }

            player.sendMessage("Contacting backend...")

            player.sendMessage("$failedAmount/$amount items were not deposssited and returned to your online account")
        }


        player.sendMessage("Success")

//        val inventory = Bukkit.createInventory(player, 9, "${ChatColor.GREEN}Deposit: Drag items here to deposit them")
//        player.openInventory(inventory)

    }

    override fun onTabComplete(
        sender: CommandSender, command: Command, label: String, args: Array<out String>?
    ): MutableList<String>? {

        if (args?.size == 1) {
            return Material.values().map { it.toString().lowercase() }.filter { it.contains(args[0].lowercase()) }
                .toMutableList()
        }

        if (args?.size == 2) {
            return mutableListOf("2", "4", "8", "16", "32", "64")
        }

        return mutableListOf()
    }

}