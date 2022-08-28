use derivative::Derivative;
use serde::{ Deserialize };
use std::{ fs::File, io::Read };
use toml::value::Table;

#[derive(Derivative, Deserialize, Clone)]
#[derivative(Debug, Default)]
pub struct Config {
    #[derivative(Default(value = "String::from(\"snitch.db\")"))]
    pub database_name: String,

    #[derivative(Default(value = "10"))]
    pub total_threads: u8,

    #[derivative(Default(value = "String::from(\"TODO\")"))]
    pub prefix: String,
}

fn load_config() -> Option<Table> {
    let mut config_toml = String::new();
    
    let mut file = match File::open("./snitch.toml") {
        Ok(file) => file,
        Err(error) => {
            println!("Error opening config: {:?}", error);
            return None;
        }
    };

    file.read_to_string(&mut config_toml)
            .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

    let config_toml: Table = toml::from_str(&config_toml).unwrap();
    
    Some(config_toml)
}

fn parse_config(config_toml: Table) -> Config {
    let mut base_config = Config::default();

    // TODO: find a more idiomatic solution to populate configuration
    for (key, value) in config_toml {

        if key == "database_name" {
            base_config.database_name = String::from(value.as_str().unwrap());
        } 
        else if key == "total_threads" {
            base_config.total_threads = value.as_integer().unwrap() as u8;
        }
        else if key == "prefix" {
            base_config.prefix = String::from(value.as_str().unwrap());
        }
    }

    base_config
}

pub fn init() -> Config {
    let config = match load_config() {
        Some(config_toml) => parse_config(config_toml),
        None => Config::default()
    };

    config
}
