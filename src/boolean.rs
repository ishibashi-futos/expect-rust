use crate::Assert;

impl<'a> Assert<'a, bool> {
    pub fn is_false(&self) -> &Self {
        if self.is_not() {
            assert!(self.actual, "expected true, got false");
            return self;
        }
        assert!(!self.actual, "expected false, got true");
        self
    }

    pub fn is_true(&self) -> &Self {
        if self.is_not() {
            assert!(!self.actual, "expected false, got true");
            return self;
        }
        assert!(self.actual, "expected true, got false");
        self
    }
}
