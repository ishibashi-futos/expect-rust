use expect::expect;

#[test]
fn equals_str() {
    let actual = "Hoge";

    let expect = expect(&actual);

    expect.equals(&"Hoge");
}

#[test]
fn equals_float() {
    let actual = 100.0f64;

    let expect = expect(&actual);

    expect.equals(&100.0_f64);
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
struct TestStruct {
    x: f64,
    y: f64,
    name: String,
}

#[test]
fn equals_struct() {
    let s = TestStruct {
        x: 100.0,
        y: 3.14,
        name: "TestStruct".to_owned(),
    };

    expect(&s).equals(&TestStruct {
        x: 100.0,
        y: 3.14,
        name: "TestStruct".to_owned(),
    });
}

#[test]
fn not_equals_struct() {
    let s = TestStruct {
        x: 100.0,
        y: 3.14,
        name: "TestStruct".to_owned(),
    };

    expect(&s).not().equals(&TestStruct {
        x: 100.0,
        y: 3.15,
        name: "TestStruct".to_owned(),
    });
}
