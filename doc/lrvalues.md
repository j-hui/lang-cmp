# L/R-values

What are the rules and restrictions surrounding the assignment operation? What
is a variable? What are the rules surrounding their designation of a location
versus their symbolizing a value?

## Examples

### Assignment to locally-declared variable

Declare mutable integer variable `x`, initialized to `1`, and immutable variable
`y`, initialized to `2`. Directly assign `x + y` to `x`, i.e., `x = x + y`.

### Assignment to variable via function call

Declare mutable integer variable `x`, initialized to `1`, and immutable variable
`y`, initialized to `2`. Assign `x + y` to `x` indirectly using a `do_assign`
function.

### Assignment to field of locally-declared struct

Declare mutable integer array `x`, initialized to `[1, 1]`, and immutable
variable `y`, initialized to `2`. Directly assign `x[0] + y` to `x[0]`, i.e.,
`x[0] = x[0] + y`.

### Assignment to struct field via function call

Declare mutable integer array `x`, initialized to `[1, 1]`, and immutable
variable `y`, initialized to `2`. Assign `x[0] + y` to `x` indirectly using a
`do_assign` function.

### Assignment to element of locally-declared array

Declare a mutable struct `x`, containing integer field `x.i` and boolean field
`x.b`, initialized to `{i: 1, b: true}`. Directly assign `x.i + y` to `x.i`,
i.e., `x.i = x.i + y`.

### Assignment to array element via function call

Declare a mutable struct `x`, containing integer field `x.i` and boolean field
`x.b`, initialized to `{i: 1, b: true}`. Assign `x.i + y` to `x.i` indirectly
using a `do_assign` function.

## Discussion

Test cases were implemented in the following languages:

- C
- C++
- Rust
- Java
- Scala
- OCaml
- Go
- Zig

### When can we assign

Most languages surveyed classify expressions using some notion of l-values
versus r-values, and specify that the left operand of an assignment must be an
l-value. The expressions that comprise l-values typically comprise variables
themselves and the result of dereferences, array accesses, and struct member
accesses. The determination of l-valuedness, and thus the validity of an
assignment, takes place during semantic checking, after parsing.

The exceptions to this pattern were in OCaml and Scala. OCaml does not support
assignment to regular variables, only to `ref` type variables. Thus, it relies
on its type system to determine whether an assignment is valid. Scala, on the
other hand, supports direct assignment to variables, array elements, and struct
members. However, rather than justify this operation on the basis of
l-valuedness, assignment is a syntactically sanctioned operation: the grammar
only allows variables and function application results appearing as the left
operand of an assignment.

### What does it mean to assign

In most languages that support mutable variables, there is an inconsistency
between what it means for a variable to appear as the left operand versus the
right operand. When a programmer comes across a statement of the form:

```
x = y
```

The LHS expression `x` designates a *location*---where we are assigning
to---while the RHS expression `y` designates a *value*---what we are assigning
to that location. Despite the semantic inconsistency, many languages would like
to support this syntax, and justify this syntax in different ways.

Most of the languages I surveyed---C, C++, Rust, Go, and Zig---embrace this
inconsistency, and insist that l-values correspond to locations. In particular,
when a variable is declared and initialized, the variable is understood to live
somewhere visible to the programmer. These are the languages that have
first-class references types (e.g., pointers), where it is natural to also say
that l-values also correspond to terms whose address can be taken (e.g., with
the `&` operator), to promote an l-value to a reference type value.

Other languages---Java, Scala, and OCaml---pad the notion of variables with
a layer of indirection to justify the apparent inconsistency: variables are
inherently *references* to values. When an assignment of the form `x = y` takes
place, `x` is made to point at the same object that `y` points to, creating an
alias. These semantics are also shared widely among less low-level languages
such as Python, Julia, Crystal, and Kotlin. A common trend seems to be that
none of these languages directly distinguish between values and references
(i.e., they don't have pointer types), and all rely on garbage collection.
