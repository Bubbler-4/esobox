//! An implementation of [Brainfuck].
//!
//! This implementation uses a cyclic memory tape of fixed length (65536) with 8-bit wrapping cells.
//! On EOF, `,` command does not modify the current cell.
//!
//! Since optimizing Brainfuck is a well-studied area and there are various extremely
//! performant implementations out there, this one mostly serves as a practice implementation
//! of a naive bytecode interpreter.
//!
//! [Brainfuck]: https://esolangs.org/wiki/Brainfuck

use std::io::{self, BufRead, Write};
use thiserror::Error;

/// Error enum for Brainfuck.
#[derive(Error, Debug)]
pub enum Error {
    /// Syntax error; the source code contains unmatched brackets.
    #[error("unmatched bracket `{0}`")]
    SyntaxError(char),
    /// I/O error, which may occur during I/O operations.
    #[error("unexpected I/O error")]
    IoError(#[from] io::Error),
}

const MEMORY_SIZE: usize = 65536;

/// Brainfuck interpreter.
pub fn run<I: BufRead, O: Write>(source: &str, input: &mut I, output: &mut O) -> Result<(), Error> {
    let basic_blocks = into_basic_blocks(source)?;
    let mut bb_no = 0usize;
    let mut memory = vec![0u8; MEMORY_SIZE];
    let mut ptr = 0usize;
    loop {
        let BasicBlock { instrs, jz, jnz } = &basic_blocks[bb_no];
        for &instr in instrs {
            match instr {
                Cmd::Inc => memory[ptr] = memory[ptr].wrapping_add(1),
                Cmd::Dec => memory[ptr] = memory[ptr].wrapping_sub(1),
                Cmd::Left => {
                    ptr = ptr.wrapping_sub(1) % MEMORY_SIZE;
                }
                Cmd::Right => {
                    ptr = (ptr + 1) % MEMORY_SIZE;
                }
                Cmd::Getc => {
                    if let Some(byte) = getc(input)? {
                        memory[ptr] = byte;
                    }
                }
                Cmd::Putc => putc(output, memory[ptr])?,
            }
        }
        if let &Some(next_bb) = if memory[ptr] == 0 { jz } else { jnz } {
            bb_no = next_bb;
        } else {
            break;
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Cmd {
    Inc,
    Dec,
    Left,
    Right,
    Getc,
    Putc,
}

#[derive(Debug)]
struct BasicBlock {
    instrs: Vec<Cmd>,
    jz: Option<usize>,
    jnz: Option<usize>,
}

type ByteCodeProgram = Vec<BasicBlock>;

fn into_basic_blocks(source: &str) -> Result<ByteCodeProgram, Error> {
    let mut bbno_stack = vec![]; // stores ids right before `[`
    let mut basic_blocks = vec![];
    let mut cur_basic_block = vec![];
    let mut cur_bb_id = 0usize;
    for c in source.chars() {
        match c {
            '+' => cur_basic_block.push(Cmd::Inc),
            '-' => cur_basic_block.push(Cmd::Dec),
            '<' => cur_basic_block.push(Cmd::Left),
            '>' => cur_basic_block.push(Cmd::Right),
            ',' => cur_basic_block.push(Cmd::Getc),
            '.' => cur_basic_block.push(Cmd::Putc),
            '[' => {
                // starts next basic block
                // jnz target is always bb+1; handle jz target when `]` is found
                let bb = BasicBlock {
                    instrs: cur_basic_block,
                    jz: None,
                    jnz: Some(cur_bb_id + 1),
                };
                basic_blocks.push(bb);
                bbno_stack.push(cur_bb_id);
                cur_bb_id += 1;
                cur_basic_block = vec![];
            }
            ']' => {
                // starts next basic block
                // jz target is bb+1; jnz target is popped+1; jz of popped is bb+1
                let popped = bbno_stack.pop().ok_or(Error::SyntaxError(']'))?;
                let bb = BasicBlock {
                    instrs: cur_basic_block,
                    jz: Some(cur_bb_id + 1),
                    jnz: Some(popped + 1),
                };
                basic_blocks.push(bb);
                basic_blocks[popped].jz = Some(cur_bb_id + 1);
                cur_bb_id += 1;
                cur_basic_block = vec![];
            }
            _ => (),
        }
    }
    if !bbno_stack.is_empty() {
        return Err(Error::SyntaxError('['));
    }
    let bb = BasicBlock {
        instrs: cur_basic_block,
        jz: None,
        jnz: None,
    };
    basic_blocks.push(bb);
    Ok(basic_blocks)
}

fn getc<I: BufRead>(input: &mut I) -> Result<Option<u8>, Error> {
    let buf = input.fill_buf()?;
    let value = buf.get(0).copied();
    input.consume(1);
    Ok(value)
}

fn putc<O: Write>(output: &mut O, byte: u8) -> Result<(), Error> {
    output.write_all(&[byte][..])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn test_printer_1() {
        // from https://codegolf.stackexchange.com/a/108062/78410
        let code = "+[[-<]-[->]<-]<.<<<<.>>>>-.<<-.<.>>.<<<+++.>>>---.<++.";
        let mut stdin = BufReader::new(&b""[..]);
        let mut stdout: Vec<u8> = vec![];
        let res = run(code, &mut stdin, &mut stdout);
        assert!(res.is_ok());
        assert_eq!(stdout, b"brainfuck");
    }

    #[test]
    fn test_printer_2() {
        // from https://codegolf.stackexchange.com/a/99016/78410
        let code = "++[>+++++<-]++[>>>+++++<<<-]++++++[>>+++++++<<-]>[>..........>.<<-]";
        let mut stdin = BufReader::new(&b""[..]);
        let mut stdout: Vec<u8> = vec![];
        let res = run(code, &mut stdin, &mut stdout);
        assert!(res.is_ok());
        assert_eq!(stdout, b"**********\n**********\n**********\n**********\n**********\n**********\n**********\n**********\n**********\n**********\n");
    }

    #[test]
    fn test_io_1() {
        // from https://codegolf.stackexchange.com/a/210478/78410
        let code = "+[,>++++[<-------->-]<]>++++[<++++++++>-]<[>,]++++[<-------->-]<[[-]++++[<-------->-]<]<[<]>>[.>]";
        let testcases = [
            &b"Samantha Vee Hills"[..],
            b"Bob Dillinger",
            b"John Jacob Jingleheimer Schmidt",
            b"Jose Mario Carasco-Williams",
            b"James Alfred Van Allen",
        ];
        let outputs = [
            &b"Vee"[..],
            b"",
            b"Jacob Jingleheimer",
            b"Mario",
            b"Alfred Van",
        ];
        for (testcase, expected) in testcases.into_iter().zip(outputs) {
            let mut stdin = BufReader::new(testcase);
            let mut stdout: Vec<u8> = vec![];
            let res = run(code, &mut stdin, &mut stdout);
            assert!(res.is_ok());
            assert_eq!(stdout, expected);
        }
    }
}
