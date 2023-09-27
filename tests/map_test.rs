use std::collections::HashMap;

use expect_rs::expect;

#[test]
fn found_it() {
    let mut map = HashMap::new();
    map.insert("Hello", "World!");

    expect(&map).contains_key(&"Hello");
}

#[test]
fn do_not_found() {
    let map: HashMap<&str, &str> = HashMap::new();
    assert!(std::panic::catch_unwind(|| {
        expect(&map).contains_key(&"Hello");
    })
    .is_err())
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

#[test]
fn to_have_item_found() {
    let mut map = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    expect(&map).to_have_item(&1, &"one");
}

#[test]
#[should_panic]
fn to_have_item_not_found_key() {
    let mut map = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    expect(&map).to_have_item(&4, &"one");
}

#[test]
#[should_panic]
fn to_have_item_not_found_value() {
    let mut map = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    expect(&map).to_have_item(&2, &"three");
}
