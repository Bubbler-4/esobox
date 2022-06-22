//! Rust (re-)implementations of various esolangs
//!
//! Includes implementations of various esolangs through a unified interface.
//! Since many languages have commands for I/O side effects, each implementation
//! takes input and output streams as parameters in addition to the source code:
//!
//! ```ignore
//! pub fn run<I: BufRead, O: Write>(source: &str, input: &mut I, output: &mut O) -> Result<(), Error>
//! ```
//!
//! This `run` function returns `Ok(())` if run successfully, and `Err(...)` if
//! the program was terminated by some kind of error. The `Error` enum is unique
//! to each language, containing all possible error situations. Refer to the
//! respective docs for details.
//!
//! Each language implementation is intended to be "faster than naive",
//! which will often be achieved by compiling "halfway" to bytecode.

#![warn(missing_docs)]

pub mod brainfuck;
