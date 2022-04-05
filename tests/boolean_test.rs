use expect::expect;

#[test]
fn is_false_ok() {
    let expect = expect(&false);
    expect.is_false();
}

#[test]
#[should_panic(expected = "expected false, got true")]
fn is_false_should_panic() {
    let expect = expect(&true);
    expect.is_false();
}

#[test]
fn is_true_ok() {
    let expect = expect(&true);
    expect.is_true();
}

// #[test]
// #[should_panic]
// fn not_is_true_ok() {
//     let expect = expect(&true);
//     expect.not().is_true();
// }

// #[test]
// #[should_panic]
// fn not_is_false_ok() {
//     let expect = expect(&false);
//     expect.not().is_false();
// }

#[test]
#[should_panic(expected = "expected true, got false")]
fn is_true_should_panic() {
    let expect = expect(&false);
    expect.is_true();
}
