package gg.arun.atb.commands

import com.github.shynixn.mccoroutine.bukkit.SuspendingCommandExecutor
import com.github.shynixn.mccoroutine.bukkit.SuspendingTabCompleter
import gg.arun.atb.commands.subcommands.DepositCommand
import gg.arun.atb.commands.subcommands.EmailCommand
import gg.arun.atb.commands.subcommands.LoginCommand
import gg.arun.atb.commands.subcommands.WithdrawCommand
import org.bukkit.Bukkit
import org.bukkit.command.Command
import org.bukkit.command.CommandSender
import org.bukkit.entity.Player

class CommandManager : SuspendingCommandExecutor, SuspendingTabCompleter {
    private var subcommands: HashMap<String, SubCommand> = HashMap()

    init {
        subcommands["deposit"] = DepositCommand()
        subcommands["withdraw"] = WithdrawCommand()
        subcommands["login"] = LoginCommand()
        subcommands["email"] = EmailCommand()
    }

    private fun helpMessage(): String {

        return "Available Commands:\n" + subcommands.map { it.value.getSyntax() + "\n" }.reduce { acc, it -> acc + it }
    }

    override suspend fun onCommand(
        sender: CommandSender,
        command: Command,
        label: String,
        args: Array<out String>
    ): Boolean {
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

    override suspend fun onTabComplete(
        sender: CommandSender, command: Command, alias: String, args: Array<out String>
    ): List<String>? {
        Bukkit.getLogger().info(args.joinToString())
        print("trying to tab complete $command - $alias - $args")
        if (args.size == 1) {
            return subcommands.keys.toList()
        }

        if (args.size < 2) {
            return null
        }

        val subcommand = args[0]

        if (!subcommands.containsKey(subcommand)) {
            return null
        }

        return subcommands[subcommand]?.onTabComplete(sender, command, alias, args.sliceArray(1 until args.size))

    }
}