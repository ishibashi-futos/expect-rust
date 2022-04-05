mod arrays;
mod boolean;
mod equals;
mod map;
mod num;
mod panics;
mod result;
mod some;
mod strings;

pub struct Assert<'a, T> {
    actual: &'a T,
}

pub struct Not<'a, T> {
    actual: &'a T,
}

pub fn expect<'a, T>(actual: &'a T) -> Assert<'a, T> {
    Assert {
        actual,
    }
}

impl<'a, T> Assert<'a, T> {
    pub fn not(&self) -> Not<'a, T> {
        Not {actual: self.actual}
    }

}
