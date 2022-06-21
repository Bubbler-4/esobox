use std::io::{self, BufRead, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unmatched bracket `{0}`")]
    SyntaxError(char),
    #[error("pointer out of bounds on `{0}`")]
    PointerOutOfBoundsError(char),
    #[error("unexpected I/O error")]
    IoError(#[from] io::Error),
    #[error("unknown error")]
    UnknownError,
}

const MEMORY_SIZE: usize = 30000;

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
                Cmd::Left => ptr = ptr.checked_sub(1).ok_or_else(|| Error::PointerOutOfBoundsError('<'))?,
                Cmd::Right => ptr = Some(ptr + 1).filter(|&x| x < MEMORY_SIZE).ok_or_else(|| Error::PointerOutOfBoundsError('>'))?,
                Cmd::Getc => if let Some(byte) = getc(input)? { memory[ptr] = byte; },
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
    Inc, Dec, Left, Right, Getc, Putc,
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
                let popped = bbno_stack.pop().ok_or_else(|| Error::SyntaxError(']'))?;
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
            _ => ()
        }
    }
    if !bbno_stack.is_empty() {
        Err(Error::SyntaxError('['))?
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
    output.write(&[byte][..])?;
    Ok(())
}