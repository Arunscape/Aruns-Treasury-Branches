package gg.arun.atb

import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.engine.cio.*
import io.ktor.client.plugins.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.request.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.utils.io.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.*
import org.gradle.api.DefaultTask
import org.gradle.api.tasks.InputFile
import org.gradle.api.tasks.OutputFile
import org.gradle.api.tasks.TaskAction
import java.io.File
import java.util.*
import kotlin.jvm.optionals.getOrDefault


val client = HttpClient(CIO) {
    install(ContentNegotiation) {
        json()
    }
}

@Serializable
data class PurpurData(val project: String, val versions: List<String>)

open class DownloadServerJar : DefaultTask() {


//    @InputFile
//    lateinit var version_txt: Optional<File>

//    @OutputFiles
//    lateinit var outputFiles: OutputFiles

    @OutputFile
    lateinit var server_jar: File

    @OutputFile
    lateinit var version_txt: File


    @TaskAction
    fun download() {

        val version = {
            if (!version_txt.exists()) {
                version_txt.createNewFile()
            }
            version_txt.readText()
        }

        val latest_version: String = runBlocking {

            val versions_response = client.get("https://api.purpurmc.org/v2/purpur")

            val versions: PurpurData = versions_response.body()

            println("VERSIONS: $versions")

            val latest_version = versions.versions.last()

            latest_version

        }

        println("MINECRAFT SERVER VERSION: $latest_version")

        if (version.equals(latest_version)) {
            return
        }

        runBlocking {
            val response = client.get("https://api.purpurmc.org/v2/purpur/$latest_version/latest/download") {
//                onDownload { bytesSentTotal, contentLength ->
//                    println("Received $bytesSentTotal bytes / $contentLength")
//                }
            }
            val body: ByteArray = response.body()
            server_jar.writeBytes(body)
        }

        version_txt.writeText(latest_version)
    }
}