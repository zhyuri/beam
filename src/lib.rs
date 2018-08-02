extern crate flexi_logger;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

pub mod config;

pub const ENV_CONFIG_FILE_PATH: &str = "BEAM_CONFIG_PATH";
