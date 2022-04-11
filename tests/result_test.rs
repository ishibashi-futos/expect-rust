use expect_rs::expect;

#[test]
fn is_ok_ok_passed() {
    let actual: Result<i32, String> = Ok(1);
    expect(&actual).is_ok();
}

#[test]
#[should_panic]
fn not_is_ok_ok_should_panic() {
    let actual: Result<i32, String> = Ok(1);
    expect(&actual).not().is_ok();
}

#[test]
#[should_panic(expected = "must be Ok: actual: Err(\"Panic!\")")]
fn is_ok_err_should_panic() {
    let actual: Result<i32, String> = Err("Panic!".to_owned());
    expect(&actual).is_ok();
}

#[test]
fn not_is_ok_err() {
    let actual: Result<i32, String> = Err("Panic!".to_owned());
    expect(&actual).not().is_ok();
}

#[test]
fn is_err_err_passed() {
    let actual: Result<i32, String> = Err("Panic!".to_owned());
    expect(&actual).is_err();
}

#[test]
#[should_panic(expected = "must be Err: actual: Ok(1)")]
fn is_err_ok_should_panic() {
    let actual: Result<i32, String> = Ok(1);
    expect(&actual).is_err();
}

#[test]
fn ok_and_equals_ok_is_equal() {
    let actual: Result<f32, String> = Ok(100.0);
    expect(&actual).ok_and_equals(&100.0);
}

#[test]
#[should_panic(expected = "must be Ok: actual: Err(\"panic!panic!\")")]
fn ok_and_equals_err_should_panic() {
    let actual: Result<f32, String> = Err("panic!panic!".to_owned());
    expect(&actual).ok_and_equals(&100.0);
}

#[test]
fn err_and_equals_err_is_equal() {
    let actual: Result<(), String> = Err("panic".to_owned());
    expect(&actual).err_and_equals(&"panic".to_owned());
}

#[test]
#[should_panic(expected = "must be Err: actual: Ok(100.0)")]
fn err_and_equals_err_should_panic() {
    let actual: Result<f32, String> = Ok(100.0);
    expect(&actual).err_and_equals(&"panic".to_owned());
}
