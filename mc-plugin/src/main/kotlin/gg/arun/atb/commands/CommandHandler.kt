//import gg.arun.atb.server.signmessage
//import kotlinx.serialization.Serializable
//import org.bukkit.entity.Player
//import revxrsal.commands.annotation.Command
//import revxrsal.commands.bukkit.BukkitCommandActor
//import kotlin.concurrent.thread
//
////import revxrsal.commands.annotation.Command
////import revxrsal.commands.annotation.Default
////import revxrsal.commands.annotation.Optional
////import revxrsal.commands.annotation.Range
////
////class PingCommand {
////
////    @Command("ping")
////    fun pingCommand(
////        sender: Audience,
////        @Optional @Default("1") @Range(min = 1.0, max = 10.0) times: Int
////    ) {
////        for (i in 1..times) {
////            sender.sendMessage(
////                Component.text("Pong!")
////                    .color(NamedTextColor.LIGHT_PURPLE)
////                    .decorate(TextDecoration.UNDERLINED)
////            )
////        }
////    }
////
////}
//
//class PingCommand {
//
//
//    @Command("hello")
//    fun sayHello(actor: BukkitCommandActor) {
//        actor.reply("Why, hello there!")
//    }
//
//    @Command("atb login")
//    suspend fun login(actor: BukkitCommandActor) {
//        val uuid = actor.uniqueId
//        val username = actor.name
//
//        @Serializable
//        data class payload(val uuid: String, val username: String)
//
//        val p = payload(uuid.toString(), username)
//
//        thread {
//            val x = signmessage(p.toString())
//            println(x)
//            actor.reply("Your key is $x")
//        }
//
//
//    }
//}
