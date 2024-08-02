//val ktor_version: String by project
//val kotlin_version: String by project

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


idea {
    module {

        isDownloadJavadoc = true
        isDownloadSources = true
    }
}
//allprojects {

dependencies {

    implementation("io.ktor:ktor-client-core")
    implementation("io.ktor:ktor-client-cio")
    implementation("io.ktor:ktor-client-content-negotiation")
    implementation("io.ktor:ktor-serialization-kotlinx-json")

    implementation(libs.com.github.shynixn.mccoroutine.mccoroutine.bukkit.api)
    implementation(libs.com.github.shynixn.mccoroutine.mccoroutine.bukkit.core)
    
    implementation("io.jsonwebtoken:jjwt-api")
    runtimeOnly("io.jsonwebtoken:jjwt-impl")
    runtimeOnly("io.jsonwebtoken:jjwt-jackson")

    compileOnly("org.purpurmc.purpur", "purpur-api", "+")

}

//}

