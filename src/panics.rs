use std::panic;

use crate::Assert;

impl<'a, T> Assert<'a, T>
where
    T: Fn() + std::panic::RefUnwindSafe,
{
    pub fn should_panic(&mut self) -> &Self {
        if let Ok(_) = panic::catch_unwind(|| {
            (self.actual)();
        }) {
            panic!("the provided function did not panic as expected")
        }
        self
    }
}
