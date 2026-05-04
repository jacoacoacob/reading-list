use std::fs;
use std::path::PathBuf;

fn main() {
    rl_build_utils::set_app_env();

    let mut config = rl_shared::Config::default();

    let assets_dir = PathBuf::from(&config.assets_dir);
    
    let parent = assets_dir.parent().unwrap();

    if assets_dir.ends_with("assets") && parent.ends_with("rl_cli") {
        let mut assets_dir = parent.parent().unwrap().to_path_buf();
        assets_dir.push("assets");
        
        config.assets_dir = assets_dir.to_str().unwrap().to_string();
    }


    match fs::canonicalize(&config.assets_dir) {
        Ok(path) if path.is_dir() => {/* assets dir exists */}
        _ => {
            // path is either file or doesn't exist
            fs::create_dir_all(
                PathBuf::from(config.assets_dir)
            ).expect("create assets dir")
        }
    }
}
