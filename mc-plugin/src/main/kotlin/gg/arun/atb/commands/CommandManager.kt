package gg.arun.atb.commands

import gg.arun.atb.commands.subcommands.*
import org.bukkit.Bukkit
import org.bukkit.command.Command
import org.bukkit.command.CommandExecutor
import org.bukkit.command.CommandSender
import org.bukkit.command.TabCompleter
import org.bukkit.entity.Player

class CommandManager : CommandExecutor, TabCompleter {
    var subcommands: HashMap<String, ArunSubCommand> = HashMap()

    init {
        subcommands["deposit"] = DepositCommand()
        subcommands["withdraw"] = WithdrawCommand()
        subcommands["login"] = LoginCommand()
        subcommands["email"] = EmailCommand()
    }

    fun helpMessage(): String {
        return "Available Commands:\n" + subcommands.map { it.value.getSyntax() + "\n" }.reduce { acc, it -> acc + it }
    }

    override fun onCommand(sender: CommandSender, command: Command, label: String, args: Array<out String>): Boolean {
        if (sender !is Player) {
            sender.sendMessage("Error: This command is meant to be run as a player.")
            return false
        }

        if (args.isEmpty()) {
            sender.sendMessage(helpMessage())
            return false
        }

        val subcommand = args[0]
        if (subcommand !in subcommands) {

            sender.sendMessage("Error: command not found\n" + helpMessage())
            return false
        }

        subcommands[args[0]]?.perform(sender, args)



        return true
    }

    override fun onTabComplete(
        sender: CommandSender, command: Command, label: String, args: Array<out String>?
    ): MutableList<String>? {
        if (args == null) {
            return null
        }
        Bukkit.getLogger().info(args.joinToString())
        if (args.size == 1) {
            return subcommands.keys.toMutableList()
        }

        if (args.size < 2) {
            return null
        }

        val subcommand = args[1]

        if (!subcommands.containsKey(subcommand)) {
            return null
        }



        return subcommands[subcommand]?.onTabComplete(sender, command, label, args)
    }
}