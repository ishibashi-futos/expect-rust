use expect_rs::expect;

#[test]
fn is_match() {
    expect(&"2014-01-01").is_match(r"^\d{4}-\d{2}-\d{2}$");
}

#[test]
#[should_panic(expected = "Invalid pattern:")]
fn is_match_invalid_pattern() {
    expect(&"2014-01-01").is_match(r"[0-9]\");
}

#[test]
#[should_panic(expected = "pattern not found:")]
fn is_match_non_match_pattern() {
    expect(&"20140-01-01").is_match(r"^\d{4}-\d{2}-\d{2}$");
}

#[test]
fn starts_with_ok() {
    let str = "2022-04-05";

    expect(&str).starts_with(&"2022-04-");
}

#[test]
#[should_panic(expected = "not started with the specified string.")]
fn starts_with_err() {
    let str = "2022-05-05";

    expect(&str).starts_with(&"2022-04-");
}
