package xyz.woosaree.atb.commands.subcommands

import org.bukkit.Material
import org.bukkit.entity.Player
import org.bukkit.inventory.ItemStack
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

//        val items = player.inventory.filterNotNull().filter { it.type == item }\
//        player.sendMessage("found these items: $items")


        val removeThese = ItemStack(item, amount)
        val cantRemove = player.inventory.removeItem(removeThese)

        if (cantRemove.isNotEmpty()) {
            player.sendMessage("failed to remove these items: $cantRemove")
            return
        }


        player.sendMessage("Removed items. Contacting backend...")


//        val inventory = Bukkit.createInventory(player, 9, "${ChatColor.GREEN}Deposit: Drag items here to deposit them")
//        player.openInventory(inventory)

    }


}
