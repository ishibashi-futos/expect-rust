use crate::Assert;
use std::panic;

impl<'a, T> Assert<'a, T>
where
    T: Fn() + std::panic::RefUnwindSafe,
{
    pub fn should_panic(&mut self) -> &Self {
        match panic::catch_unwind(|| {
            (self.actual)();
            println!("hello");
        }) {
            Ok(_) => {
                panic!("should panic!")
            }
            Err(_) => return self,
        }
    }
}
