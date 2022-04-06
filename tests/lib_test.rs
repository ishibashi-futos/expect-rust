use expect::expect;

#[test]
fn not_expect_can_be_used_without_even_not() {
    let actual = 100;

    let expect = expect(&actual);
    let n = expect.not();
    n.equals(&200);
    expect.equals(&100);
}
