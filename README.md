# AudiosurfScrobblerLib

## Releases

CI builds are now tested and work! Get them [here](https://nightly.link/duckfromdiscord/audiosurfscrobblerlib/workflows/rust/main/dll_release.zip), or find them in the GitHub Actions builds on every commit.

## Description

An extension for preloadenabler that provides the logic needed to submit a scrobble to maloja. Written in Rust and compiled to a C DLL to be put alongside preloadenabler.

Support for other scrobble servers may be added soon.

Absolutely **required** to be built for 32-bit Windows: `cargo build --target=i686-pc-windows-msvc`