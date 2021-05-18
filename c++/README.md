# Language Comparison Playground: C++

## Build System: GNU Make

To compile, run:

```
make [targets]
```

By default, the Makefile is configured to build all targets, which will produce
executables that match the name of each module. These will contain assertions
that will fail and crash the program if the test fails.

To run a target, just execute it like any exectuable:

```
./[target]
```

To clean build directory and remove all build artifacts:

```
make clean
```

When adding new modules/targets, make sure to add the executable names to the
`.gitignore` file as well.

TODO: change to CMakelists?
