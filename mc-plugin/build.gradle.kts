import org.jetbrains.kotlin.gradle.dsl.JvmTarget
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    kotlin("jvm") version "2.0.21"
    id("fabric-loom") version "1.8-SNAPSHOT"
    id("maven-publish")
    kotlin("plugin.serialization") version "2.0.21"
}

repositories {
    mavenCentral()
}


version = project.property("mod_version") as String
group = project.property("maven_group") as String


val silk_version = "1.10.7"
dependencies {
    testImplementation(kotlin("test"))
    modImplementation("net.silkmc:silk-commands:$silk_version")
    modImplementation("net.silkmc:silk-core:$silk_version")
    modImplementation("net.silkmc:silk-game:$silk_version")
    modImplementation("net.silkmc:silk-igui:$silk_version")
    modImplementation("net.silkmc:silk-nbt:$silk_version")
    modImplementation("net.silkmc:silk-network:$silk_version")
    modImplementation("net.silkmc:silk-persistence:$silk_version")
//    modImplementation("net.silkmc:silk-compose:+")


    // To change the versions see the gradle.properties file
    minecraft("com.mojang:minecraft:${project.property("minecraft_version")}")
    mappings("net.fabricmc:yarn:${project.property("yarn_mappings")}:v2")
    modImplementation("net.fabricmc:fabric-loader:${project.property("loader_version")}")
    modImplementation("net.fabricmc:fabric-language-kotlin:${project.property("kotlin_loader_version")}")
    modImplementation("net.fabricmc.fabric-api:fabric-api:${project.property("fabric_version")}")


    val jjwt_version = "latest.release"
    implementation("io.jsonwebtoken:jjwt-api:$jjwt_version")
    runtimeOnly("io.jsonwebtoken:jjwt-impl:$jjwt_version")
    runtimeOnly("io.jsonwebtoken:jjwt-jackson:$jjwt_version")

    val ktor_version = "latest.release"
    implementation("io.ktor:ktor-client-core:$ktor_version")
    implementation("io.ktor:ktor-client-cio:$ktor_version")
    implementation("io.ktor:ktor-client-content-negotiation:$ktor_version")
    implementation("io.ktor:ktor-serialization-kotlinx-json:$ktor_version")

}

base {
    archivesName.set(project.property("archives_base_name") as String)
}

val targetJavaVersion = 21
java {
    toolchain.languageVersion = JavaLanguageVersion.of(targetJavaVersion)
    // Loom will automatically attach sourcesJar to a RemapSourcesJar task and to the "build" task
    // if it is present.
    // If you remove this line, sources will not be generated.
    withSourcesJar()
}

loom {
    splitEnvironmentSourceSets()

    mods {
        register("atb-mc-plugin") {
            sourceSet("main")
//            sourceSet("client")
        }
    }
}

repositories {
    // Add repositories to retrieve artifacts from in here.
    // You should only use this when depending on other mods because
    // Loom adds the essential maven repositories to download Minecraft and libraries from automatically.
    // See https://docs.gradle.org/current/userguide/declaring_repositories.html
    // for more information about repositories.
}

//dependencies {
//    // To change the versions see the gradle.properties file
//    minecraft("com.mojang:minecraft:${project.property("minecraft_version")}")
//    mappings("net.fabricmc:yarn:${project.property("yarn_mappings")}:v2")
//    modImplementation("net.fabricmc:fabric-loader:${project.property("loader_version")}")
//    modImplementation("net.fabricmc:fabric-language-kotlin:${project.property("kotlin_loader_version")}")
//
//    modImplementation("net.fabricmc.fabric-api:fabric-api:${project.property("fabric_version")}")
//}

tasks.processResources {
    inputs.property("version", project.version)
    inputs.property("minecraft_version", project.property("minecraft_version"))
    inputs.property("loader_version", project.property("loader_version"))
    filteringCharset = "UTF-8"

    filesMatching("fabric.mod.json") {
        expand(
            "version" to project.version,
            "minecraft_version" to project.property("minecraft_version"),
            "loader_version" to project.property("loader_version"),
            "kotlin_loader_version" to project.property("kotlin_loader_version")
        )
    }
}

tasks.withType<JavaCompile>().configureEach {
    // ensure that the encoding is set to UTF-8, no matter what the system default is
    // this fixes some edge cases with special characters not displaying correctly
    // see http://yodaconditions.net/blog/fix-for-java-file-encoding-problems-with-gradle.html
    // If Javadoc is generated, this must be specified in that task too.
    options.encoding = "UTF-8"
    options.release.set(targetJavaVersion)
}

tasks.withType<KotlinCompile>().configureEach {
    compilerOptions.jvmTarget.set(JvmTarget.fromTarget(targetJavaVersion.toString()))
}

tasks.jar {
    from("LICENSE") {
        rename { "${it}_${project.base.archivesName}" }
    }
}

// configure the maven publication
publishing {
    publications {
        create<MavenPublication>("mavenJava") {
            artifactId = project.property("archives_base_name") as String
            from(components["java"])
        }
    }

    // See https://docs.gradle.org/current/userguide/publishing_maven.html for information on how to set up publishing.
    repositories {
        // Add repositories to publish to here.
        // Notice: This block does NOT have the same function as the block in the top level.
        // The repositories here will be used for publishing your artifact, not for
        // retrieving dependencies.
    }
}
