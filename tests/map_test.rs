use std::collections::HashMap;

use expect::expect;

#[test]
fn found_it() {
    let mut map = HashMap::new();
    map.insert("Hello", "World!");

    expect(&map).contains_key(&"Hello");
}

#[test]
fn found_all() {
    let mut map = HashMap::new();
    map.insert("Hello", "World!");
    map.insert("Hallo", "World!");
    map.insert("こんにちは", "世界!");

    let mut expected = HashMap::new();
    expected.insert("Hello", "World!");
    expected.insert("Hallo", "World!");

    expect(&map).contains_all(&expected);
}
