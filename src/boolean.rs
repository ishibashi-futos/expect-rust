use crate::{Assert, Not};

impl<'a> Assert<'a, bool> {
    pub fn is_false(&self) -> &Self {
        assert!(!self.actual, "expected false, got true");
        self
    }

    pub fn is_true(&self) -> &Self {
        assert!(self.actual, "expected true, got false");
        self
    }
}

impl<'a> Not<'a, bool> {
    pub fn is_false(&self) -> &Self {
        assert!(self.actual, "expected true, got false");
        self
    }

    pub fn is_true(&self) -> &Self {
        assert!(!self.actual, "expected false, got true");
        self
    }
}
