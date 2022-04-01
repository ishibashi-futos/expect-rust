use crate::Assert;
use std::fmt::Debug;

impl<'a, T> Assert<'a, Vec<T>>
where
    T: PartialEq + Debug,
{
    pub fn contains_all(&self, expected: &Vec<T>) -> &Self {
        let mut has: Vec<&T> = Vec::new();

        for expect in expected {
            if self.actual.contains(expect) {
                has.push(expect);
            }
        }

        if self.is_not() && expected.len() == has.len() {
            panic!(
                "should not be found: expected: {:?}, actual: {:?}",
                expected, self.actual
            )
        } else if !self.is_not() && expected.len() != has.len() {
            panic!(
                "should be found: expected: {:?}, actual: {:?}",
                expected, self.actual
            )
        }

        self
    }

    pub fn contains(&self, expected: &T) -> &Self {
        if self.is_not() && self.actual.contains(expected) {
            panic!(
                "mustn't be found: array: {:?}, expected: {:?}",
                self.actual, expected
            )
        } else if !self.is_not() && !self.actual.contains(expected) {
            panic!(
                "must be found: array: {:?}, expected: {:?}",
                self.actual, expected
            )
        }
        self
    }
}
