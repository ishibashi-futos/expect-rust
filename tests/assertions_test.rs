use expect_rs::expect;

#[test]
fn assertions_ok() {
    let actual = "Hello".to_string();

    expect(&actual).assertions(|str| {
        assert_eq!("Hello", str);

        Ok(())
    });
}

#[test]
#[should_panic]
fn assertions_err() {
    let actual = "Hello".to_string();

    expect(&actual).assertions(|_str| Err("panic".to_string()));
}
