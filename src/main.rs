// use esobox::brainfuck::run;
use clap::{arg, command, value_parser, ArgGroup};

fn main() {
    let matches = command!()
        .override_usage("esobox <LANGUAGE> <FILE>\n    esobox <LANGUAGE> -- <ARGS>...")
        .arg(
            arg!(lang: <LANGUAGE> "Name of the language to run")
                .required(true)
                .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(file: <FILE> "Name of the source file to run")
                .required(false)
                .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(args: <ARGS> "Arguments to pass into the program (source is taken from stdin)")
                .multiple_values(true)
                .last(true)
                .conflicts_with("file")
                .required(false)
                .value_parser(value_parser!(String)),
        )
        .group(ArgGroup::new("input")
             .args(&["file", "args"])
             .required(true))
        .get_matches();
    let lang = matches.get_one::<String>("lang");
    let file = matches.get_one::<String>("file");
    let args = matches.get_many::<String>("args");
    println!("{:?}\n{:?}\n{:?}", lang, file, args);
}
