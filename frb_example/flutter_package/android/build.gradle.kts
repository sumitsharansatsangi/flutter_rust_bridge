plugins {
    id("com.android.library")
}

group = "com.flutter_rust_bridge.flutter_package"
version = "1.0"

repositories {
    google()
    mavenCentral()
}

android {
    namespace = "com.flutter_rust_bridge.flutter_package"

    compileSdk = 36

    // Uses the NDK version supplied by the consuming Flutter app.
    ndkVersion = "30.0.14904198"

    defaultConfig {
        minSdk = 24
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_21
        targetCompatibility = JavaVersion.VERSION_21
    }
}

apply(from = "../cargokit/gradle/plugin.gradle")

(extensions.getByName("cargokit") as groovy.lang.GroovyObject).apply {
    setProperty("manifestDir", "../rust")
    setProperty("libname", "flutter_package")
}
