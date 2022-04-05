use crate::Assert;

impl<'a, T> Assert<'a, Option<T>> {
    pub fn is_some(&self) {
        match self.actual {
            Some(_) => {}
            None => panic!(),
        }
    }
}
