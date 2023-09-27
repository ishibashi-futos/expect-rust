use crate::Assert;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

impl<'a, K, V> Assert<'a, HashMap<K, V>>
where
    K: Eq + Hash + Debug + PartialEq,
    V: PartialEq + Debug,
{
    pub fn contains_key(&self, key: &K) -> &Self {
        self.actual.get(key).unwrap_or_else(|| {
            panic!("[contains_key]should be found: {:?}", key);
        });
        self
    }

    pub fn contains_all(&self, expected: &HashMap<K, V>) -> &Self {
        let mut not_found = HashMap::new();

        for (key, value) in expected {
            match self.actual.get(key) {
                Some(v) => {
                    if value != v {
                        not_found.insert(key, value);
                    }
                }
                None => {
                    not_found.insert(key, value);
                }
            }
        }

        assert_eq!(
            not_found.len(),
            0,
            "[contains_all]should not be found: {:?}",
            not_found
        );

        self
    }

    pub fn to_have_item(&self, key: &K, value: &V) -> &Self {
        match self.actual.get(key) {
            Some(v) => {
                assert_eq!(
                    v, value,
                    "should be found: key = {:?}, value = {:?}",
                    key, value
                );
            }
            None => {
                panic!("should be found: key = {:?}, value = {:?}", key, value)
            }
        }
        self
    }
}
