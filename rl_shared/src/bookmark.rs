use std::fmt::Display;

use chrono::{DateTime, Utc};

pub fn is_bookmark_url_field(value: &str) -> bool {
    let value = value.trim();

    value.starts_with("https://")
        || value.starts_with("http://")
        || value.starts_with("file://")
        // :nogo: is for bookmarks that don't need to link to an external file or web page
        // (i.e. ones you're saving during development and testing)
        || value.starts_with(":nogo:")
}

pub fn extract_url_from_autocomplete_selection(choice: &str) -> Option<String> {
    for line in choice.split('\n') {
        let line = line.trim();

        if is_bookmark_url_field(line) {
            return Some(line.to_string());
        }
    }

    None
}

#[derive(Debug, Clone)]
pub struct Bookmark {
    pub url: String,
    pub name: String,
    pub tags: Vec<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub visited: DateTime<Utc>,
}

impl Bookmark {
    pub fn new(url: &str, name: &str, tags: Vec<String>) -> Bookmark {
        Bookmark {
            url: url.to_string(),
            name: name.to_string(),
            tags,
            created: Utc::now(),
            updated: Utc::now(),
            visited: Utc::now(),
        }
    }
}

impl Bookmark {
    pub fn builder() -> BookmarkBuilder {
        BookmarkBuilder::default()
    }

    pub fn is_nogo(&self) -> bool {
        return self.url.starts_with(":nogo:");
    }
}

#[derive(Debug, Default)]
pub struct BookmarkBuilder {
    url: Option<String>,
    name: Option<String>,
    tags: Vec<String>,
    created: Option<DateTime<Utc>>,
    updated: Option<DateTime<Utc>>,
    visited: Option<DateTime<Utc>>,
}

impl BookmarkBuilder {
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags.iter().map(|tag| tag.to_string()).collect();
        self
    }

    pub fn created(mut self, created: DateTime<Utc>) -> Self {
        self.created = Some(created);
        self
    }

    pub fn updated(mut self, updated: DateTime<Utc>) -> Self {
        self.updated = Some(updated);
        self
    }

    pub fn visited(mut self, visited: DateTime<Utc>) -> Self {
        self.visited = Some(visited);
        self
    }

    pub fn build(self) -> Bookmark {
        Bookmark {
            url: self.url.unwrap(),
            name: self.name.unwrap(),
            tags: self.tags,
            created: self.created.unwrap_or_else(|| Utc::now()),
            updated: self.updated.unwrap_or_else(|| Utc::now()),
            visited: self.visited.unwrap_or_else(|| Utc::now()),
        }
    }
}

impl Display for Bookmark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            let name = self.name.to_string();
            let tags = self.tags.join(",");
            let url = self.url.to_string();
            let created = self.created;
            let updated = self.updated;
            let visited = self.visited;

            write!(
                f,
                "
   name    | {name}
   tags    | {tags}
   url     | {url}
   created | {created}
   updated | {updated}
   visited | {visited}
"
            )
        } else {
            let mut display = String::from("");

            display.push_str(&format!("{}\n", self.name));

            if self.tags.len() > 0 {
                display.push_str(&format!("  {}\n", self.tags.join(", ")));
            }

            display.push_str(&format!("  {}\n", self.url));

            display.push_str("  ");

            write!(f, "{}", display)
        }
    }
}
