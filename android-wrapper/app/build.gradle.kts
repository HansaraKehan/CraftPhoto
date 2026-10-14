plugins {
    id("com.android.application")
}

android {
    namespace = "com.example.photocraft"
    compileSdk = 35

    defaultConfig {
        applicationId = "com.example.photocraft"
        minSdk = 26
        targetSdk = 35
        versionCode = 1
        versionName = "1.0"
    }

    buildTypes {
        release {
            isMinifyEnabled = false
        }
    }
}

dependencies {
    implementation(platform("org.jetbrains.kotlin:kotlin-bom:1.8.22"))
    implementation("androidx.webkit:webkit:1.12.1")
    implementation("androidx.activity:activity:1.9.3")
}
