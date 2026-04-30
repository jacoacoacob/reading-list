use std::{error::Error, fmt::Display, process::Command};

use inquire::{Confirm, CustomUserError, InquireError, Select, Text, error::InquireResult, required};
use rl_shared::{Bookmark, Context, Utc, extract_url_from_autocomplete_selection};

use crate::{autocompletion::BookmarkAutocompleter, validators::UrlValidator};

pub fn handle_add(ctx: &Context) -> InquireResult<()> {
    let url = Text::new("URL")
        .with_validator(required!("URL is required"))
        .with_validator(UrlValidator)
        .with_help_message("[required] add a URL or file path for this bookmark")
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

    match ctx.database.add_bookmark(&bookmark, None) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{:#?}", error);
        }
    }

    Ok(())
}

pub fn handle_search(ctx: &Context) -> InquireResult<()> {
    let bookmarks = ctx.database.list_bookmarks(None).expect("list bookmarks");

    let choice = Text::new("")
        .with_autocomplete(BookmarkAutocompleter::new(bookmarks))
        .with_formatter(&|x| x.split('\n').next().expect("get first line").to_string())
        .prompt()?;

    match extract_url_from_autocomplete_selection(&choice) {
        Some(url) => match ctx.database.get_bookmark_by_url(&url, None) {
            Ok(bookmark) => {
                let action = Select::new(
                    &format!("selected bookmark:\n{:#}", bookmark),
                    vec![
                        "Visit:     open this bookmark's file or website (if it links to one)",
                        "Edit:      edit this bookmark",
                        "Delete:    delete this bookmark",
                        "Back:      go back to search",
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
                    "Visit:" => handle_visit(ctx, &bookmark)?,
                    "Edit:" => handle_edit(ctx, &bookmark)?,
                    _ => {}
                }
            }
            _ => {
                eprintln!("Couldn't find bookmark with url '{url}'");
            }
        },
        None => {
            eprintln!("Couldn't extract a url from selection\n\n  {choice}");
        }
    }

    Ok(())
}

#[derive(Debug)]
struct MyError(String);

impl MyError {
    pub fn new(message: &str) -> MyError {
        MyError(message.to_string())
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for MyError {}

pub fn handle_visit(ctx: &Context, bookmark: &Bookmark) -> InquireResult<()> {
    if bookmark.is_nogo() {
        let error = Box::new(MyError::new(
            "Unable to open a file or web page for this bookmark. Try updating the URL to a proper URL or file path",
        ));

        return Err(InquireError::Custom(error));
    }

    match &ctx.config.cli.browser_bin_path {
        Some(browser_bin_path) => {
            let mut command = Command::new(browser_bin_path);

            if let Some(args) = &ctx.config.cli.browser_args {
                command.args(args);
            }

            match command.arg(&bookmark.url).spawn() {
                Ok(_) => {
                    let mut new_bookmark = bookmark.clone();
                    
                    new_bookmark.visited = Utc::now();

                    match ctx.database.update_bookmark(bookmark, &new_bookmark, None) {
                        Ok(_) => {}
                        Err(error) => {
                            return Err(
                                InquireError::Custom(
                                    Box::new(error)
                                )
                            )
                        }
                    }
                }
                Err(error) => {
                    return Err(
                        InquireError::Custom(
                            Box::new(error)
                        )
                    )
                }
            }
        }
        None => {
            eprintln!(
                "Please add an entry for 'cli.browser_bin_path' in your installation's config.local file"
            );
        }
    }

    Ok(())
}

pub fn handle_edit(ctx: &Context, bookmark: &Bookmark) -> InquireResult<()> {
    let mut new_bookmark = bookmark.clone();

    new_bookmark.url = Text::new("URL:")
        .with_help_message("Update this bookmark's URL")
        .with_initial_value(&bookmark.url)
        .prompt()?;

    new_bookmark.name = Text::new("Name:")
        .with_help_message("Update this bookmark's name")
        .with_initial_value(&bookmark.name)
        .prompt()?;

    new_bookmark.tags = Text::new("Tags:")
        .with_initial_value(bookmark.tags.join(",").as_str())
        .with_help_message("Update this bookmark's tags")
        .prompt()?
        .split(",")
        .map(|tag| tag.trim().to_string())
        .filter(|tag| tag.len() > 0)
        .collect();

    let confirm_message =
        &format!("Are you sure you want to save\n\n {:#}", new_bookmark);

    let did_confirm = Confirm::new(&confirm_message).prompt()?;
    
    if did_confirm {
        new_bookmark.updated = Utc::now();

        match ctx.database.update_bookmark(&bookmark, &new_bookmark, None) {
            Ok(_) => {}
            Err(error) => {
                return Err(
                    InquireError::Custom(
                        Box::new(error)
                    )
                )
            }
        }
    }

    Ok(())

}