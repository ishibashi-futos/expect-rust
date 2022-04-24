use regex::Regex;

use crate::Assert;

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

    pub fn starts_with(&self, pattern: &str) -> &Self {
        let str = self.actual.to_string();
        assert!(
            str.starts_with(pattern),
            "not started with the specified string. pattern={}, actual={}",
            pattern,
            str
        );

        self
    }
}
