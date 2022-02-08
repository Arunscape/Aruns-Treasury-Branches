package xyz.woosaree.arunstreasurybranches;

import org.bukkit.command.Command;
import org.bukkit.command.CommandSender;
import org.bukkit.plugin.java.JavaPlugin;

public final class ATB extends JavaPlugin {

    @Override
    public void onEnable() {
        // Plugin startup logic
        System.out.println("ATB started");

    }

    @Override
    public void onDisable() {
        // Plugin shutdown logic
    }

    @Override
    public boolean onCommand(CommandSender sender, Command cmd, String label, String[] args) {

        sender.sendMessage("Welcome to Arun's Treasury Branches!");
        return true;
    }
}
