use crate::code::Code;
use std::fmt::format;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

pub struct Parser<R: Read> {
    pub buffer: BufReader<R>,
    current_command: Option<String>,
}

impl<R: Read> Parser<R> {
    pub fn new(buffer: BufReader<R>) -> Self {
        Parser {
            buffer,
            current_command: None,
        }
    }

    pub fn advance(&mut self) -> Option<String> {
        loop {
            let res = self.read_line();
            match res {
                None => {
                    return None;
                }
                Some(s) => {
                    if s.starts_with("//") || s == "\r\n" {
                        continue;
                    } else {
                        let v: Vec<String> = s.trim().split("//").map(|x| x.to_string()).collect();
                        let s = v.first().unwrap_or(&s);
                        self.current_command = Some(s.to_string());
                        return Some(s.to_string());
                    }
                }
            }
        }
    }

    fn read_line(&mut self) -> Option<String> {
        let mut s = String::new();
        match self.buffer.read_line(&mut s) {
            Ok(0) => None,
            _ => Some(s),
        }
    }

    pub fn get_command_type(&self) -> Option<CommandType> {
        if let Some(current_command) = &self.current_command {
            match current_command.chars().next().unwrap() {
                '@' => Some(CommandType::A_COMMAND),
                '(' => Some(CommandType::L_COMMAND),
                _ => Some(CommandType::C_COMMAND),
            }
        } else {
            None
        }
    }

    pub fn get_symbol(&self) -> Option<String> {
        match &self.current_command {
            None => None,
            Some(command) => match self.get_command_type() {
                Some(CommandType::A_COMMAND) => Some(command[1..].to_string()),
                Some(CommandType::L_COMMAND) => Some(command[1..command.len() - 1].to_string()),
                _ => None,
            },
        }
    }

    pub fn dest(&self) -> Option<Destination> {
        match self.get_command_type() {
            Some(CommandType::C_COMMAND) => {
                let command = self.current_command.as_ref().unwrap();
                if command.contains('=') {
                    let dest_str = extract_dest(command);
                    Destination::from_str(&dest_str)
                } else {
                    None
                }
            }
            None => None,
            _ => None,
        }
    }

    pub fn comp(&self) -> Option<Comp> {
        match self.get_command_type() {
            Some(CommandType::C_COMMAND) => {
                let command = &self.current_command.as_ref().unwrap();
                let comp_str = extract_comp(command);
                Comp::from_str(&comp_str[..])
            }
            None => None,
            _ => None,
        }
    }

    pub fn jump(&self) -> Option<Jump> {
        match self.get_command_type() {
            Some(CommandType::C_COMMAND) => {
                let command = &self.current_command.as_ref().unwrap();
                if command.contains(';') {
                    let jump_str = extract_jmp(command);
                    Jump::from_str(&jump_str)
                } else {
                    None
                }
            }
            None => None,
            _ => None,
        }
    }
}

fn extract_dest(command: &String) -> String {
    let segments: Vec<&str> = command.split('=').collect();
    segments.first().unwrap().to_string()
}

fn extract_comp(command: &String) -> String {
    if command.contains('=') && command.contains(';') {
        let start_index = command.find('=').unwrap();
        let end_index = command.find(';').unwrap();
        let comp_str = &command[start_index + 1..end_index];
        comp_str.to_string()
    } else if command.contains('=') {
        let segments: Vec<&str> = command.split('=').collect();
        segments.last().unwrap().to_string()
    } else {
        let segments: Vec<&str> = command.split(';').collect();
        segments.first().unwrap().to_string()
    }
}

fn extract_jmp(command: &String) -> String {
    let segments: Vec<&str> = command.split(';').collect();
    segments.last().unwrap().to_string()
}

#[derive(Debug)]
pub enum CommandType {
    A_COMMAND,
    C_COMMAND,
    L_COMMAND,
}

#[derive(Debug)]
pub enum Destination {
    M,
    D,
    MD,
    A,
    AM,
    AD,
    AMD,
}

