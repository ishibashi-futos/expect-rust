use crate::{Assert, Not};
use std::fmt::Debug;

impl<'a, T> Assert<'a, T>
where
    T: PartialEq + Debug,
{
    pub fn equals(&self, expected: &T) {
        assert_eq!(
            self.actual, expected,
            "should be equal actual: {:?} and expected: {:?}",
            self.actual, expected
        )
    }
}

impl<'a, T> Not<'a, T>
where
    T: PartialEq + Debug,
{
    pub fn equals(&self, expected: &T) {
        assert_ne!(
            self.actual, expected,
            "should not be equal actual: {:?} and expected: {:?}",
            self.actual, expected
        )
    }
}
