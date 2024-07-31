plugins {
    idea
    kotlin("jvm") version "2.0.0"
    kotlin("plugin.serialization") version "2.0.0"
    "kotlin-dsl"

}

repositories {
    mavenCentral()
    gradlePluginPortal()
}

//tasks.register<DownloadServerJar>("downloadServerJar") {
//    description = "Downloads purpur server"
////    outputs.upToDateWhen { false }
//}
//


//dependencies {
//    implementation("io.ktor:ktor-client-core:${project.ktor_version}")
//    implementation("io.ktor:ktor-client-cio:${project.ktor_version}")
//    implementation("io.ktor:ktor-client-content-negotiation:${project.ktor_version}")
//    implementation("io.ktor:ktor-serialization-kotlinx-json:${project.ktor_version}")


val ktor_version = "latest.release"
dependencies {
    implementation("io.ktor:ktor-client-core:$ktor_version")
    implementation("io.ktor:ktor-client-cio:$ktor_version")
    implementation("io.ktor:ktor-client-content-negotiation:$ktor_version")
    implementation("io.ktor:ktor-serialization-kotlinx-json:$ktor_version")
}