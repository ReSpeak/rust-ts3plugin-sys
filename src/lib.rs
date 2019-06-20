//! The low-level rust bindings of the TeamSpeak3 plugin API.
//! The original SDK and documentation can be downloaded from
//! the [TeamSpeak website](https://www.teamspeak.com/en/downloads/).

#[macro_use]
extern crate bitflags;

pub mod clientlib_publicdefinitions;
pub mod plugin_definitions;
pub mod public_definitions;
pub mod public_errors;
pub mod ts3functions;
