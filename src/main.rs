use crate::{parser::*, symbol_table::*};
use code::Code;
use std::{
    env,
    fs::File,
    io::{BufReader, LineWriter, Seek, Write},
};

mod code;
mod parser;
mod symbol_table;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!(
            "Please provide exactly two arguments: an input file name and an output file name."
        );
        std::process::exit(1);
    }

    let input_file_name = &args[1];
    let output_file_name = &args[2];

    let buf_reader = get_buf_reader(input_file_name);
    let mut parser = parser::Parser::new(buf_reader);

    let symbol_table = SymbolTable::new(&mut parser);

    let output_file = File::create(output_file_name).unwrap();
    let mut output_file = LineWriter::new(output_file);

    parser.buffer.rewind().unwrap();
    while parser.advance().is_some() {
        if let Some(CommandType::L_COMMAND) = parser.get_command_type() {
            continue;
        }

        let output = match parser.get_command_type() {
            Some(CommandType::C_COMMAND) => {
                format!(
                    "111{}{}{}",
                    get_comp_binary_string(parser.comp()),
                    get_dest_binary_string(parser.dest()),
                    get_jump_binary_string(parser.jump())
                )
            }
            Some(CommandType::A_COMMAND) => {
                let symbol = parser
                    .get_symbol()
                    .expect("'A' commands should always have a symbol")
                    .trim()
                    .to_string();
                if let Ok(binary) = symbol.parse::<u32>() {
                    format!("0{:015b}", binary)
                } else {
                    let binary = symbol_table.get_binary(&symbol).unwrap();
                    format!("0{}", binary)
                }
            }
            _ => "".to_string(),
        };

        output_file.write_all(output.as_bytes()).unwrap();
        output_file.write_all(b"\n").unwrap();
    }
}

fn get_buf_reader(file_name: &str) -> BufReader<File> {
    let input_file = std::fs::File::open(file_name).unwrap();
    let buf_reader = BufReader::new(input_file);
    buf_reader
}

fn get_dest_binary_string(dest: Option<Destination>) -> String {
    if let Some(dst) = dest {
        dst.to_code()
    } else {
        "000".to_string()
    }
}

fn get_comp_binary_string(comp: Option<Comp>) -> String {
    comp.expect("comp should always be some here").to_code()
}

fn get_jump_binary_string(jump: Option<Jump>) -> String {
    if let Some(jmp) = jump {
        jmp.to_code()
    } else {
        "000".to_string()
    }
}
