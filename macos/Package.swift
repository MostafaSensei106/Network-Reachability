// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "network_reachability",
    platforms: [
        .macOS(.v10_15)
    ],
    products: [
        .library(name: "network_reachability", targets: ["network_reachability"])
    ],
    dependencies: [],
    targets: [
        .target(
            name: "network_reachability",
            dependencies: [],
            path: "Classes",
            resources: []
        )
    ]
)
