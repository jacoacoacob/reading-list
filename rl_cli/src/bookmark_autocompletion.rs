use inquire::{Autocomplete, CustomUserError, autocompletion::Replacement};
use rl_shared::Bookmark;

#[derive(Clone)]
pub struct BookmarkAutocompleter {
    bookmarks: Vec<Bookmark>,
}

impl BookmarkAutocompleter {
    pub fn new(bookmarks: Vec<Bookmark>) -> BookmarkAutocompleter {
        BookmarkAutocompleter { bookmarks }
    }
}

struct WeightedSuggestion {
    value: String,
    weight: u8,
}

impl Autocomplete for BookmarkAutocompleter {
    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        Ok(match highlighted_suggestion {
            Some(suggestion) => Replacement::Some(suggestion),
            None => Replacement::None,
        })
    }

    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, inquire::CustomUserError> {
        let input = input.to_lowercase();

        let mut result: Vec<WeightedSuggestion> = vec![];

        if input.trim().len() == 0 {
            let result = self.bookmarks.iter().map(|b| format!("{}", b)).collect();

            return Ok(result);
        }

        for b in &self.bookmarks {
            let mut weight = 0;
            if b.url.to_lowercase().contains(&input) {
                weight += 1;
            }
            if b.name.to_lowercase().contains(&input) {
                weight += 1;
            }
            for t in &b.tags {
                if t.to_lowercase().contains(&input) {
                    weight += 1;
                }
            }
            if weight > 0 {
                result.push(WeightedSuggestion {
                    value: format!("{}", b),
                    weight
                });
            }
        }

        result.sort_by(|a, b| a.weight.cmp(&b.weight));

        Ok(result.iter().map(|x| x.value.clone()).collect())
    }
}
