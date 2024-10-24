package gg.arun.atb

import net.fabricmc.api.ModInitializer

class ATB : ModInitializer {
    override fun onInitialize() {
        println("Hello from fabric arun")

        command("mycommand") {

        }

    }
}

