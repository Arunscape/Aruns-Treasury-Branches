package xyz.woosaree.atb

import org.bukkit.event.EventHandler
import org.bukkit.event.Listener
import org.bukkit.event.player.PlayerJoinEvent
import org.bukkit.event.player.PlayerLoginEvent

class Events : Listener{

    @EventHandler
    fun onPlayerJoin(event: PlayerJoinEvent){
        val player = event.player
        val name = player.name
        val message = "Hello $name, welcome to Arun's Treasury Branches.\nType /atb to get started!"
        player.sendMessage(message)
        print("INFO: sent this message to $name: \"$message\"")
    }
}