// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "IntentsBridge",
    platforms: [
        .macOS(.v11)
    ],
    products: [
        .library(
            name: "IntentsBridge",
            type: .static,
            targets: ["IntentsBridge"])
    ],
    targets: [
        .target(
            name: "IntentsBridge",
            path: "Sources/IntentsBridge",
            publicHeadersPath: "include")
    ]
)
