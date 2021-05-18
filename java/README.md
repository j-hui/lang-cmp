# Language Comparison Playground: Java

## Build System: GNU Make

(Note that I'm using a Makefile for now because I was too lazy to set up an
actual Java build system like Ant, Gradle, or Maven. I might do that later.)

To compile, run:

```
make
```

By default, the Makefile is configured to build all targets, which will produce
`.class` files corresponding to each target. These will contain assertions that
will fail and crash the program if the test fails.

To manually run a target:

```
CLASSPATH=out/ java <target>
```

The Makefile provides a convenience target for this:

```
make run/<target>
```

To clean build directory and remove all build artifacts:

```
make clean
```

When adding new modules/targets, make sure to add the executable names to the
`.gitignore` file as well.
