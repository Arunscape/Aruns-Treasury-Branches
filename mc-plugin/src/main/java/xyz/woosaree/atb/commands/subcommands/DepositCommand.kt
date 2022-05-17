package xyz.woosaree.atb.commands.subcommands

import org.bukkit.Bukkit
import org.bukkit.ChatColor
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
    }

    fun sendErr(player: Player, message: String) {
        player.sendMessage("Error: $message\nUsage: ${getSyntax()}")
    }

    override fun perform(player: Player, args: Array<out String>) {
        if (args.size != 3) {
            sendError(player)
            return
        }

        val item = Material.matchMaterial(args[1])

        item ?: run {
            sendErr(player, "Item ${args[1]} does not exist")
            return
        }

        val amount = args[2].toIntOrNull()

        amount ?: run {
            sendErr(player, "The amount you entered was not a number")
            return
        }

//        val items = player.inventory.filterNotNull().filter { it.type == item }\
//        player.sendMessage("found these items: $items")


        val remove_these = ItemStack(item, amount)

        val cant_remove = player.inventory.removeItem(remove_these)

        if (cant_remove.isNotEmpty()) {
            player.sendMessage("failed to remove these items: $cant_remove")
            return
        }


        player.sendMessage("Removed items. Contacting backend...")


//        val inventory = Bukkit.createInventory(player, 9, "${ChatColor.GREEN}Deposit: Drag items here to deposit them")
//        player.openInventory(inventory)

    }


}