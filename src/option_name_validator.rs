use crate::Config;
use anyhow::Result;
use inquire::validator::{StringValidator, Validation};
use inquire::CustomUserError;

#[derive(Clone)]
pub struct OptionNameValidator {
    options: Vec<String>,
}

impl OptionNameValidator {
    pub fn new(settings: &Config) -> Result<Self> {
        let options = settings.keys();
        Ok(Self { options })
    }
}

impl StringValidator for OptionNameValidator {
    fn validate(&self, input: &str) -> Result<Validation, CustomUserError> {
        if self.options.contains(&input.to_string()) {
            Ok(Validation::Invalid(
                "An option with this name already exists. Delete it first.".into(),
            ))
        } else {
            Ok(Validation::Valid)
        }
    }
}
