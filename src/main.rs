extern crate clap;
use clap::{App, Arg};
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let file = Arg::with_name("FILE")
        .help("path to file")
        .required(true)
        .index(1);
    let string = Arg::with_name("STRING")
        .help("string to search, can use regular expression")
        .required(true)
        .index(2);
    let command = App::new("perg")
        .about("dig is a command-line tool to search files for string")
        .arg(string)
        .arg(file)
        .get_matches();

    let path = command.value_of("FILE").unwrap();
    let string = command.value_of("STRING").unwrap();
    let file = File::open(&path).expect("could not read file");

    let mut data = String::new();
    let mut reader = BufReader::new(file);
    reader
        .read_to_string(&mut data)
        .expect("unable to read string");
    dig::search(&data, &string, &mut std::io::stdout(), path);
}