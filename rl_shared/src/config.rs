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
    pub database_directory: String,
    pub database_name: String,
    pub assets_dir: String,
}

fn parse_positional_env_arg(arg_name: &str) -> Option<String> {
    let mut args = env::args();

    while args.len() > 0 {
        match args.next() {
            Some(arg) if arg == arg_name => match args.next() {
                Some(arg) => {
                    return Some(arg.to_string());
                }
                _ => {}
            },
            _ => {}
        }
    }

    None
}

/// Return the value passed as an arg (e.g. `--db <database_name>`)
fn resolve_database_name_env_arg() -> String {
    match parse_positional_env_arg("--db") {
        Some(database_name) => {
            let parts: Vec<_> = database_name.split(".").map(|x| x.to_string()).collect();

            match parts.get(0) {
                Some(database_name) => return format!("{database_name}.db"),
                None => {}
            }
        }
        None => {}
    };

    "database.db".to_string()
}

impl Default for Config {
    fn default() -> Self {
        let mut default_assets_dir = env::home_dir().expect("Get home directory");
        default_assets_dir.push(".rl");

        let default_database_directory = env::current_dir().unwrap();

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
            database_name: resolve_database_name_env_arg(),
            database_directory: default_database_directory
                .to_str()
                .expect("convert default_database_directory to &str")
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
                            "database_directory" => {
                                config.database_directory = value.to_string();
                            }
                            "database_name" => {
                                config.database_name = value.to_string();
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

        if cfg!(feature = "dev") {
            println!("{:#?}", config);
        }

        config
    }
}
