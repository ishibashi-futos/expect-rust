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
    is_not: bool,
}

pub fn expect<'a, T>(actual: &'a T) -> Assert<'a, T> {
    Assert {
        actual,
        is_not: false,
    }
}

impl<'a, T> Assert<'a, T> {
    pub fn not(&self) -> Self {
        Assert {
            actual: self.actual,
            is_not: !self.is_not,
        }
    }

    fn is_not(&self) -> bool {
        self.is_not
    }
}

#[test]
fn not_once_is_not_true() {
    let expect = expect(&true).not();
    assert!(expect.is_not());
    assert!(expect.is_not);
}

#[test]
fn not_twice_is_not_false() {
    let expect = expect(&true).not().not();
    assert!(!expect.is_not());
    assert!(!expect.is_not);
}
