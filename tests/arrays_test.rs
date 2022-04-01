use expect::expect;

#[test]
fn should_be_found() {
    let arr = vec![1, 2, 3, 4, 5];
    let expect = expect(&arr);
    expect.contains_all(&vec![1, 2, 3]);
}

#[test]
#[should_panic]
fn should_not_be_found() {
    let arr = vec!["a".to_owned(), "b".to_owned(), "c".to_owned()];

    expect(&arr).contains_all(&vec![
        "a".to_owned(),
        "b".to_owned(),
        "c".to_owned(),
        "d".to_owned(),
    ]);
}

#[test]
#[should_panic]
fn not_should_be_found() {
    let arr = vec![1, 2, 3, 4, 5];
    let mut expect = expect(&arr);
    expect.not().contains_all(&vec![1, 2, 3]);
}

#[test]
fn not_should_not_be_found() {
    let arr = vec!["a".to_owned(), "b".to_owned(), "c".to_owned()];

    expect(&arr).not().contains_all(&vec!["d".to_owned()]);
}

#[derive(PartialEq, Debug)]
struct S {
    name: String,
}

#[test]
fn must_be_found() {
    let s = vec![S {
        name: "S".to_owned(),
    }];

    expect(&s).contains(&S {
        name: "S".to_owned(),
    });
}

#[test]
#[should_panic]
fn must_not_be_found() {
    let s = vec![S {
        name: "S".to_owned(),
    }];

    expect(&s).contains(&S {
        name: "SP".to_owned(),
    });
}
