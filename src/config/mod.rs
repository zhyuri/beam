use self::server::ServerConfig;
use serde::de;
use serde::Deserialize;
use serde_yaml;
use std::env;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::result::Result;


pub mod server;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {

    #[serde(default, deserialize_with = "failure_default")]
    server: ServerConfig,
}

impl Config {

    /// Get server config
    #[inline]
    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn load_from<P: Into<PathBuf>>(path: P) -> Result<Config, Error> {
        let path = path.into();
        let file = File::open(path)?;

        let config: Config = serde_yaml::from_reader(file)?;

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            server: ServerConfig::default(),
        }
    }
}

/// Errors occurring during config loading
#[derive(Debug)]
pub enum Error {
    /// Config file not found
    NotFound,

    /// Config file empty
    Empty,

    /// Couldn't read $HOME environment variable
    ReadingEnvHome(env::VarError),

    /// io error reading file
    Io(io::Error),

    /// Not valid yaml or missing parameters
    Yaml(serde_yaml::Error),
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NotFound => "could not locate config file",
            Error::Empty => "empty config file",
            Error::ReadingEnvHome(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
            Error::Yaml(ref err) => err.description(),
        }
    }
    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::NotFound | Error::Empty => None,
            Error::ReadingEnvHome(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::Yaml(ref err) => Some(err),
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Error::NotFound | Error::Empty => write!(f, "{}", ::std::error::Error::description(self)),
            Error::ReadingEnvHome(ref err) => {
                write!(f, "could not read $HOME environment variable: {}", err)
            }
            Error::Io(ref err) => write!(f, "error reading config file: {}", err),
            Error::Yaml(ref err) => write!(f, "problem with config: {}", err),
        }
    }
}

impl From<env::VarError> for Error {
    fn from(val: env::VarError) -> Error {
        Error::ReadingEnvHome(val)
    }
}

impl From<io::Error> for Error {
    fn from(val: io::Error) -> Error {
        if val.kind() == io::ErrorKind::NotFound {
            Error::NotFound
        } else {
            Error::Io(val)
        }
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(val: serde_yaml::Error) -> Error {
        Error::Yaml(val)
    }
}

fn failure_default<'a, D, T>(deserializer: D)
                             -> ::std::result::Result<T, D::Error>
    where D: de::Deserializer<'a>,
          T: Deserialize<'a> + Default
{
    match T::deserialize(deserializer) {
        Ok(value) => Ok(value),
        Err(err) => {
            eprintln!("problem with config: {}; Using default value", err);
            Ok(T::default())
        }
    }
}
