use std::fmt::Debug;

use crate::{Assert, Not};

impl<'a, T> Assert<'a, Option<T>>
    where
        T: Debug + PartialEq,
{
    pub fn is_some(&self) -> &Self {
        assert!(
            self.actual.is_some(),
            "should be equal Some: {:?}",
            self.actual
        );

        self
    }

    pub fn is_some_equal_to(&self, expected: &T) -> &Self {
        assert_eq!(
            self.is_some().actual.as_ref().unwrap(),
            expected,
            "should be equal Some({:?}) and {:?}",
            expected,
            self.actual
        );

        self
    }

    pub fn unwrap_some(&self) -> Assert<'a, T> {
        Assert {
            actual: self.is_some().actual.as_ref().unwrap(),
        }
    }

    pub fn is_none(&self) -> &Self {
        assert!(
            self.actual.is_none(),
            "should be equal None: {:?}",
            self.actual
        );

        self
    }
}

impl<'a, T> Not<'a, Option<T>>
    where
        T: Debug + PartialEq,
{
    pub fn is_some(&self) -> &Self {
        assert!(
            self.actual.is_none(),
            "should be equal None: {:?}",
            self.actual
        );

        self
    }

    pub fn is_none(&self) -> &Self {
        assert!(
            self.actual.is_some(),
            "should be equal Some: {:?}",
            self.actual
        );

        self
    }
}
