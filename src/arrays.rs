use crate::{Assert, Not};
use std::fmt::Debug;

impl<'a, T> Assert<'a, Vec<T>>
where
    T: PartialEq + Debug,
{
    pub fn contains_all(&self, expected: &Vec<T>) -> &Self {
        let has = check_contains(self.actual, expected);
        assert_eq!(
            expected.len(),
            has.len(),
            "should be found: expected: {:?}, actual: {:?}",
            expected,
            self.actual
        );
        self
    }

    pub fn contains(&self, expected: &T) -> &Self {
        assert!(
            self.actual.contains(expected),
            "must be found: expected: {:?}, actual: {:?}",
            self.actual,
            expected
        );

        self
    }

    pub fn len(&self, length: usize) -> &Self {
        assert_eq!(
            self.actual.len(),
            length,
            "equal to length {}. actual = {}",
            length,
            self.actual.len()
        );
        self
    }
}

impl<'a, T> Not<'a, Vec<T>>
where
    T: PartialEq + Debug,
{
    pub fn contains_all(&self, expected: &Vec<T>) -> &Self {
        let has = check_contains(self.actual, expected);
        assert_eq!(has.len(), 0, "should not have {:?} in actual", expected);

        self
    }

    pub fn contains(&self, expected: &T) -> &Self {
        assert!(
            !self.actual.contains(expected),
            "should not have {:?} in {:?}",
            expected,
            self.actual
        );
        self
    }
}

// 配列の要素を検証する共通のヘルパー関数
fn check_contains<'a, T: PartialEq + Debug>(
    actual: &'a Vec<T>,
    expected: &'a Vec<T>,
) -> Vec<&'a T> {
    expected.iter().filter(|&e| actual.contains(e)).collect()
}

#[cfg(test)]
mod tests {
    use super::check_contains;

    #[test]
    fn test_check_contains_all_elements() {
        let actual = vec![1, 2, 3, 4, 5];
        let expected = vec![1, 3, 5];
        let result = check_contains(&actual, &expected);

        assert_eq!(result, vec![&1, &3, &5]);
    }

    #[test]
    fn test_check_contains_some_elements() {
        let actual = vec![1, 2, 3, 4, 5];
        let expected = vec![1, 6, 5];
        let result = check_contains(&actual, &expected);

        assert_eq!(result, vec![&1, &5]);
    }

    #[test]
    fn test_check_contains_no_elements() {
        let actual = vec![1, 2, 3, 4, 5];
        let expected = vec![6, 7, 8];
        let result = check_contains(&actual, &expected);
        assert_eq!(result, vec![] as Vec<&i32>);
    }

    #[test]
    fn test_check_contains_empty() {
        let actual = vec![1, 2, 3, 4, 5];
        let expected = vec![];
        let result = check_contains(&actual, &expected);
        assert_eq!(result, vec![] as Vec<&i32>);
    }
}
