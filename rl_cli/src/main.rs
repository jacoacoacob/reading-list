fn main() {

    let config = rl_utils::Config::default();

    if cfg!(feature = "dev") {
        println!("{:#?}", config);
    }

    let database = rl_db::Database::new(&config);

    let is_good = database.healthcheck().expect("database healthcheck");

    if cfg!(feature = "dev") {
        println!("database online: {is_good}");
    }

}
