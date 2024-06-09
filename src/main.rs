use std::env::args;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::parser::Parser;

mod lexer;
mod parser;
mod translator;

fn main() {
    let file_name = args().nth(1).expect("Usage: cargo run <filename>");

    let mut data = String::new();
    let _ = File::open(&file_name)
        .unwrap()
        .read_to_string(&mut data)
        .unwrap();
    let content = data.chars().collect::<Vec<char>>();

    let file_name = Path::new(&file_name).file_name().unwrap().to_str().unwrap();

    let mut parser = Parser::new(&content);
    let mut index = 0;
    while let Some(comm) = parser.next_command() {
        println!("{:?}", comm);
        println!("{:?}", comm.translate(file_name, &mut index));
    }
}
