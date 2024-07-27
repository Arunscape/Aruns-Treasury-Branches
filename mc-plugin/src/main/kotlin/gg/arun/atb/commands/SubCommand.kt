package gg.arun.atb.commands

import com.github.shynixn.mccoroutine.bukkit.SuspendingTabCompleter
import org.bukkit.command.Command
import org.bukkit.command.CommandSender
import org.bukkit.entity.Player

abstract class SubCommand : SuspendingTabCompleter {
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

    override suspend fun onTabComplete(
        sender: CommandSender,
        command: Command,
        alias: String,
        args: Array<out String>
    ): List<String>? {
        return listOf()
    }
}