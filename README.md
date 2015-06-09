This is a little macro that compiles regular expressions into
functions. The result can be pretty fast. Due to the limitations of
Rust macros, the syntax isn't quite the same as ordinary regular
expressions, but it's reasonably close.

In lieu of real docs, see the [test file](src/test.rs) for examples.
