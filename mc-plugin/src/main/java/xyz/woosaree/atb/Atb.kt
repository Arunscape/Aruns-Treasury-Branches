package xyz.woosaree.atb

import org.bukkit.Bukkit
import org.bukkit.entity.Player
import org.bukkit.plugin.java.JavaPlugin

class Atb : JavaPlugin() {
    override fun onEnable() {
        // Plugin startup logic
        print("Hello from atb")
    }

    override fun onDisable() {
        // Plugin shutdown logic
        print("bye from atb")
    }

    override fun onCommand(sender: org.bukkit.command.CommandSender, command: org.bukkit.command.Command, label: kotlin.String, args: kotlin.Array<out kotlin.String>): kotlin.Boolean {
        if (sender !is Player) {
            return false
        }

        val uuid = sender.uniqueId
        val name = sender.name

        sender.sendMessage("ATB says: Hello, $name, your id is $uuid")
        return true
    }
}