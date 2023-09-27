use expect_rs::expect;

#[test]
fn should_panic_has_err() {
    let f = || {
        panic!("err");
    };

    expect(&f).should_panic();
}

#[test]
#[should_panic(expected = "the provided function did not panic as expected")]
fn should_panic_has_not_err() {
    let f = || {
        let _result: Result<(), String> = Ok(());
    };

    expect(&f).should_panic();
}
