use crate::parser::Parser;
use crate::translator::Translator;
use std::env::args;
use std::fs::{read_dir, File};
use std::io::{Read, Write};
use std::path::Path;

mod command;
mod lexer;
mod parser;
mod translator;

fn main() {
    let path_input = args().nth(1).expect("Usage: cargo run <filename>");
    let path = Path::new(&path_input);

    let file_paths: Vec<_>;

    // Create the assembly file and begin writing the init code
    let mut file = File::create(path.with_extension("asm")).unwrap();
    let mut translator = Translator::new(path_input.clone());

    if path.is_dir() {
        let files = read_dir(path).unwrap();
        file_paths = files
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                if path.is_file() && path.extension().unwrap() == "vm" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();
        if !file_paths
            .iter()
            .any(|x| x.file_name().unwrap() == "Sys.vm")
        {
            panic!("Sys.vm not found in the directory");
        }
        // Init command is only written when a directory is passed
        let init_command = translator.init().join("\n") + "\n";
        file.write_all("// call Sys.init\n".as_bytes()).unwrap();
        file.write_all(init_command.as_bytes()).unwrap();
    } else {
        // Otherwise simply convert the file
        file_paths = vec![path.with_extension("vm")];
    }

    // Translate all the files and write it in the assembly file
    for file_path in file_paths {
        let mut data = String::new();
        let _ = File::open(&file_path)
            .unwrap()
            .read_to_string(&mut data)
            .unwrap();

        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        let mut translator = Translator::new(file_name.to_string());

        for (i, line) in data.lines().enumerate() {
            let content = line.chars().collect::<Vec<char>>();
            let mut parser = Parser::new(&content);
            let next_comm = parser
                .next_command()
                .map_err(|err| format!("Error occurred in file {file_name} at line {i} : {err}"))
                .unwrap();

            if let Some(comm) = next_comm {
                let stack_command = format!("// {:?}\n", comm);
                let assembly_instruction =
                    format!("{}\n\n", translator.translate_command(comm).join("\n"));

                file.write_all(stack_command.as_bytes()).unwrap();
                file.write_all(assembly_instruction.as_bytes()).unwrap();
            }
        }
    }
}
