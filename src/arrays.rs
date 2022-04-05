use crate::{Assert, Not};
use std::fmt::Debug;

impl<'a, T> Assert<'a, Vec<T>>
where
    T: PartialEq + Debug,
{
    pub fn contains_all(&self, expected: &Vec<T>) -> &Self {
        let mut has: Vec<&T> = Vec::with_capacity(expected.len());

        for expect in expected {
            if self.actual.contains(expect) {
                has.push(expect);
            }
        }

        if expected.len() != has.len() {
            panic!(
                "should be found: expected: {:?}, actual: {:?}",
                expected, self.actual
            )
        }

        self
    }

    pub fn contains(&self, expected: &T) -> &Self {
        if !self.actual.contains(expected) {
            panic!(
                "must be found: array: {:?}, expected: {:?}",
                self.actual, expected
            )
        }
        self
    }
}

impl<'a, T> Not<'a, Vec<T>>
where
    T: PartialEq + Debug,
{
    pub fn contains_all(&self, expected: &Vec<T>) -> &Self {
        let mut has: Vec<&T> = Vec::with_capacity(expected.len());
        for expect in expected {
            if self.actual.contains(expect) {
                has.push(expect);
            }
        }
        if expected.len() != 0 {
            panic!(
                "should"
            )
        }

        self
    }
}
