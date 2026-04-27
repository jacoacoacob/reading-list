use std::env;
use std::fs;

#[derive(Debug)]
pub struct CliConfig {
    pub browser_bin_path: Option<String>,
    pub browser_args: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct Config {
    pub cli: CliConfig,
    pub database_location: String,
    pub assets_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        let mut default_assets_dir = env::home_dir().expect("Get home directory");
        default_assets_dir.push(".rl");

        let mut default_database_location = env::current_dir().unwrap();
        default_database_location.push("database.db");

        let mut config_path = env::current_dir().unwrap();

        config_path.push("config.local");

        let mut config = Config {
            assets_dir: default_assets_dir
                .to_str()
                .expect("convert default_assets_dir to &str")
                .to_string(),
            cli: CliConfig {
                browser_args: None,
                browser_bin_path: None,
            },
            database_location: default_database_location
                .to_str()
                .expect("convert default_database_location to &str")
                .to_string(),
        };

        match fs::read_to_string(config_path) {
            Ok(text) => {
                if text.trim().len() > 0 {
                    for line in text.split('\n') {
                        let mut parts = line.split('=');
    
                        let key = parts.next().expect("get config key");
                        let value = parts.next().expect(&format!("get config value for {key}"));
    
                        match key.to_lowercase().as_str() {
                            "cli.browser_args" => {
                                config.cli.browser_args =
                                    Some(value.split(',').map(|arg| arg.to_string()).collect());
                            }
                            "cli.browser_bin_path" => {
                                config.cli.browser_bin_path = Some(value.to_string());
                            }
                            "assets_dir" => {
                                config.assets_dir = value.to_string();
                            }
                            "database_location" => {
                                config.database_location = value.to_string();
                            }
                            _ => {}
                        }
                    }
                }
            }
            Err(error) => {
                if cfg!(feature = "dev") {
                    eprintln!("Error reading config {:#?}", error);
                }
            }
        }

        config
    }
}
