package gg.arun.atb

import com.github.shynixn.mccoroutine.bukkit.SuspendingJavaPlugin
import com.github.shynixn.mccoroutine.bukkit.registerSuspendingEvents
import com.github.shynixn.mccoroutine.bukkit.setSuspendingExecutor
import com.github.shynixn.mccoroutine.bukkit.setSuspendingTabCompleter
import gg.arun.atb.commands.CommandManager


import org.bukkit.configuration.file.FileConfiguration

class Atb : SuspendingJavaPlugin() {
    override suspend fun onEnableAsync() {

        // Plugin startup logic

        Atb.config = config.options().copyDefaults(true).configuration()


        // todo: don't log the secrets lol
        logger.info("config: " + config.saveToString())

//
        server.pluginManager.registerSuspendingEvents(Events(), this)
        getCommand("atb")?.setSuspendingExecutor(CommandManager())
        getCommand("atb")?.setSuspendingTabCompleter(CommandManager())
//        sayHello()

//        val handler = BukkitCommandHandler.create(this);
//        handler.supportSuspendFunctions()
//        handler.register(this)
//        handler.register(CommandHandler())
//        handler.setHelpWriter { command, actor ->
//            String.format(
//                "%s %s - %s",
//                command.getPath().toRealString(),
//                command.getUsage(),
//                command.getDescription()
//            )
//        }
        logger.info("ATB plugin loaded")
    }

    override suspend fun onDisableAsync() {
        // Plugin shutdown logic
        logger.info("Stopping ATB plugin...")
    }

    companion object {
        lateinit var config: FileConfiguration
    }


}