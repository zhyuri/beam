extern crate beam;
extern crate flexi_logger;
#[macro_use]
extern crate log;

use beam::config::Config;
use beam::ENV_CONFIG_FILE_PATH;
use flexi_logger::{Logger, opt_format};
use std::env;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let config = load_config();
    init_log(&config);

    let listener = TcpListener::bind(config.server().address()).unwrap();
    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        handle_connection(_stream);
    }
}

fn init_log(_config : &Config) {
    Logger::with_env()
        .log_to_file()
        .directory("logs")
        .format(opt_format)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));
}

fn load_config() -> Config {
    let config_file = env::var(ENV_CONFIG_FILE_PATH).unwrap_or_else(|err| {
        eprintln!("Get config path from env {} failed, reason '{}', using default path 'config.yaml'", ENV_CONFIG_FILE_PATH, err);
        String::from("config.yaml")
    });
    Config::load_from(&config_file).unwrap_or_else(|err| {
        eprintln!("Loading config from path {} failed, reason '{}'; Loading default config", config_file, err);
        Config::default()
    })
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
