use std::{fmt::Display, ops::Range};

use crate::Assert;

impl<'a, T> Assert<'a, T>
where
    T: PartialOrd + Display,
{
    pub fn greater_than_or_equal_to(&self, expected: &T) -> &Self {
        assert!(
            self.actual >= expected,
            "should be greater than or equal to expected: expected={}, actual{}",
            expected,
            self.actual
        );
        self
    }

    pub fn greater_than_to(&self, expected: &T) -> &Self {
        assert!(
            self.actual > expected,
            "should be greater than to expected: expected={}, actual{}",
            expected,
            self.actual
        );
        self
    }

    pub fn less_than_or_equal_to(&self, expected: &T) -> &Self {
        assert!(
            self.actual <= expected,
            "should be less than or equal to expected: expected={}, actual{}",
            expected,
            self.actual
        );
        self
    }

    pub fn less_than_to(&self, expected: &T) -> &Self {
        assert!(
            self.actual < expected,
            "should be less than to expected: expected={}, actual{}",
            expected,
            self.actual
        );
        self
    }

    pub fn in_range(&self, expected: Range<T>) -> &Self {
        assert!(
            expected.contains(self.actual),
            "not included in range: {} in [{}..{}]",
            self.actual,
            expected.start,
            expected.end
        );
        self
    }
}
