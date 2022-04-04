use expect::expect;

#[test]
fn should_panic_has_err() {
    let f = || {
        panic!("err");
    };

    expect(&f).should_panic();
}

#[test]
#[should_panic(expected = "should panic!")]
fn should_panic_has_not_err() {
    let f = || {
        if true {
            let _result: Result<(), String> = Ok(());
        } else {
            panic!("panic!");
        }
    };

    expect(&f).should_panic();
}
