#[derive(Debug)]
enum Profile {
    Release,
    Debug,
}

fn get_profile_from_out_dir() -> Profile {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let mut prev: Option<&str> = None;

    for part in out_dir.split(std::path::MAIN_SEPARATOR) {
        match prev {
            Some(prev_part) if prev_part == "target" => match part {
                "release" => {
                    return Profile::Release;
                }
                "debug" => {
                    return Profile::Debug
                }
                _ => {}
            }
            _ => {}
        }
        prev = Some(part);
    }

    panic!("Couldn't determine profilf from {out_dir}");
}

pub fn set_app_env() {
    let profile = get_profile_from_out_dir();

    println!("cargo::rustc-check-cfg=cfg(app_env, values(\"dev\"))");
    
    match profile {
        Profile::Debug => {
            println!("cargo::rustc-cfg=app_env=\"dev\"");
        }
        _ => {}
    }
}