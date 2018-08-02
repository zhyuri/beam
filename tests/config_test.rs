extern crate beam;

use beam::config::Config;

#[test]
fn read_config_from_file() {
    let config = Config::load_from("tests/asset/test_config.yaml").unwrap();

    assert_eq!(config.server().address(), "localhost:8081", "Testing parse server address");
}