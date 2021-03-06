# Roller Script REPL

This is a reference implementation of a REPL for Roller Script language.

Roller Script is a toy language project of mine, and it's name will probably change in the future.
It is not designed to be a particularly useful language, at the moment, but even this might change.

There is no language reference yet, but you might want to check out the syntax description file at src/parser/expr.lalrpop for a better understanding of the current syntax.
It is written in the [LALRPOP](https://github.com/nikomatsakis/lalrpop) rule syntax.

## What is implemented
* Comments
    * C-style, with nesting block comments (like in Rust)
    * `//` ignores text till end-of-line and all text enclosed between `/*` and _the matching_ `*/` is ignored
    * Maximum block comment nesting depth is 255
* Datatypes
    * None (`none`)
    * Boolean (`true`, `false`)
    * Numerals
        * Only 32-bit ratios atm
        * Accepts scientific syntax, like `2.3e4` or `1e-2`
    * Strings
        * No string operations yet
        * String escaping works, but even escaped backslashes are not allowed before the ending quotation mark
            * (like in Python)
    * List (`[1, 2, 3]`)
        * Implemented as a Rust vector
    * Map (`[1:"a", 2:"b", 3:"c"]`)
        * Implemented as a Rust binary tree map
    * Distribution (`[ 1 | 2 | 3 ]` or `[ true:2 | false:1 ]`)
        * A weighted discrete distribution, implemented as a binary tree map, like the previous map datatype
        * Each element allows an optional weight value separated by a colon
            * The value has to be an integer
        * Duplicate key values are merged and the result's weight value will be their sum
    * Functions (`{a, b; a + b}`)
        * Functions are just another datatype
* Variables
    * Variable identifiers are unicode strings
    * Identifiers start with any character in the word unicode class or underscore, followed by any number of characters from word and number unicode classes and underscores
        * Matched with `[\pL_][\pL\pN_]*` regex pattern
* Arithmetic operations
    * Addition (`+`), substraction, negation (`-`), multiplication (`*`), division (`/`), exponentiation (`^`)
        * Negation requires to be enclosed by parentheses like `(-a)`, like in Haskell.
    * Only defined between numerals
* Boolean operations
    * `not`, `and`, `or` and `xor`
    * Only defined for booleans, for now
* Comparison
    * Allowed comparison operators are `is`, `isnt`, `<`, `>`, `<=` and `>=`
        * Equality and inequality are `a is b` and `a isnt b`
    * Returns a boolean value
* Function call and collection indexing
    * `foo(5)`, `{x y; x + y*2}(1, 2)`
* If-expression
    * `if x is 2 then "hello" else "hi"`
    * Only works for booleans
    * Else-part is required (for now)
        * This makes recursive if expressions are possible
        * Maybe an if-statement that doesn't have else-part and doesn't return a value is added sometime?
* Enabling debug prints via the `--debug`/`-d` flag
* Interpreter commands
    * `#debug [true|false]` enables or disables debug prints

## What is not yet implemented
* So many things
* File input
* Error catching and throwing
* More command-line arguments and REPL-specific flags and variables
* Solution for sequential code execution (code blocks? semicolon operator? just use lists of expressions?)
* Loops
* More builtin functions
    * `read` and `readln`
        * Could input reading operations exploit tokens from the lexer?
    * String operations
    * Collection operations
* Piecewise and assignment operations
    * Easy to implement
* Distribution operations
    * Print distributions nicely, show probabilities, and resolve outputs using a random generator
* Integer logical operations, including bitshifts and maybe rotations
    * Would work best with the better type system (since these are not defined for ratios)
* Comprehensive type system, type constraints
    * Pretty big one
    * Probably after finishing the "MVP"
* List comprehensions
    * Needs type system probably
* Other numerals beside ratios
    * To stop crashing when you type large numbers like `1e10`
    * Bigratios
    * Normal integers and bigints
    * Floats
        * Meeting a NaN value should throw an error
    * Might need the type system for most idiomatic implementation
* Maybe use reference counted or copy-on-write values, or maybe even garbage collecting
    * Current implementation is pretty inefficient and copies data a lot
* References
    * Maybe even move from copy-semantics to reference-semantics
* Using symbol operators as prefixed function names
    * Might be prefixed with the backtick character, like ``` `+ <- 1 2 ```
* Functions as infix operators
    * Also might be prefixed with the backtick
* Some unit tests...

## Possible plans for future
* Separate project into multiple crates
    * Parser
    * AST
    * Library
    * Interpreter and REPL
* Dynamic library loading
* Module/library system
    * Move as many builtins as possible into a prelude module
* Physical unit system/library
    * Bits and bytes, with SI and mebi- kibi- etc prefixes that might be built-in
* LLVM IR compiler frontend?
    * Might be an interesting hobby project
