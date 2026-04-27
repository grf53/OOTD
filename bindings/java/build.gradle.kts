plugins {
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

repositories {
    mavenCentral()
}

dependencies {
    testImplementation("org.junit.jupiter:junit-jupiter:5.10.2")
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
