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
    pub total_threads: usize,

    #[derivative(Default(value = "String::from(\"TODO\")"))]
    pub prefix: String,

    pub owner: String,
    pub repo: String,
    pub token: String,
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
        let stringified_value = String::from(value.as_str().unwrap());

        match key.as_str() {
            "database_name" => base_config.database_name = stringified_value,
            "total_threads" => base_config.total_threads = stringified_value.parse::<usize>().unwrap(),
            "prefix" => base_config.prefix = stringified_value,
            "owner" => base_config.owner = stringified_value,
            "repo" => base_config.repo = stringified_value,
            "token" => base_config.token = stringified_value,
            _ => { println!("Couldn't parse: {:?}", key) }
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
