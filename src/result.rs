use crate::Assert;
use std::fmt::Debug;

impl<'a, S, E> Assert<'a, Result<S, E>>
where
    S: Debug + PartialEq,
    E: Debug + PartialEq,
{
    pub fn is_ok(&self) -> &Self {
        if self.actual.is_err() {
            panic!("must be Ok: actual: {:?}", self.actual);
        }
        self
    }

    pub fn ok_and_equals(&self, expected: &S) -> &Self {
        match &self.actual {
            &Ok(ref v) => {
                if v != expected {
                    panic!(
                        "must be Ok and equals: expected: {:?}, actual: {:?}",
                        expected, v
                    );
                }
            }
            &Err(_) => {
                panic!("must be Ok: actual: {:?}", self.actual);
            }
        }
        self
    }

    pub fn is_err(&self) -> &Self {
        if self.actual.is_ok() {
            panic!("must be Err: actual: {:?}", self.actual);
        }
        self
    }

    pub fn err_and_equals(&self, expected: &E) -> &Self {
        match &self.actual {
            &Ok(_) => {
                panic!("must be Err: actual: {:?}", self.actual);
            }
            &Err(ref e) => {
                if e != expected {
                    panic!(
                        "must be Err and equals: expected: {:?}, actual: {:?}",
                        expected, e
                    );
                }
            }
        }
        self
    }
}
