package gg.arun.atb

import gg.arun.atb.commands.CommandManager
import org.bukkit.configuration.file.FileConfiguration
import org.bukkit.plugin.java.JavaPlugin

class Atb : JavaPlugin() {
    override fun onEnable() {
        // Plugin startup logic

        config.options().copyDefaults(true)
        // Atb.config = config.getValues(true).map { (k, v) -> k!! to v.toString() }.toMap()
        //     .toMutableMap() as HashMap<String, String>

        Atb.config = config

        // todo: don't log the secrets lol
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