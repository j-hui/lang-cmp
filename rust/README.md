# Language Comparison Playground: Rust

## Build System: Cargo

We use [Cargo][cargo] as our build system for Rust. To build:

```
cargo build
```

We also use Cargo's builtin testing features to write language comparison
modules as [unit tests][unit-tests]. These are just functions tagged with the
`#[test]` attribute, as well as [doc tests][doc-tests] embedded in doc comments.

To run all tests:

```
cargo test
```

[cargo]: https://doc.rust-lang.org/cargo/
[unit-tests]: https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.ht
[doc-tests]: https://doc.rust-lang.org/stable/rust-by-example/testing/doc_testing.html
