use crate::Assert;

impl<'a, T> Assert<'a, Option<T>> {
    pub fn is_some(&self) {
        if self.is_not() {
            match self.actual {
                Some(_) => panic!(),
                None => {},
            }
        } else {
            match self.actual {
                Some(_) => {},
                None => panic!(),
            }
        }
    }
}
