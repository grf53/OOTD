// swift-tools-version: 5.9

import PackageDescription

let package = Package(
    name: "OOTD",
    platforms: [
        .macOS(.v13),
    ],
    products: [
        .library(name: "OOTD", targets: ["OOTD"]),
        .executable(name: "ootd-parity", targets: ["OOTDParity"]),
    ],
    targets: [
        .target(name: "OOTD"),
        .executableTarget(
            name: "OOTDParity",
            dependencies: ["OOTD"],
            path: "Sources/OOTDParity"
        ),
    ]
)
