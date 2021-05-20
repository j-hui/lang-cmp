//! # Assignment semantics for Rust
//!
//! Rust only allows assignments to mutable place expressions.
//!
//! Place expressions are what Rust's reference calls l-values, and can be
//! identified syntactically. That is, only variables, array indexes, field
//! accesses, and dereferences are considered place expressions.
//!
//! Mutability is determined by a combination of the type system and its
//! syntactic context. For variables, this amounts to whether the variable was
//! bound with a `mut` specifier. For aggregate types, the mutability of the
//! whole justifies the mutability of its parts.
//!
//! Note that the reference seems to imply that temporary values can be assigned
//! to, since value expressions are promoted to temporaries in place expression
//! contexts, and temporaries are mutable. However, the section on assignment
//! expressions explicitly forbids using value expressions in the left operand.
//!
//! https://doc.rust-lang.org/reference/expressions/operator-expr.html#assignment-expressions
//! https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions
//! https://doc.rust-lang.org/reference/expressions.html#mutability

/// Declare mutable `x` and immutable `y`, add them together and assign to `x`.
pub mod local_var {
    #[test]
    fn example() {
        let mut x: i32 = 1;
        let y: i32 = 2;

        // x can be assigned because it is a place expression (a variable) that
        // was declared mutable.
        x = x + y;

        assert_eq!(x, 3);
    }

    /// As a sanity check, see to it that compilation fails when `x` is not
    /// declared mutable:
    ///
    /// ```compile_fail
    /// let x: i32 = 1;
    /// let y: i32 = 2;
    ///
    /// x = x + y;
    /// ```
    ///
    /// error[E0384]: cannot assign twice to immutable variable `x`
    ///  --> src/lrvalues.rs:9:1
    ///   |
    /// 3 | let x: i32 = 1;
    ///   |     -
    ///   |     |
    ///   |     first assignment to `x`
    ///   |     help: make this binding mutable: `mut x`
    /// 4 | let y: i32 = 2;
    /// 5 | x = x + y;
    ///   | ^^^^^^^^^ cannot assign twice to immutable variable
    ///
    ///
    ///
    fn compile_fail() {}
}

/// Declare function `do_assign(r, v)` which assigns value v to reference r.
/// Call it to assign `x + y` to `x`.
pub mod func_var {
    /// First we define the do_assign function, which explicitly dereferences
    /// r in order to assign to it.
    pub fn do_assign(r: &mut i32, v: i32) {
        // r can be assigned because it is a place expression (dereference) and
        // mutable (dereference of &mut)
        *r = v;
    }

    #[test]
    fn example() {
        let mut x: i32 = 0;
        let y: i32 = 3;
        let t: i32 = x + y;

        do_assign(&mut x, t);

        assert_eq!(x, 3);
    }

