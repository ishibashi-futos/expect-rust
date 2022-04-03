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
        match self.actual.get(key) {
            Some(_) => {
                // ok
            }
            None => {
                panic!("[contains_key]should be found: {:?}", key);
            }
        }
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

        if not_found.len() > 0 {
            panic!("[contains_all]should not be found: {:?}", not_found);
        }

        self
    }
}
