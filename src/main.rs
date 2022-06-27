use std::{
    fs,
    io::{stdin, stdout, Read},
};

use clap::{arg, command, value_parser};
use esobox::*;

fn main() {
    let matches = command!()
        .override_usage("esobox <LANGUAGE> <FILE>\n    esobox <LANGUAGE> - <ARGS>...")
        .arg(
            arg!(lang: <LANGUAGE> "Name of the language to run")
                .required(true)
                .value_parser(["brainfuck", "bf"]),
        )
        .arg(
            arg!(file: <FILE> "Name of the source file to run")
                .required(true)
                .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(args: <ARGS> "Arguments to pass into the program (source is taken from stdin)")
                .multiple_values(true)
                .required(false)
                .value_parser(value_parser!(String)),
        )
        .get_matches();
    let lang_name = matches.get_one::<String>("lang").unwrap();
    let file = matches.get_one::<String>("file").unwrap();
    let args = matches.get_many::<String>("args");
    if file == "-" {
        let mut source = String::new();
        let mut stdin = stdin();
        stdin
            .read_to_string(&mut source)
            .expect("Unexpected error while reading source from stdin");
        let mut input = String::new();
        if let Some(args) = args {
            for arg in args {
                input.push_str(arg);
                input.push('\0');
            }
        }
        let mut output = stdout();
        match lang_name as &str {
            "brainfuck" | "bf" => {
                if let Err(error) = brainfuck::run(&source, &mut input.as_bytes(), &mut output) {
                    eprintln!("Error: {:?}", error);
                }
            }
            _ => unreachable!(),
        }
    } else {
        let source =
            fs::read_to_string(file).expect("Unexpected error while reading source from file");
        let mut input = stdin().lock();
        let mut output = stdout();
        match lang_name as &str {
            "brainfuck" | "bf" => {
                if let Err(error) = brainfuck::run(&source, &mut input, &mut output) {
                    eprintln!("Error: {:?}", error);
                }
            }
            _ => unreachable!(),
        }
    }
}
