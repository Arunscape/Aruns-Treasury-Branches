package gg.arun.atb

import net.minecraft.text.Text
import net.silkmc.silk.commands.command
import net.silkmc.silk.core.annotations.ExperimentalSilkApi

@OptIn(ExperimentalSilkApi::class)
fun init() {
    println("hello arun")

    command("mycommand") {
        runs {
            // inside the command handler
            source.sendMessage(Text.literal("why hello there"))
        }
    }
}