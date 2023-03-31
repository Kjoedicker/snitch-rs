use derivative::Derivative;
use serde::{ Deserialize };
use std::{ fs::File, io::Read };
use toml::value::Table;

#[derive(Derivative, Deserialize, Clone)]
#[derivative(Debug, Default)]
pub struct Config {
    #[derivative(Default(value = "10"))]
    pub total_threads: usize,
    #[derivative(Default(value = "String::from(\"TODO\")"))]
    pub prefix: String,
    #[derivative(Default(value = "String::from(\"10\")"))]
    pub issues_per_request: String,

    pub owner: String,
    pub repo: String,
    pub token: String,
    pub base_tracker_url: String
}

fn load_config(config_path: &str) -> Option<Table> {
    let mut config_toml = String::new();
    
    let mut file = match File::open(config_path) {
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
            "total_threads" => base_config.total_threads = stringified_value.parse::<usize>().unwrap(),
            "prefix" => base_config.prefix = stringified_value,
            "owner" => base_config.owner = stringified_value,
            "repo" => base_config.repo = stringified_value,
            "token" => base_config.token = stringified_value,
            "issues_per_request" => base_config.issues_per_request = stringified_value,
            "base_tracker_url" => base_config.base_tracker_url = stringified_value,
            _ => { println!("Couldn't parse: {:?}", key) }
        }
    }

    base_config
}

pub fn init() -> Config {
    match load_config("./snitch.toml") {
        Some(config_toml) => parse_config(config_toml),
        None => Config::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod load_config {
        use super::*;

        #[test]
        fn handles_invalid_path () {
            let config = load_config("1");
            assert_eq!(config, None, "load_config should return `None` for invalid paths");
        }

        #[test]
        fn handles_valid_path () {
            let config = load_config("./snitch.toml");
            assert_eq!(config.is_some(), true, "load_config should return `Some` for valid paths");
        }
    }

    mod parse_config {
        use super::*;

        #[test]
        fn maps_config () {
            let config = load_config("./snitch.toml").unwrap();

            let parsed_config = parse_config(config);

            let expected_conditions = &[
                (parsed_config.owner.is_empty(), false, "owner"),
                (parsed_config.repo.is_empty(), false, "repo"),
                (parsed_config.token.is_empty(), false, "token"),
                (parsed_config.prefix.is_empty(), false, "prefix"),
                ((parsed_config.total_threads == 0), false, "total_threads"),
            ];

            for (reality, expectation, desc) in expected_conditions {
                assert_eq!(reality, expectation, "{}", desc);
            }
        }
    }
}

