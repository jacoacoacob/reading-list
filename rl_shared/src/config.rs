use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CliConfig {
    pub browser_bin_path: Option<String>,
    pub browser_args: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct Config {
    pub cli: CliConfig,
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
fn get_rl_database_name() -> String {
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

fn get_rl_assets_dir() -> String {
    match env::var("RL_ASSETS_DIR") {
        Ok(value) if fs::canonicalize(&value).is_ok() => return value,
        Ok(value) => {
            eprintln!("RL_ASSETS_DIR '{value}' must be a valid path");
        }
        Err(_) => { /* Quietly continue */ }
    }

    if cfg!(app_env = "dev") {
        let current_dir = env::current_dir().unwrap();
        
        let mut assets_dir = PathBuf::from(&current_dir);
        assets_dir.push("assets");

        assets_dir.to_str().unwrap().to_string()
    } else {
        let home_dir = env::home_dir().unwrap();
    
        let mut default_rl_assets_dir = PathBuf::from(&home_dir);
        default_rl_assets_dir.push(".rl");
    
        default_rl_assets_dir.to_str().unwrap().to_string()
    }
}

fn get_rl_cli_browser_args() -> Option<Vec<String>> {
    match env::var("RL_CLI_BROWSER_ARGS") {
        Ok(value) => Some(
            value.split(',').map(|arg| arg.to_string()).collect()
        ),
        _ => None
    }
}

fn get_rl_cli_browser_bin_path() -> Option<String> {
    match env::var("RL_CLI_BROWSER_BIN_PATH") {
        Ok(value) if fs::canonicalize(&value).is_ok() => {
            Some(value)
        }
        Ok(value) => {
            eprintln!("RL_CLI_BROWSER_BIN_PATH '{value}' must be a valid path");
            None
        }
        Err(_) => None
    }
}

impl Default for Config {
    fn default() -> Self {
        if cfg!(app_env = "dev") {
            println!("config.rs: app_env == dev");
        }

        Config {
            assets_dir: get_rl_assets_dir(),
            cli: CliConfig {
                browser_args: get_rl_cli_browser_args(),
                browser_bin_path: get_rl_cli_browser_bin_path(),
            },
            database_name: get_rl_database_name(),
        }
    }
}
