# Programming Language Comparison

A repo where I collect examples from different programming languages to compare
how they work.

## Organization

This repo is organized in *modules*, representing some interesting test case or
language property, implemented for each language.

At the top-level, `doc/` contains all the write-up for each module, while each
of the other directories contains a self-contained project for each language.
In each language directory, there should be a `README.md` containing
instructions about how to run tests in the language.

This is as much an attempt to evaluate the ergonomics of each language (from a
beginner's standpoint) as it is an exploration of their syntax and semantics.
As much as possible, I set up each language subdirectory to use the recommended
build system (or what ever is considered the de facto default). I implement the
modules as unit tests when those are easily available and not too difficult to
set up.

As I implement the modules in each language, I _try_ to do what I think is
idiomatic in that language (even if the directive of the module is not something
the language is designed for).

## Modules

- [L/R-values](doc/lrvalues.md): assigning to variables
