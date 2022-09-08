package gg.arun.atb.commands.subcommands

import org.bukkit.Material
import org.bukkit.entity.Player
import org.bukkit.inventory.ItemStack
import gg.arun.atb.commands.SubCommand

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


}