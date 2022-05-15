package xyz.woosaree.atb

import org.bukkit.Bukkit
import org.bukkit.Material
import org.bukkit.entity.Player
import org.bukkit.event.EventHandler
import org.bukkit.event.player.PlayerJoinEvent
import org.bukkit.inventory.ItemStack
import org.bukkit.plugin.java.JavaPlugin
import xyz.woosaree.atb.commands.CommandManager

class Atb : JavaPlugin() {
    override fun onEnable() {
        // Plugin startup logic

        server.pluginManager.registerEvents(Events(), this)
        getCommand("atb")?.setExecutor(CommandManager())
        logger.info("ATB plugin loaded")

    }

    override fun onDisable() {
        // Plugin shutdown logic
        logger.info("Stopping ATB plugin...")
    }

    fun deposit(player: Player, material: Material, amount: Int) {

        val inventory = player.inventory

        val itemstack = ItemStack(material, amount)

        inventory.removeItem()


    }

    // override fun onCommand(sender: org.bukkit.command.CommandSender, command: org.bukkit.command.Command, label: kotlin.String, args: kotlin.Array<out kotlin.String>): kotlin.Boolean {
    //     if (sender !is Player) {
    //         sender.sendMessage("Error: Only players can run this command")
    //         return false
    //     }

    //     val uuid = sender.uniqueId
    //     val name = sender.name


    //     sender.sendMessage("ATB says: Hello, $name, your id is $uuid")
    //     return true
    // }

}