impl Destination {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "M" => Some(Destination::M),
            "D" => Some(Destination::D),
            "MD" => Some(Destination::MD),
            "A" => Some(Destination::A),
            "AM" => Some(Destination::AM),
            "AD" => Some(Destination::AD),
            "AMD" => Some(Destination::AMD),
            _ => None,
        }
    }
}

impl Code for Destination {
    fn to_code(&self) -> String {
        match self {
            Destination::M => "001".to_string(),
            Destination::D => "010".to_string(),
            Destination::MD => "011".to_string(),
            Destination::A => "100".to_string(),
            Destination::AM => "101".to_string(),
            Destination::AD => "110".to_string(),
            Destination::AMD => "111".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Comp {
    Zero,
    One,
    Minus1,
    D,
    A,
    NotD,
    NotA,
    MinusD,
    MinusA,
    DPlus1,
    APlus1,
    DMinus1,
    AMinus1,
    DPlusA,
    DMinusA,
    AMinusD,
    DAndA,
    DOrA,
    M,
    NotM,
    MinusM,
    MPlus1,
    MMinus1,
    DPlusM,
    DMinusM,
    MMinusD,
    DAndM,
    DOrM,
}

impl Comp {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "0" => Some(Comp::Zero),
            "1" => Some(Comp::One),
            "-1" => Some(Comp::Minus1),
            "D" => Some(Comp::D),
            "A" => Some(Comp::A),
            "!D" => Some(Comp::NotD),
            "!A" => Some(Comp::NotA),
            "-D" => Some(Comp::MinusD),
            "-A" => Some(Comp::MinusA),
            "D+1" => Some(Comp::DPlus1),
            "A+1" => Some(Comp::APlus1),
            "D-1" => Some(Comp::DMinus1),
            "A-1" => Some(Comp::AMinus1),
            "D+A" => Some(Comp::DPlusA),
            "D-A" => Some(Comp::DMinusA),
            "A-D" => Some(Comp::AMinusD),
            "D&A" => Some(Comp::DAndA),
            "D|A" => Some(Comp::DOrA),
            "M" => Some(Comp::M),
            "!M" => Some(Comp::NotM),
            "-M" => Some(Comp::MinusM),
            "M+1" => Some(Comp::MPlus1),
            "M-1" => Some(Comp::MMinus1),
            "D+M" => Some(Comp::DPlusM),
            "D-M" => Some(Comp::DMinusM),
            "M-D" => Some(Comp::MMinusD),
            "D&M" => Some(Comp::DAndM),
            "D|M" => Some(Comp::DOrM),
            v => None,
        }
    }
}

impl Code for Comp {
    fn to_code(&self) -> String {
        match self {
            Comp::Zero => "0101010".to_string(),
            Comp::One => "0111111".to_string(),
            Comp::Minus1 => "0111010".to_string(),
            Comp::D => "0001100".to_string(),
            Comp::A => "0110000".to_string(),
            Comp::NotD => "0001101".to_string(),
            Comp::NotA => "0110001".to_string(),
            Comp::MinusD => "0001111".to_string(),
            Comp::MinusA => "0110011".to_string(),
            Comp::DPlus1 => "0011111".to_string(),
            Comp::APlus1 => "0110111".to_string(),
            Comp::DMinus1 => "0001110".to_string(),
            Comp::AMinus1 => "0110010".to_string(),
            Comp::DPlusA => "0000010".to_string(),
            Comp::DMinusA => "0010011".to_string(),
            Comp::AMinusD => "0000111".to_string(),
            Comp::DAndA => "0000000".to_string(),
            Comp::DOrA => "0010101".to_string(),
            Comp::M => "1110000".to_string(),
            Comp::NotM => "1110001".to_string(),
            Comp::MinusM => "1110011".to_string(),
            Comp::MPlus1 => "1110111".to_string(),
            Comp::MMinus1 => "1110010".to_string(),
            Comp::DPlusM => "1000010".to_string(),
            Comp::DMinusM => "1010011".to_string(),
            Comp::MMinusD => "1000111".to_string(),
            Comp::DAndM => "1000000".to_string(),
            Comp::DOrM => "1010101".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Jump {
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

impl Jump {
    pub fn from_str(s: &str) -> Option<Jump> {
        match s.trim() {
            "JGT" => Some(Jump::JGT),
            "JEQ" => Some(Jump::JEQ),
            "JGE" => Some(Jump::JGE),
            "JLT" => Some(Jump::JLT),
            "JNE" => Some(Jump::JNE),
            "JLE" => Some(Jump::JLE),
            "JMP" => Some(Jump::JMP),
            _ => None,
        }
    }
}

impl Code for Jump {
    fn to_code(&self) -> String {
        match self {
            Jump::JGT => "001".to_string(),
            Jump::JEQ => "010".to_string(),
            Jump::JGE => "011".to_string(),
            Jump::JLT => "100".to_string(),
            Jump::JNE => "101".to_string(),
            Jump::JLE => "110".to_string(),
            Jump::JMP => "111".to_string(),
        }
    }
}

#[test]
#[ignore]
fn can_read_all_lines() {
    let file = File::open("Add.asm").unwrap();
    let buffer = BufReader::new(file);
    let mut my_parser = Parser::new(buffer);
    while let Some(current_line) = my_parser.advance() {
        println!("{:?}", current_line);
    }
}

#[test]
#[ignore]
fn gets_correct_command_type() {
    let file = File::open("Add.asm").unwrap();
    let buffer = BufReader::new(file);
    let mut my_parser = Parser::new(buffer);
    while let Some(current_line) = my_parser.advance() {
        println!("{}: {:?}", current_line, my_parser.get_command_type());
    }
}

#[test]
fn gets_correct_symbol() {
    let file = File::open("Add.asm").unwrap();
    let buffer = BufReader::new(file);
    let mut my_parser = Parser::new(buffer);
    while let Some(current_line) = my_parser.advance() {
        println!(
            "{}:{}",
            current_line,
            my_parser.get_symbol().unwrap_or("0".to_string())
        );
    }
}

#[test]
#[ignore]
fn gets_correct_dest() {
    let file = File::open("Add.asm").unwrap();
    let buffer = BufReader::new(file);
    let mut my_parser = Parser::new(buffer);
    while let Some(current_line) = my_parser.advance() {
        println!("{}: {:?}", current_line, my_parser.dest());
    }
}

#[test]
#[ignore]
fn gets_correct_comp() {
    let file = File::open("Add.asm").unwrap();
    let buffer = BufReader::new(file);
    let mut my_parser = Parser::new(buffer);
    while let Some(current_line) = my_parser.advance() {
        println!(
            "current_line: {} comp: {:?}",
            current_line,
            my_parser.comp()
        );
        println!("");
    }
}

#[test]
#[ignore]
fn gets_correct_jmp() {
    let file = File::open("Add.asm").unwrap();
    let buffer = BufReader::new(file);
    let mut my_parser = Parser::new(buffer);
    while let Some(current_line) = my_parser.advance() {
        println!(
            "current_line: {} jump: {:?}",
            current_line,
            my_parser.jump()
        );
        println!("");
    }
}

#[test]
#[ignore]
fn prints_correct_dest_code() {
    let file = File::open("Add.asm").unwrap();
    let buffer = BufReader::new(file);
    let mut my_parser = Parser::new(buffer);
    while let Some(current_line) = my_parser.advance() {
        if let Some(dst) = my_parser.dest() {
            println!("dest: {:?} code: {}", dst, dst.to_code());
        }
        println!("");
    }
}

#[test]
#[ignore]
fn prints_correct_comp_code() {
    let file = File::open("Add.asm").unwrap();
    let buffer = BufReader::new(file);
    let mut my_parser = Parser::new(buffer);
    while let Some(current_line) = my_parser.advance() {
        if let Some(comp) = my_parser.comp() {
            println!("comp: {:?} code: {}", comp, comp.to_code());
        }
        println!("");
    }
}

#[test]
#[ignore]
fn prints_correct_jmp_code() {
    let file = File::open("Add.asm").unwrap();
    let buffer = BufReader::new(file);
    let mut my_parser = Parser::new(buffer);
    while let Some(current_line) = my_parser.advance() {
        if let Some(jump) = my_parser.jump() {
            println!("jmp: {:?} code: {}", jump, jump.to_code());
        }
        println!("");
    }
}
