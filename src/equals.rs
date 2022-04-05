use crate::Assert;
use std::fmt::Debug;

impl<'a, T> Assert<'a, T>
where
    T: PartialEq + Debug,
{
    pub fn equals(&self, expected: &T) {
        assert_eq!(self.actual, expected)
    }
}
