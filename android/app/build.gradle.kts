// SoWorker for Android: a GameActivity around the Rust library (android/rust, built by `make android-lib` into
// src/main/jniLibs), plus the few classes Android needs in Java: the call's foreground service and media session,
// the sign-in WebView. See docs/android-plan.md.
plugins {
    id("com.android.application")
}

android {
    namespace = "dev.soworker.app"
    compileSdk = 36

    defaultConfig {
        applicationId = "dev.soworker.app"
        minSdk = 26
        targetSdk = 36
        versionCode = 108
        versionName = "0.1.8"
        ndk { abiFilters += "arm64-v8a" }
    }

    // Release APKs are signed with our own key, kept outside the repository (sideloaded: every update must carry the
    // same signature, so back the keystore up). Set SOWORKER_KEYSTORE, SOWORKER_KEYSTORE_PASS, SOWORKER_KEY_ALIAS and
    // SOWORKER_KEY_PASS in the environment or in ~/.gradle/gradle.properties.
    val keystore = (findProperty("SOWORKER_KEYSTORE") ?: System.getenv("SOWORKER_KEYSTORE"))?.toString()
    signingConfigs {
        if (keystore != null) {
            create("release") {
                storeFile = file(keystore)
                storePassword = (findProperty("SOWORKER_KEYSTORE_PASS") ?: System.getenv("SOWORKER_KEYSTORE_PASS"))?.toString()
                keyAlias = (findProperty("SOWORKER_KEY_ALIAS") ?: System.getenv("SOWORKER_KEY_ALIAS"))?.toString()
                keyPassword = (findProperty("SOWORKER_KEY_PASS") ?: System.getenv("SOWORKER_KEY_PASS"))?.toString()
            }
        }
    }
    buildTypes {
        release {
            isMinifyEnabled = false
            if (keystore != null) signingConfig = signingConfigs.getByName("release")
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    packaging {
        // the .so is already stripped (release) or kept whole for debugging
        jniLibs { keepDebugSymbols += "**/*.so" }
    }
}

kotlin {
    compilerOptions { jvmTarget.set(org.jetbrains.kotlin.gradle.dsl.JvmTarget.JVM_17) }
}

dependencies {
    implementation("androidx.games:games-activity:4.4.0")
    implementation("androidx.appcompat:appcompat:1.7.1")
    implementation("androidx.core:core-ktx:1.13.1")
    implementation("androidx.activity:activity-ktx:1.9.3")
    // WebRTC's Java half (audio device module, Android context): copied here by `make android-lib`
    implementation(files("libs/libwebrtc.jar"))
}
