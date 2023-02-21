<div id="top"></div>

<!-- PROJECT SHIELDS -->

<h1 align="center">
    <a href="https://github.com/pulanski/leafc"><img src="assets/img/logo.png" alt="Rustub logo" width="200" href></a>
    <br>
    Leafc, <em>The Leaf Compiler</em>
    <br>
</h1>

<h4 align="center"><a href="https://leaf.dev">Leaf</a>, a statically-typed Programming Language inspired by, <a href="https://rust-lang/rust" target="_blank">Rust</a>, <a href="https://go.dev" target="_blank">Go</a>, <a href="https://elm-lang.org" target="_blank">Elm</a>, <a href="https://swift.org" target="_blank">Swift</a>, and <a href="https://elixir-lang.org" target="_blank">Elixir</a></h4>

<p align="center">
    <!-- GitHub release -->
    <a href="https://github.com/pulanski/leaf/releases">
        <img src="https://img.shields.io/github/v/release/orhun/git-cliff?style=flat&labelColor=1C2C2E&color=00F88F&logo=GitHub&logoColor=white">
        <!-- <img src="https://img.shields.io/github/v/release/pulanski/leaf?style=flat&labelColor=1C2C2E&color=C96329&logo=GitHub&logoColor=white"> -->
    </a>
    <!-- crates.io version -->
    <a href="https://crates.io/crates/compiler/leafc">
        <img src="https://img.shields.io/crates/v/git-cliff?style=flat&labelColor=1C2C2E&color=00F88F&logo=Rust&logoColor=white">
        <!-- <img src="https://img.shields.io/crates/v/leafc?style=flat&labelColor=1C2C2E&color=00F88F&logo=Rust&logoColor=white"> -->
    </a>
    <!-- lines of code -->
    <a href="https://github.com/pulanski/leaf">
        <img src="https://img.shields.io/tokei/lines/github/pulanski/leaf?style=flat&labelColor=1C2C2E&color=00F88F&logo=GitHub&logoColor=white">
        <!-- <img src="https://img.shields.io/tokei/lines/github/pulanski/leaf?style=flat&labelColor=1C2C2E&color=C96329&logo=GitHub&logoColor=white"> -->
    </a>
    <!-- docs.rs docs -->
    <a href="https://docs.rs/leafc">
        <img src="https://img.shields.io/badge/docs.rs-leafc-00F88F?style=flat&labelColor=1C2C2E&color=00F88F&logo=Rust&logoColor=white">
        <!-- <img src="https://img.shields.io/badge/docs.rs-leafc-C96329?style=flat&labelColor=1C2C2E&color=C96329&logo=Rust&logoColor=white"> -->
    </a>
    <!-- The Leafc Reference -->
    <a href="https://leaf.dev/reference">
        <img src="https://img.shields.io/badge/Leaf-Reference-00F88F?style=flat&labelColor=1C2C2E&color=00F88F&logo=Leaf&logoColor=white">
        <!-- <img src="https://img.shields.io/badge/Leaf-Reference-C96329?style=flat&labelColor=1C2C2E&color=C96329&logo=Leaf&logoColor=white"> -->
    </a>
</p>

<!-- Build Status, a matrix of build statuses for the compiler being cross compiled to different platforms -->

<!--

|                            | **Architecture** |                                                                                       **Build**                                                                                       |
| -------------------------- | :--------------: | :-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: |
| **macOS**                  |      x86_64      |                  [![Build Status](https://ci.swift.org/job/oss-swift-package-macos/lastCompletedBuild/badge/icon)](https://ci.swift.org/job/oss-swift-package-macos)                  |
| **Ubuntu 18.04**           |      x86_64      |           [![Build Status](https://ci.swift.org/job/oss-swift-package-ubuntu-18_04/lastCompletedBuild/badge/icon)](https://ci.swift.org/job/oss-swift-package-ubuntu-18_04)           |
| **Ubuntu 20.04**           |      x86_64      |           [![Build Status](https://ci.swift.org/job/oss-swift-package-ubuntu-20_04/lastCompletedBuild/badge/icon)](https://ci.swift.org/job/oss-swift-package-ubuntu-20_04)           |
| **Ubuntu 20.04**           |     AArch64      |   [![Build Status](https://ci.swift.org/job/oss-swift-package-ubuntu-20_04-aarch64/lastCompletedBuild/badge/icon)](https://ci.swift.org/job/oss-swift-package-ubuntu-20_04-aarch64)   |
| **Ubuntu 22.04**           |      x86_64      |           [![Build Status](https://ci.swift.org/job/oss-swift-package-ubuntu-22_04/lastCompletedBuild/badge/icon)](https://ci.swift.org/job/oss-swift-package-ubuntu-22_04)           |
| **Ubuntu 22.04**           |     AArch64      |   [![Build Status](https://ci.swift.org/job/oss-swift-package-ubuntu-22_04-aarch64/lastCompletedBuild/badge/icon)](https://ci.swift.org/job/oss-swift-package-ubuntu-22_04-aarch64)   |
| **CentOS 7**               |      x86_64      |               [![Build Status](https://ci.swift.org/job/oss-swift-package-centos-7/lastCompletedBuild/badge/icon)](https://ci.swift.org/job/oss-swift-package-centos-7)               |
| **Amazon Linux 2**         |      x86_64      |         [![Build Status](https://ci.swift.org/job/oss-swift-package-amazon-linux-2/lastCompletedBuild/badge/icon)](https://ci.swift.org/job/oss-swift-package-amazon-linux-2)         |
| **Amazon Linux 2**         |     AArch64      | [![Build Status](https://ci.swift.org/job/oss-swift-package-amazon-linux-2-aarch64/lastCompletedBuild/badge/icon)](https://ci.swift.org/job/oss-swift-package-amazon-linux-2-aarch64) |
| **Universal Base Image 9** |      x86_64      |                  [![Build Status](https://ci.swift.org/job/oss-swift-package-ubi-9/lastCompletedBuild/badge/icon)](https://ci.swift.org/job/oss-swift-package-ubi-9)                  |

-->

[repo]: https://github.com/pulanski/leaf
[lines-of-code]: https://img.shields.io/tokei/lines/github/pulanski/leaf?style=flat&labelColor=1C2C2E&color=00F88F&logo=GitHub&logoColor=white
