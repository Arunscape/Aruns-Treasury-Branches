package gg.arun.atb

import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.engine.cio.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.request.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Serializable
import org.gradle.api.DefaultTask
import org.gradle.api.tasks.OutputFile
import org.gradle.api.tasks.TaskAction
import org.gradle.api.tasks.UntrackedTask
import java.io.File


val client = HttpClient(CIO) {
    install(ContentNegotiation) {
        json()
    }
}

suspend fun downloadServerJar(f: File, version: String) {
    println("downlading server jar...")
    val response = client.get("https://api.purpurmc.org/v2/purpur/$version/latest/download") {
//                onDownload { bytesSentTotal, contentLength ->
//                    println("Received $bytesSentTotal bytes / $contentLength")
//                }
    }
    val body: ByteArray = response.body()
    f.writeBytes(body)
}

suspend fun getLatestVersion(): String {
    val versions_response = client.get("https://api.purpurmc.org/v2/purpur")

    val versions: PurpurData = versions_response.body()

    println("VERSIONS: $versions")

    val latest_version = versions.versions.last()

    return latest_version
}

@Serializable
data class PurpurData(val project: String, val versions: List<String>)

@UntrackedTask(because = "")
open class DownloadServerJar : DefaultTask() {


//    @InputFile
//    lateinit var version_txt: Optional<File>

//    @OutputFiles
//    lateinit var outputFiles: OutputFiles

    @OutputFile
    lateinit var server_jar: File

    @OutputFile
    lateinit var version_txt: File

    @OutputFile
    lateinit var eula_txt: File


    @TaskAction
    fun download() {


        if (!version_txt.exists()) {
            version_txt.createNewFile()
        }

        val version = version_txt.readText()


        val latest_version: String = runBlocking {

            getLatestVersion()

        }

        println("MINECRAFT SERVER VERSION: $latest_version")
        println("contents of version.txt: $version  ")

        if (version.equals(latest_version)) {
            println("latest version already installed")
            return
        }


        runBlocking {
            downloadServerJar(server_jar, latest_version)
        }

        version_txt.writeText(latest_version)
        eula_txt.writeText("eula=true")
    }
}