package gg.arun.atb

import org.bukkit.Bukkit
import org.bukkit.Material
import org.bukkit.entity.Player
import org.bukkit.event.EventHandler
import org.bukkit.event.player.PlayerJoinEvent
import org.bukkit.inventory.ItemStack
import org.bukkit.plugin.java.JavaPlugin
import gg.arun.atb.commands.CommandManager

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


}