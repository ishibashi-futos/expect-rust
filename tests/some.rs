use expect::expect;

#[test]
fn some() {
    let v = Some(100);

    expect(&v).is_some();
}

// #[test]
// fn not_some() {
//     let v: Option<i32> = None;

//     expect(&v).not().is_some();
// }

#[test]
#[should_panic]
fn none() {
    let v: Option<i32> = None;

    expect(&v).is_some();
}

// #[test]
// #[should_panic]
// fn not_none() {
//     let v = Some(100);

//     expect(&v).not().is_some();
// }
