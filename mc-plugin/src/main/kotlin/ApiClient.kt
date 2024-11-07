import io.ktor.client.*
import io.ktor.client.engine.cio.*
import io.ktor.client.plugins.contentnegotiation.ContentNegotiation
import io.ktor.client.request.*
import io.ktor.client.statement.*
import io.ktor.serialization.kotlinx.json.json
import kotlinx.serialization.json.Json
import net.minecraft.item.Item
import kotlin.uuid.ExperimentalUuidApi
import kotlin.uuid.Uuid

val client = HttpClient(CIO) {
    install(ContentNegotiation) {
        json(Json {
            prettyPrint = true
            isLenient = true
        })
    }
}


@OptIn(ExperimentalUuidApi::class)
fun deposit(account: Uuid, item: Item, amount: UInt){

}
