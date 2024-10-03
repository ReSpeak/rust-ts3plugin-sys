TS3Plugin-sys &emsp; [![Latest version](https://img.shields.io/crates/v/ts3plugin-sys.svg)](https://crates.io/crates/ts3plugin-sys)
=============
The documentation can be found here: [![At docs.rs](https://docs.rs/ts3plugin-sys/badge.svg)](https://docs.rs/ts3plugin-sys)

A higher level library that depends on this ffi bindings is [ts3plugin](https://github.com/ReSpeak/rust-ts3plugin).

TeamSpeak 3.6 updates the plugin api version to 26.  
Version 0.5 is compatible with this version.

TeamSpeak 3.3 updates the plugin api version from 22 to 23.  
Version 0.4 is compatible with this version.

TeamSpeak 3.1.1 updates the plugin api version from 21 to 22 but stays backwards compatible so plugins with version 21 can still be loaded.

TeamSpeak 3.1 updates the plugin api version from 20 to 21.  
Version 0.2 and 0.3 are compatible with this version while version 0.1 is
compatible with the api version 20.

Usage
-----
Add the following to your `Cargo.toml`:
```toml
[dependencies]
ts3plugin-sys = "0.5"
```

License
-------
Licensed under either of

 * [Apache License, Version 2.0](LICENSE-APACHE)
 * [MIT license](LICENSE-MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
