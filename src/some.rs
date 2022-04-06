use crate::{Assert, Not};
use std::fmt::Debug;

impl<'a, T> Assert<'a, Option<T>>
where
    T: Debug,
{
    pub fn is_some(&self) -> &Self {
        assert!(
            self.actual.is_some(),
            "should be equal to Some: {:?}",
            self.actual
        );

        self
    }
}

impl<'a, T> Not<'a, Option<T>>
where
    T: Debug,
{
    pub fn is_some(&self) -> &Self {
        assert!(
            self.actual.is_none(),
            "should be equal to None: {:?}",
            self.actual
        );

        self
    }
}
