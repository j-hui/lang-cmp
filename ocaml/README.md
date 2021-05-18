# Language Comparison Playground: OCaml

## Build System: Dune

For OCaml, we use the [Dune][dune] framework. The build configuration is specified
inside the `dune` file.

To build:

```
dune build
```

Tests are written using Jane Street's [`ppx_inline_test`][ppx] framework, which
allows tests to be written inline as:

```ocaml
let%test "name" = (* true for success, false otherwise *)
```

These can be run using:

```
dune runtest
```

[dune]: https://dune.build/
[ppx]: https://github.com/janestreet/ppx_inline_test
