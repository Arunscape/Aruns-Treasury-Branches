package gg.arun.atb.commands

import org.bukkit.command.TabCompleter
import org.bukkit.configuration.file.YamlConfiguration
import org.bukkit.entity.Player

abstract class SubCommand : TabCompleter {
    abstract fun getName(): String
    abstract fun getDescription(): String
    abstract fun getSyntax(): String
    abstract fun perform(player: Player, args: Array<out String>)
    fun sendError(player: Player) {
        player.sendMessage("Error. Usage: ${getSyntax()}")
    }

    fun sendError(player: Player, message: String) {
        player.sendMessage("Error: $message\nUsage: ${getSyntax()}")
    }

}