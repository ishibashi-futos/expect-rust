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
