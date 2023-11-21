use std::{collections::HashMap, error::Error, fs::File, io::Seek};

use crate::parser::{CommandType, Parser};

pub struct SymbolTable {
    hash_map: HashMap<String, String>,
}

impl SymbolTable {
    pub fn new(parser: &mut Parser<File>) -> Self {
        let mut symbol_table = SymbolTable {
            hash_map: HashMap::new(),
        };
        symbol_table.add_predefined_symbols();
        symbol_table.first_pass(parser);
        parser.buffer.rewind().unwrap();
        symbol_table.second_pass(parser);
        // println!("{:?}", symbol_table.hash_map);
        symbol_table
    }

    fn add_predefined_symbols(&mut self) {
        self.hash_map
            .insert("SP".to_string(), format!("{:015b}", 0));
        self.hash_map
            .insert("LCL".to_string(), format!("{:015b}", 1));
        self.hash_map
            .insert("ARG".to_string(), format!("{:015b}", 2));
        self.hash_map
            .insert("THIS".to_string(), format!("{:015b}", 3));
        self.hash_map
            .insert("THAT".to_string(), format!("{:015b}", 4));
        self.hash_map
            .insert("R0".to_string(), format!("{:015b}", 0));
        self.hash_map
            .insert("R1".to_string(), format!("{:015b}", 1));
        self.hash_map
            .insert("R2".to_string(), format!("{:015b}", 2));
        self.hash_map
            .insert("R3".to_string(), format!("{:015b}", 3));
        self.hash_map
            .insert("R4".to_string(), format!("{:015b}", 4));
        self.hash_map
            .insert("R5".to_string(), format!("{:015b}", 5));
        self.hash_map
            .insert("R6".to_string(), format!("{:015b}", 6));
        self.hash_map
            .insert("R7".to_string(), format!("{:015b}", 7));
        self.hash_map
            .insert("R8".to_string(), format!("{:015b}", 8));
        self.hash_map
            .insert("R9".to_string(), format!("{:015b}", 9));
        self.hash_map
            .insert("R10".to_string(), format!("{:015b}", 10));
        self.hash_map
            .insert("R11".to_string(), format!("{:015b}", 11));
        self.hash_map
            .insert("R12".to_string(), format!("{:015b}", 12));
        self.hash_map
            .insert("R13".to_string(), format!("{:015b}", 13));
        self.hash_map
            .insert("R14".to_string(), format!("{:015b}", 14));
        self.hash_map
            .insert("R15".to_string(), format!("{:015b}", 15));
        self.hash_map
            .insert("SCREEN".to_string(), format!("{:015b}", 16384));
        self.hash_map
            .insert("KBD".to_string(), format!("{:015b}", 24576));
    }

    fn first_pass(&mut self, parser: &mut Parser<File>) {
        let mut current_address = -1;
        while parser.advance().is_some() {
            match parser.get_command_type() {
                Some(CommandType::L_COMMAND) => {
                    self.hash_map.insert(
                        parser.get_symbol().unwrap(),
                        format!("{:015b}", current_address + 1),
                    );
                }
                _ => {
                    current_address += 1;
                }
            }
        }
    }

    fn second_pass(&mut self, parser: &mut Parser<File>) {
        let mut next_available_address = 16;
        while parser.advance().is_some() {
            if let Some(CommandType::A_COMMAND) = parser.get_command_type() {
                let symbol = parser.get_symbol().unwrap();
                if symbol.parse::<u32>().is_err() && self.hash_map.get(&symbol).is_none() {
                    // println!("Processing symbol {}", symbol);
                    self.hash_map
                        .insert(symbol, format!("{:015b}", next_available_address));
                    next_available_address += 1;
                }
            }
        }
    }

    pub fn get_binary(&self, symbol: &str) -> Option<String> {
        self.hash_map.get(symbol).cloned()
    }
}
