# Language Comparison Playground: Go

## Build System: Go

Go comes with [its own build system][go-build], which works without a top-level
build configuration.

To build:

```
go build
```

Go also comes with [its own testing framework][go-test], which automatically
runs any function whose name begins with `Test`, in files suffixed `_test.go`.
These should receive a parameter of type [`testing.T`][testing], which is used
to communicate with the test framework.

To run all tests:

```
go test
```

[go-build]: https://golang.org/pkg/go/build/
[go-test]: https://golang.org/pkg/cmd/go/internal/test/
[go-testing]: https://golang.org/pkg/testing/
