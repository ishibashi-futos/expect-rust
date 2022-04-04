use crate::Assert;
use regex::Regex;

impl<'a, T> Assert<'a, T>
where
    T: ToString,
{
    pub fn is_match(&self, pattern: &str) -> &Self {
        match Regex::new(pattern) {
            Err(e) => {
                panic!("Invalid pattern: {:?}", e);
            }
            Ok(regex) => {
                assert!(
                    regex.is_match(&self.actual.to_string()),
                    "pattern not found: '{}' in '{}'",
                    pattern,
                    self.actual.to_string()
                );
            }
        };

        self
    }
}
