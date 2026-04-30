use inquire::{Select, error::InquireResult};

use rl_shared::{Config, Context, Database};

use crate::handlers::{handle_add, handle_search};

mod autocompletion;
mod handlers;
mod validators;

fn main() -> InquireResult<()> {
    let config = Config::default();

    let database = Database::new(&config);

    let ctx = Context::new(&database, &config);

    database.healthcheck().expect("database healthcheck");

    println!("Welcome to your reading list!");

    loop {
        let action = Select::new(
            "Choose an action",
            vec![
                "Search:    search your bookmarks",
                "Add:       add a new bookmark",
            ],
        )
        .with_formatter(&|x| {
            x.to_string()
                .split_inclusive(":")
                .last()
                .expect("get action name")
                .trim()
                .to_string()
        })
        .prompt()?;

        let action = action.split_inclusive(":").next().expect("Get action name");

        match action {
            "Search:" => handle_search(&ctx)?,
            "Add:" => handle_add(&ctx)?,
            _ => {}
        }
    }
}
