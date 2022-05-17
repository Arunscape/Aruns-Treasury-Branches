package xyz.woosaree.atb.commands

import org.bukkit.command.Command
import org.bukkit.command.CommandExecutor
import org.bukkit.command.CommandSender
import org.bukkit.entity.Player
import xyz.woosaree.atb.commands.subcommands.DepositCommand
import xyz.woosaree.atb.commands.subcommands.WithdrawCommand

class CommandManager : CommandExecutor {
    var subcommands: HashMap<String, SubCommand> = HashMap()


    init {
        subcommands["deposit"] = DepositCommand()
        subcommands["withdraw"] = WithdrawCommand()
    }

    override fun onCommand(sender: CommandSender, command: Command, label: String, args: Array<out String>): Boolean {
        if (sender !is Player) {
            sender.sendMessage("Error: This command is meant to be run as a player.")
            return false
        }

        if (args.isEmpty()) {
            sender.sendMessage("idk show help message or something")
            return false
        }

        var msg = ""
        args.forEach {
            msg += "\n$it"
        }

        val subcommand = args[0]
        if (subcommand !in subcommands) {
            val m = "Error: subcommand not found. Available commands are:\n" +
                    subcommands.map { it.value.getSyntax() + "\n" }.reduce { acc, it -> acc + it }

            sender.sendMessage(m)
            return false
        }

        subcommands[args[0]]?.perform(sender, args)



        return true
    }
}