    /// As expected, not declaring `x` as mutable or assigning to the immutable
    /// `y` triggers compiler failures, though these cases are omitted here.
    ///
    /// Failing to take a reference of `x` leads the compiler to complain:
    ///
    /// ```compile_fail
    /// extern crate lang_cmp;
    /// use lang_cmp::lrvalues::func_var::do_assign;
    ///
    /// let mut x: i32 = 0;
    /// let y: i32 = 3;
    /// let t: i32 = x + y;
    /// do_assign(x, t);
    /// ```
    ///
    /// error[E0308]: mismatched types
    ///  --> src/lrvalues.rs:67:11
    ///   |
    /// 9 | do_assign(x, t);
    ///   |           ^
    ///   |           |
    ///   |           expected `&mut i32`, found `i32`
    ///   |           help: consider mutably borrowing here: `&mut x`
    ///
    /// Only writing `&x` and not `&mut x` leads to a similar complaint (and the
    /// same suggestion).
    ///
    /// Note that I had to bind a temporary value `t`, to pass the value of
    /// `x + y` to `do_assign`. Otherwise, the borrow checker complains that
    /// I am attempting to use a mutably borrowed variable:
    ///
    /// ```compile_fail
    /// extern crate lang_cmp;
    /// use lang_cmp::lrvalues::func_var::do_assign;
    ///
    /// let mut x: i32 = 0;
    /// let y: i32 = 3;
    /// do_assign(&mut x, x + y);
    /// assert_eq!(x, 3);
    /// ```
    ///
    /// error[E0503]: cannot use `x` because it was mutably borrowed
    ///  --> src/lrvalues.rs:61:19
    ///   |
    /// 8 | do_assign(&mut x, x + y);
    ///   | --------- ------  ^ use of borrowed `x`
    ///   | |         |
    ///   | |         borrow of `x` occurs here
    ///   | borrow later used by call
    ///
    /// Now we discuss possible failures encountered when writing `do_assign`.
    ///
    /// Omitting the `mut` when declaring `r` in `do_assign` leads to
    /// compilation failure.
    ///
    /// Failing to dereference `r` explicitly leads to a warning:
    ///
    /// ```compile_fail
    /// pub fn do_assign(r: &mut i32, v: i32) {
    ///     r = v;
    /// }
    /// ```
    ///
    /// error[E0308]: mismatched types
    ///  --> src/lrvalues.rs:64:9
    ///   |
    /// 4 |     r = v;
    ///   |         ^ expected `&mut i32`, found `i32`
    ///   |
    /// help: consider dereferencing here to assign to the mutable borrowed piece of memory
    ///   |
    /// 4 |     *r = v;
    ///   |     ^^
    ///
    /// Now say we declare the parameter `v` as `&mut i32` as well. This
    /// _almost_ a valid operation, save for two problems: (1) the parameter `r`
    /// is immutable (which we could fix by declaring it `mut r: &mut i32`); and
    /// (2) the lifetimes of `r` and `v` are incompatible:
    ///
    /// ```compile_fail
    /// pub fn do_assign(r: &mut i32, v: &mut i32) {
    ///     r = v;
    /// }
    /// ```
    ///
    /// error[E0623]: lifetime mismatch
    ///  --> src/lrvalues.rs:85:9
    ///   |
    /// 3 | pub fn do_assign(r: &mut i32, v: &mut i32) {
    ///   |                     --------     -------- these two types are declared with different lifetimes.
    /// ..
    /// 4 |     r = v;
    ///   |         ^ ...but data from `v` flows into `r` here
    ///
    /// If we really wanted to make this compile, we could declare `r` as
    /// mutable, and add explicit lifetime annotations:
    ///
    /// ```
    /// pub fn do_assign<'a>(mut r: &'a mut i32, v: &'a mut i32) {
    ///     r = v;
    /// }
    /// ```
    ///
    /// Note that this still rightfully warns that `r` is assigned but never
    /// read: a hint that this doesn't actually perform the indirect assignment
    /// to the variable underlying `r`.
    fn compile_fail() {}
}

pub mod local_array {
    #[test]
    fn example() {
        let mut x: [i32; 2] = [1, 1];
        let y: i32 = 2;

        x[0] = x[0] + y;

        assert_eq!(x, [3, 1]);
    }
}

pub mod func_array {
    fn do_assign(r: &mut [i32], v: i32) {
        r[0] = v;
    }

    #[test]
    fn example() {
        let mut x: [i32; 2] = [1, 1];
        let y: i32 = 2;

        let t = x[0] + y;
        do_assign(&mut x[..], t);

        assert_eq!(x, [3, 1]);
    }
}

pub mod local_struct {
    #[derive(Debug, PartialEq, Eq)]
    struct S {
        i: i32,
        b: bool,
    }

    #[test]
    fn example() {
        let mut x: S = S { i: 1, b: true };
        let y: i32 = 2;

        x.i = x.i + y;
        assert_eq!(x, S { i: 3, b: true });
    }
}

pub mod func_struct {
    #[derive(Debug, PartialEq, Eq)]
    struct S {
        i: i32,
        b: bool,
    }

    fn do_assign(r: &mut S, v: i32) {
        r.i = v;
    }

    #[test]
    fn example() {
        let mut x: S = S { i: 1, b: true };
        let y: i32 = 2;

        let t = x.i + y;
        do_assign(&mut x, t);

        assert_eq!(x, S { i: 3, b: true });
    }
}

pub mod anomalies {
    #[derive(Debug, PartialEq, Eq)]
    struct S {
        i: i32,
        b: bool,
    }

    #[test]
    fn assign_to_array_literal() {
        [4, 3][0] = 2
    }

    #[test]
    fn assign_to_struct_literal() {
        S { i: 1, b: true }.i = 3
    }
}
