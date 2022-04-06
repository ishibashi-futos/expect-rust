use crate::{Assert, Not};
use std::fmt::Debug;

impl<'a, T> Assert<'a, Option<T>>
where
    T: Debug,
{
    pub fn is_some(&self) -> &Self {
        match self.actual {
            Some(_) => {}
            None => panic!("should be equal to Some: {:?}", self.actual),
        };
        self
    }
}

impl<'a, T> Not<'a, Option<T>>
where
    T: Debug,
{
    pub fn is_some(&self) -> &Self {
        match self.actual {
            Some(_) => panic!("should be equal to None: {:?}", self.actual),
            None => {}
        };
        self
    }
}
