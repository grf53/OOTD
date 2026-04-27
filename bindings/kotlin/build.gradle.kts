import org.gradle.api.tasks.Exec
import org.gradle.api.tasks.compile.JavaCompile
import org.jetbrains.kotlin.gradle.dsl.JvmTarget
import org.jetbrains.kotlin.gradle.tasks.KotlinJvmCompile

plugins {
    kotlin("jvm") version "2.1.10"
    `java-library`
}

group = "io.ootd"
version = "0.1.0"

val minJavaVersion = 22
val currentJvmMajor = JavaVersion.current().majorVersion.toIntOrNull() ?: minJavaVersion
val selectedJavaVersion = maxOf(minJavaVersion, currentJvmMajor)
val repoRoot = project.layout.projectDirectory.dir("../..").asFile
val nativeLibName = when {
    System.getProperty("os.name").lowercase().contains("mac") -> "libootd_ffi_c.dylib"
    System.getProperty("os.name").lowercase().contains("win") -> "ootd_ffi_c.dll"
    else -> "libootd_ffi_c.so"
}
val nativeLibPath = repoRoot.resolve("target/debug/$nativeLibName").absolutePath

java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(selectedJavaVersion))
    }
}

tasks.withType<JavaCompile>().configureEach {
    options.release.set(minJavaVersion)
}

tasks.withType<KotlinJvmCompile>().configureEach {
    compilerOptions {
        jvmTarget.set(JvmTarget.JVM_22)
    }
}

repositories {
    mavenCentral()
}

sourceSets {
    main {
        java.srcDir("../java/src/main/java")
        kotlin.srcDir("src/main/kotlin")
    }
    test {
        kotlin.srcDir("src/test/kotlin")
    }
}

dependencies {
    implementation(kotlin("stdlib"))
    testImplementation(kotlin("test"))
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}

val cargoBuildFfi by tasks.registering(Exec::class) {
    workingDir = repoRoot
    commandLine("cargo", "build", "-p", "ootd-ffi-c")
}

tasks.test {
    dependsOn(cargoBuildFfi)
    useJUnitPlatform()
    systemProperty("ootd.ffi.lib.path", nativeLibPath)
    jvmArgs("--enable-native-access=ALL-UNNAMED")
}
