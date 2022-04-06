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

        assert!(
            expected.len() == has.len(),
            "should be found: expected: {:?}, actual: {:?}",
            expected,
            self.actual
        );

        self
    }

    pub fn contains(&self, expected: &T) -> &Self {
        assert!(
            self.actual.contains(expected),
            "must be found: array: {:?}, expected: {:?}",
            self.actual,
            expected
        );

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
        assert!(has.len() == 0, "should not have {:?} in actual", expected);

        self
    }

    pub fn contains(&self, expected: &T) -> &Self {
        assert!(
            !self.actual.contains(expected),
            "should not have {:?} in {:?}",
            expected,
            self.actual
        );
        self
    }
}
