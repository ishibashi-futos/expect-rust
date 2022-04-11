# expect-rust

This is the `jest expect` like matcher.

## example

### expect

Function to use when you want to test a value. The test always starts here.

```rust
let a = some_function();
expect(&a).equals(&expected);
```

### equals

Evaluate equivalency

```rust
let a = some_function();
expect(&a).equals(&expected);
```

### should panic

Expect the function call to terminate at Panic

```rust
let f = || {
    panic!("err");
};

expect(&f).should_panic(); // Ok
```

```rust
let f = || {
    // non panic
};

expect(&f).should_panic(); // Ng
```

### is some and equals

Tests that one of the values can be obtained and that the expected value can be obtained.

```rust
let v = Some(100);

expect(&v).is_some_equal_to(&100);
```

## Usage

Use the crates.io repository;
add this to your Cargo.toml along with the rest of your dependencies:

```toml
[dependencies]
expect_rs = "*"
```

or

```toml
expect_rs = { git = "https://github.com/ishibashi-futos/expect-rust", branch = "main" }
```

## LICENSE

MIT
