# Notlin
This is a lexer for a basic Kotlin-like grammar written in Rust.

Instead of using a Peekable iterator, a local variable is used to store characters across loop iterations. This is essentially bringing the [peeked](https://doc.rust-lang.org/src/core/iter/adapters/peekable.rs.html#17) internal field of the Peekable iterator into the code. The performance difference of this approach is likely negligable, but it was a fun experiment.

Making this helped me learn Rust and text parsing skills which have been useful in my work since. It was made while following [Crafting Interpreters](https://craftinginterpreters.com/) by Robert Nystrom.
