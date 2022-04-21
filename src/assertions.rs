use crate::Assert;

impl<'a, T> Assert<'a, T> {
    pub fn assertions<F>(&self, f: F) -> &Self
    where
        F: Fn(&T) -> Result<(), String>,
    {
        if let Err(err) = f(self.actual) {
            panic!("assertions error: {}", err);
        }
        self
    }
}
