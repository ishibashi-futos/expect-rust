use expect_rs::expect;

#[test]
fn greater_than_or_equal_to_50_and_50() {
    let actual = 50;
    let expected = 50;

    expect(&actual).greater_than_or_equal_to(&expected);
}

#[test]
fn greater_than_or_equal_to_50_and_49_9999() {
    let actual = 50.0;
    let expected = 49.9999;

    expect(&actual).greater_than_or_equal_to(&expected);
}

#[test]
#[should_panic]
fn greater_than_or_equal_to_50_and_51_should_panic() {
    let actual = 50usize;
    let expected = 51usize;

    expect(&actual).greater_than_or_equal_to(&expected);
}

#[test]
#[should_panic]
fn greater_than_to_50_and_50_should_panic() {
    let actual = 50;
    let expected = 50;

    expect(&actual).greater_than_to(&expected);
}

#[test]
fn greater_than_to_50_and_49_9999() {
    let actual = 50.0;
    let expected = 49.99999999999999;

    expect(&actual).greater_than_to(&expected);
}

#[test]
#[should_panic]
fn greater_than_or_50_and_51_should_panic() {
    let actual = 50usize;
    let expected = 51usize;

    expect(&actual).greater_than_to(&expected);
}

#[test]
fn less_than_or_equal_to_50_and_50() {
    let actual = 50;
    let expected = 50;

    expect(&actual).less_than_or_equal_to(&expected);
}

#[test]
fn less_than_or_equal_to_49_9999_and_50_0000() {
    let actual = 49.9999;
    let expected = 50.0;

    expect(&actual).less_than_or_equal_to(&expected);
}

#[test]
#[should_panic(expected = "should be less than or equal to expected:")]
fn less_than_or_equal_to_50_and_51_should_panic() {
    let actual = 51usize;
    let expected = 50usize;

    expect(&actual).less_than_or_equal_to(&expected);
}

#[test]
#[should_panic(expected = "should be less than to expected:")]
fn less_than_to_50_and_50() {
    let actual = 50;
    let expected = 50;

    expect(&actual).less_than_to(&expected);
}

#[test]
fn less_than_to_49_9999_and_50_0000() {
    let actual = 49.9999;
    let expected = 50.0;

    expect(&actual).less_than_to(&expected);
}

#[test]
#[should_panic(expected = "should be less than to expected:")]
fn less_than_to_50_and_51_should_panic() {
    let actual = 51usize;
    let expected = 50usize;

    expect(&actual).less_than_to(&expected);
}

#[test]
fn in_range_include() {
    let actual = 100;

    expect(&actual).in_range(100..200);
}

#[test]
#[should_panic(expected = "not included in range: 100 in 101..200")]
fn in_range_not_included() {
    let actual = 100usize;

    expect(&actual).in_range(101..200);
}

#[test]
#[should_panic(expected = "not included in range: 200 in 101..200")]
fn in_range_not_included2() {
    let actual = 200usize;

    expect(&actual).in_range(101..200);
}

#[test]
#[should_panic(expected = "not included in range: 100 in 101..=200")]
fn in_range_not_included3() {
    let actual = 100usize;

    expect(&actual).in_range(101..=200);
}

#[test]
fn in_range_include2() {
    let actual = 200usize;

    expect(&actual).in_range(100..=200);
}
