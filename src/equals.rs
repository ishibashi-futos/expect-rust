use crate::Assert;
use std::fmt::Debug;

impl<'a, T> Assert<'a, T>
where
    T: PartialEq + Debug,
{
    pub fn equals(&self, expected: &T) {
        if self.is_not() {
            assert_ne!(self.actual, expected)
        } else {
            assert_eq!(self.actual, expected)
        }
    }
}
