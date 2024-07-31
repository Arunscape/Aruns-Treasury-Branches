//val ktor_version: String by project
//val kotlin_version: String by project\

val kotlin_version = "2.0.0"
val ktor_version = "latest.release"

plugins {
    idea
    kotlin("jvm") version "2.0.0"
    id("com.github.johnrengelman.shadow") version "8.+"
//    "maven-publish"
}

repositories {
    mavenCentral()
    maven {
        name = "purpurmc"
        url = uri("https://repo.purpurmc.org/snapshots")
    }
    maven("https://www.jitpack.io")

}

dependencies {

    implementation("io.ktor:ktor-client-core:$ktor_version")
    implementation("io.ktor:ktor-client-cio:$ktor_version")
    implementation("io.ktor:ktor-client-content-negotiation:$ktor_version")
    implementation("io.ktor:ktor-serialization-kotlinx-json:$ktor_version")

    implementation(libs.com.github.shynixn.mccoroutine.mccoroutine.bukkit.api)
    implementation(libs.com.github.shynixn.mccoroutine.mccoroutine.bukkit.core)


    val jjwt_version = "latest.release"
    implementation("io.jsonwebtoken:jjwt-api:$jjwt_version")
    runtimeOnly("io.jsonwebtoken:jjwt-impl:$jjwt_version")
    runtimeOnly("io.jsonwebtoken:jjwt-jackson:$jjwt_version")

    compileOnly("org.purpurmc.purpur", "purpur-api", "+")

}
//
tasks.shadowJar {
    // wait for this to merge: https://github.com/GradleUp/shadow/pull/876
//    minimize {
//
//    }
}
//
//idea {
//
//    module {
//
//        isDownloadJavadoc = true
//        isDownloadSources = true
//    }
//
//}
//
tasks.build {
//    dependsOn("shadowJar")
}

group = "gg.arun"
version = "1.0-SNAPSHOT"
description = "Arun's Treasury Branches"

tasks.processResources {
    val props = mapOf("version" to version)
    inputs.properties(props)
    filteringCharset = "UTF-8"
    filesMatching("plugin.yml") {
        expand(props)
    }
}

//publishing {
//    publications {
//        create<MavenPublication>("shadow") {
//            project.extensions.configure<com.github.jengelman.gradle.plugins.shadow.ShadowExtension>() {
//                component(this@create)
//            }=
//        }
//    }
//}

val mcserver_dir = "test"


//tasks.register<DownloadServerJar>("downloadServerJar") {
//    description = "Downloads purpur server"
////    outputs.upToDateWhen { false }
//}