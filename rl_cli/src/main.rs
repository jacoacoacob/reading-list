use inquire::{Select, Text, error::InquireResult, required};

use rl_shared::{Bookmark, Config, Database};

use crate::bookmark_autocompletion::BookmarkAutocompleter;

mod bookmark_autocompletion;

fn main() -> InquireResult<()> {
    let config = Config::default();

    let database = Database::new(&config);

    database.healthcheck().expect("database healthcheck");

    println!("Welcome to your reading list!");

    loop {
        let action = Select::new(
            "Choose an action",
            vec![
                "Search:    search your bookmarks",
                "Add:       add a new bookmark",
            ]
        ).prompt()?;

        let action = action.split_inclusive(":").next().expect("Get action name");

        match action {
            "Search:" => {
                let bookmarks = database.list_bookmarks(None).expect("list bookmarks");                

                let choice = Text::new("Search bookmarks")
                    .with_autocomplete(BookmarkAutocompleter::new(bookmarks))
                    .prompt()?;

                

                println!("{:#}", choice);

            }
            "Add:" => {
                let url = Text::new("URL")
                    .with_validator(required!("URL is required"))
                    .with_help_message("[required] add the URL for this bookmark")
                    .prompt()?;

                let name = Text::new("Name")
                    .with_help_message("[optional] give this bookmark a descriptive name")
                    .prompt()?;
                let tags = Text::new("Tags")
                    .with_help_message("[optional] add comma-separated tags for this bookmark")
                    .prompt()?
                    .split(',')
                    .map(|tag| tag.trim().to_string())
                    .filter(|tag| tag.len() > 0)
                    .collect();

                let bookmark = Bookmark::builder()
                    .url(&url.trim())
                    .name(&name.trim())
                    .tags(tags)
                    .build();

                match database.add_bookmark(&bookmark, None) {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("{:#?}", error);
                    }
                }

            }
            _ => {}
        }
    }
}
