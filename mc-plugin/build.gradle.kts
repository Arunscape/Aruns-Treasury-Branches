//val ktor_version: String by project
//val kotlin_version: String by project
import gg.arun.atb.DownloadServerJar

val kotlin_version = "2.0.0"
val ktor_version = "latest.release"

plugins {
    idea
    kotlin("jvm") version "2.0.0"
    kotlin("plugin.serialization") version "2.0.0"
    id("com.github.johnrengelman.shadow") version "8.+"
    "kotlin-dsl"

}

repositories {
    mavenCentral()
//    maven {
//        name = "purpurmc"
//        url = uri("https://repo.purpurmc.org/snapshots")
//    }
    maven("https://www.jitpack.io")

}

kotlin {
    jvmToolchain(21)
}

allprojects {


    dependencies {

//        implementation("io.ktor:ktor-client-core:$ktor_version")
//        implementation("io.ktor:ktor-client-cio:$ktor_version")
//        implementation("io.ktor:ktor-client-content-negotiation:$ktor_version")
//        implementation("io.ktor:ktor-serialization-kotlinx-json:$ktor_version")

//    implementation(libs.com.github.shynixn.mccoroutine.mccoroutine.bukkit.api)
//    implementation(libs.com.github.shynixn.mccoroutine.mccoroutine.bukkit.core)
//
//
//    val jjwt_version = "latest.release"
//    implementation("io.jsonwebtoken:jjwt-api:$jjwt_version")
//    runtimeOnly("io.jsonwebtoken:jjwt-impl:$jjwt_version")
//    runtimeOnly("io.jsonwebtoken:jjwt-jackson:$jjwt_version")
//
//    compileOnly("org.purpurmc.purpur", "purpur-api", "+")

    }

}

//val server_dir = "build/mcserver"
val server_dir = "test"
group = "gg.arun"
version = "1.0-SNAPSHOT"
description = "Arun's Treasury Branches"

tasks.register<DownloadServerJar>("downloadServerJar") {
    description = "Downloads purpur server"
    version_txt = file("$server_dir/version.txt")
    server_jar = file("$server_dir/server.jar")
    doNotTrackState("checks purpur api")
//    outputs.upToDateWhen { false }
}