use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
};
pub mod arrays;
pub mod equals;
pub mod result;
pub mod some;

pub struct Assert<'a, T> {
    actual: &'a T,
    is_not: Arc<Mutex<bool>>,
}

pub fn expect<'a, T>(actual: &'a T) -> Assert<'a, T> {
    Assert {
        actual,
        is_not: Arc::new(Mutex::new(false)),
    }
}

impl<'a, T> Assert<'a, T> {
    pub fn not(&mut self) -> &Self {
        *(self.is_not.borrow_mut().lock().unwrap()) = true;
        self
    }

    fn is_not(&self) -> bool {
        *self.is_not.as_ref().lock().unwrap()
    }
}
