use std::fmt::Debug;

use crate::{Assert, Not};

impl<'a, S, E> Assert<'a, Result<S, E>>
where
    S: Debug + PartialEq,
    E: Debug + PartialEq,
{
    pub fn is_ok(&self) -> &Self {
        assert!(self.actual.is_ok(), "must be Ok: actual: {:?}", self.actual);
        self
    }

    pub fn ok_and_equals(&self, expected: &S) -> &Self {
        let value = self.actual.as_ref().unwrap_or_else(|_| {
            panic!("must be Ok: actual: {:?}", self.actual);
        });

        assert_eq!(
            value, expected,
            "must be Ok and equals: expected: {:?}, actual: {:?}",
            expected, value
        );
        self
    }

    pub fn is_err(&self) -> &Self {
        assert!(
            self.actual.is_err(),
            "must be Err: actual: {:?}",
            self.actual
        );
        self
    }

    pub fn err_and_equals(&self, expected: &E) -> &Self {
        let error = self.actual.as_ref().err().unwrap_or_else(|| {
            panic!("must be Err: actual: {:?}", self.actual);
        });

        assert_eq!(
            error, expected,
            "must be Err and equals: expected: {:?}, actual: {:?}",
            expected, error
        );
        self
    }
}

impl<'a, S, E> Not<'a, Result<S, E>>
where
    S: Debug + PartialEq,
    E: Debug + PartialEq,
{
    pub fn is_ok(&self) -> &Self {
        assert!(
            self.actual.is_err(),
            "must be Err: actual: {:?}",
            self.actual
        );
        self
    }

    pub fn is_err(&self) -> &Self {
        assert!(self.actual.is_ok(), "must be Ok: actual: {:?}", self.actual);
        self
    }
}
