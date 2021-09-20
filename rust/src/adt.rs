//! # ADTs and pattern-matching in Rust
//!
//! Relevant reference manual entries:
//! Match expression: https://doc.rust-lang.org/stable/reference/expressions/match-expr.html
//! Enums: https://doc.rust-lang.org/stable/reference/items/enumerations.html
//!
//! Note that enums are only one of several ways to define new types in Rust.
//! Others include structs, type aliases, and, less commonly, unions.

/// A contrived example for representing height as an ADT, and performing some
/// pattern-matching on it.
pub mod height_enum {

    /// Rust enums support three different kinds of data constructors.
    enum Imperial {
        /// Struct-like data constructors with named fields.
        Mixed { feet: i32, inches: i32 },
        /// Tuple-like data constructors with unnamed fields.
        Inches(i32),
        /// Data constructors with no fields.
        NoHeight,
    }

    /// Rust enums can contain other enums, and the type names and constructor names live in
    /// a different namespace.
    ///
    /// It's possible to use different case conventions, but the recommended style is Pascal case
    /// for type names and constructors, and snake case for field names.
    enum Height {
        Imperial(Imperial),
        Metric { centimeters: i32 },
    }

    /// This is an example of a basic pattern match.
    ///
    /// Matches are expressions, and all match arms must return the same type, but the match arms
    /// can consist of a block (compound expression) that may contain multiple statements.
    /// Single-expression arm expressions are followed by a comma, while compound expressions are
    /// wrapped in braces (and may be optionally be followed by a comma). Consult grammar for
    /// details.
    #[test]
    fn get_height_imperial() {
        let h = Imperial::Mixed { feet: 5, inches: 3 };

        // match expression are expressions.
        let i = match h {
            // Single-expression match arm.
            Imperial::Mixed { feet: f, inches: i } => i * 12 + f,
            // Compound-expression match arm.
            Imperial::Inches(i) => {
                println!("This isn't actually reachable");
                i
            }
            // Last match arm may be optionally followed by a comma.
            Imperial::NoHeight => unreachable!("This is also unreachable"),
        };

        println!("Height is {} inches", i);
    }

    /// Some examples of bad pattern matches.
    ///
    /// Inconsistent return type:
    ///
    /// ```compile_fail
    /// let h = Imperial::Mixed { feet: 5, inches: 3 };
    /// match h {
    ///     Imperial::NoHeight => 1,
    ///     _ => println!("println! does not return an int"),
    /// }
    /// ```
    ///
    /// Incorrect pattern:
    ///
    /// ```compile_fail
    /// let h = Imperial::Mixed { feet: 5, inches: 3 };
    /// match h {
    ///     Imperial::NoHeight => 1,
    ///     Height::Imperial(_) => 2,
    ///     _ => 3,
    /// }
    /// ```
    ///
    /// Inexhaustive branches:
    ///
    /// ```compile_fail
    /// let h = Height::Imperial(Imperial::Mixed { feet: 5, inches: 3 });
    /// match h {
    ///     Height::Imperial(Imperial::Mixed { feet: f, inches: i }) => 1,
    ///     Height::Metric { centimeters: c } => 2,
    /// }
    /// ```
    ///
    /// The following showcases some of the extended features of pattern matches.
    ///
    /// Since all expressions are also statements, match expressions may also be used like switch
    /// statements (without let-binding the value of match arm expressions).
    ///
    /// Rust supports nested pattern matching, match guards, wildcard matches, and multi-pattern
    /// match arms.
    #[test]
    fn get_height() {
        let h = Height::Imperial(Imperial::Mixed { feet: 5, inches: 3 });
        match h {
            // Nested pattern match with guard
            Height::Imperial(Imperial::Mixed { feet: f, inches: i }) if f > 7 => {
                // rustfmt makes us use a block for better style
                println!("This fella's a {} foot and {} inch giant!", f, i)
            }
            // Nested pattern match
            Height::Imperial(Imperial::Mixed { feet: f, inches: i }) => {
                println!("This fella's {} feet and {} inches", f, i)
            }
            // Multiple pattern match
            Height::Metric { centimeters: h } | Height::Imperial(Imperial::Inches(h)) => {
                println!("This bloke's {} height units", h)
            }
            // Wildcard match
            Height::Imperial(_) => println!("some other obscure imperial height, probably"),
        }
    }
}
