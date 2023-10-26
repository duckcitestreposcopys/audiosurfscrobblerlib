# AudiosurfScrobblerLib

An extension for preloadenabler that provides the logic needed to submit a scrobble to maloja. Written in Rust and compiled to a C DLL to be put alongside preloadenabler.

Support for other scrobble servers may be added soon.

Absolutely **required** to be built for 32-bit Windows: `cargo build --target=i686-pc-windows-msvc`.
