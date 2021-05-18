# L/R-values

What are the rules and restrictions surrounding assigning to variables?

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

## Languages
