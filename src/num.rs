use std::{
    fmt::{Debug, Display},
    ops::RangeBounds,
};

use crate::Assert;

impl<'a, T> Assert<'a, T>
where
    T: PartialOrd + Display + Debug,
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

    pub fn in_range<R>(&self, expected: R) -> &Self
    where
        R: RangeBounds<T> + Debug,
    {
        assert!(
            expected.contains(self.actual),
            "not included in range: {} in {:?}",
            self.actual,
            expected,
        );
        self
    }
}
