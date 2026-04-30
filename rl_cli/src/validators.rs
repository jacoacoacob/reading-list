use inquire::validator::ErrorMessage;
use inquire::validator::StringValidator;
use inquire::validator::Validation;
use rl_shared::is_bookmark_url_field;

#[derive(Clone)]
pub struct UrlValidator;

impl StringValidator for UrlValidator {
    fn validate(&self, input: &str) -> Result<Validation, inquire::CustomUserError> {
        let input = input.trim();

        if is_bookmark_url_field(input) {
            return Ok(Validation::Valid);
        }

        Ok(Validation::Invalid(ErrorMessage::Custom(format!(
            "input must start with one of 'https://', 'http://', 'file://' (or ':nogo:' if you don't need to link to an external file or web page)"
        ))))
    }
}
