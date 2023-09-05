use config::{Config, ConfigError, Environment, File};
use dolphin_config::get_api_config_path;
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Database {
    url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Sparkpost {
    key: String,
    token: String,
    url: String,
    version: u8,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Twitter {
    consumer_token: String,
    consumer_secret: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Braintree {
    merchant_id: String,
    public_key: String,
    private_key: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    debug: bool,
    database: Database,
    sparkpost: Sparkpost,
    twitter: Twitter,
    braintree: Braintree,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        let config_path = get_api_config_path();
        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name(config_path.join("default").to_str().unwrap()))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::with_name(config_path.join( run_mode).to_str().unwrap())
                    .required(false),
            )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name(config_path.join("local").to_str().unwrap()).required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            // You may also programmatically change settings
            .set_override("database.url", "postgres://")?
            .build()?;

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", s.get_bool("debug"));
        println!("database: {:?}", s.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}
#[cfg(test)]
mod tests {

    use config::{Config, Environment, File};

    use std::env;

    use crate::config::Settings;
    use dolphin_config::get_api_config_path;
    #[test]
    fn config_is_work() {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        // get dolphin-api-config path from env
        let config_path = get_api_config_path();
        eprintln!("config_path: {:?}", config_path);
        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name(config_path.join("default").to_str().unwrap()))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::with_name(config_path.join(run_mode).to_str().unwrap())
                    .required(false),
            )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name(config_path.join("local").to_str().unwrap()).required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            // You may also programmatically change settings
            .set_override("database.url", "postgres://").unwrap()
            .build().unwrap();

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", s.get_bool("debug"));
        println!("database: {:?}", s.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        let config = s.try_deserialize::<Settings>();
        eprintln!("config{:?}", config);
    }
}
