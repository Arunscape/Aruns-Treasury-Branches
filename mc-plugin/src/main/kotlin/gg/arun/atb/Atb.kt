package gg.arun.atb

import org.bukkit.plugin.java.JavaPlugin
import gg.arun.atb.commands.CommandManager
import org.bukkit.configuration.MemoryConfiguration
import org.bukkit.configuration.file.FileConfiguration
import java.io.File

class Atb : JavaPlugin() {

    override fun onEnable() {
        // Plugin startup logic

        config.addDefault("server_url", "http://[::]:8081")
        config.addDefault("website_url", "https://atb.arun.gg")
        config.options().copyDefaults(true)
        saveConfig()

        Atb.config = config

        logger.info("config: " + config.saveToString())



        server.pluginManager.registerEvents(Events(), this)
        getCommand("atb")?.setExecutor(CommandManager())
        logger.info("ATB plugin loaded")
    }

    override fun onDisable() {
        // Plugin shutdown logic
        logger.info("Stopping ATB plugin...")
    }

    companion object {
        lateinit var config: FileConfiguration
    }


}