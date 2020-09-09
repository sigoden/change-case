# Change Case

[![Build status](https://github.com/sigoden/change-case/workflows/ci/badge.svg)](https://github.com/sigoden/change-case/actions)
[![crates.io](https://img.shields.io/crates/v/change-case.svg)](https://crates.io/crates/change-case)
[![Documentation](https://docs.rs/change-case/badge.svg)](https://docs.rs/change-case)
[![Rust](https://img.shields.io/badge/rust-1.32.0%2B-blue.svg?maxAge=3600)](https://github.com/sigoden/change-case)

Transform a string between `camelCase`, `PascalCase`, `Capital Case`, `snake_case`, `param-case`, `CONSTANT_CASE` and others.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
change-case = "0.1"
```

```rust
use change_case::*;

assert_eq!(pascal_case("test string"), "TestString");
assert_eq!(camel_case("Test String"), "testString");
assert_eq!(captial_case("test string"), "Test String");
assert_eq!(snake_case("Test String"), "test_string");
assert_eq!(constant_case("test string"), "TEST_STRING");
assert_eq!(constant_case("test string"), "TEST_STRING");
assert_eq!(param_case("test string"), "test-string");
```


## License

MIT

