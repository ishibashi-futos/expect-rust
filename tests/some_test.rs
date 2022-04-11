use expect_rs::expect;

#[test]
fn some() {
    let v = Some(100);

    expect(&v).is_some();
}

#[test]
fn not_some() {
    let v: Option<i32> = None;

    expect(&v).not().is_some();
}

#[test]
#[should_panic]
fn none() {
    let v: Option<i32> = None;

    expect(&v).is_some();
}

#[test]
#[should_panic]
fn not_none() {
    let v = Some(100);

    expect(&v).not().is_some();
}

#[test]
fn is_some_equal_to_equal() {
    let v = Some(100);

    expect(&v).is_some_equal_to(&100);
}

#[test]
#[should_panic(expected = "should be equal Some(")]
fn is_some_equal_to_not_equal() {
    let v = Some(100);

    expect(&v).is_some_equal_to(&99);
}

#[test]
#[should_panic(expected = "should be equal Some:")]
fn is_some_equal_to_none_should_panic() {
    let v: Option<i32> = None;

    expect(&v).is_some_equal_to(&99);
}

#[test]
fn unwrap_some_has_some_str() {
    let v = Some("Test");

    expect(&v).unwrap_some().equals(&"Test");
}

#[test]
fn unwrap_some_has_some_num() {
    let v = Some(150);

    expect(&v).unwrap_some().in_range(100..200);
}

#[test]
#[should_panic(expected = "should be equal Some: None")]
fn unwrap_some_none() {
    let v: Option<i32> = None;

    expect(&v).unwrap_some().in_range(100..200);
}
