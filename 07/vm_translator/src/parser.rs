use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Parser {
    file: BufReader<File>,
    texts: Vec<String>,
}

#[allow(dead_code)]
enum CommandType {
    CArithmetic,
    CPush,
    CPop,
    CLabel,
    CGoto,
    CIf,
    CFunction,
    CReturn,
    CCall,
    Invalid,
}

const ARITHMETIC_COMMANDS: [&str; 9] = ["add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not"];

impl Parser {
    pub fn new(file: BufReader<File>) -> Parser {
        Parser {
            file: file,
            texts: vec!["".to_string(); 0],
        }
    }
    pub fn parse(&mut self) {
        self.set_texts();
        for command in &self.texts {
            let c_type = self.command_type(command);
            match c_type {
                CommandType::CArithmetic => {}
                CommandType::CPush => {
                    println!("push: {}", command)
                }
                CommandType::CPop => println!("pop: {}", command),
                _ => println!("else: {}", command),
            }
        }
    }
    // コメント行、空行、行頭と行末の空白の削除を行う
    fn set_texts(&mut self) {
        let file: &mut BufReader<File> = &mut self.file;
        for line in file.lines() {
            let mut line_str: String = line.unwrap().to_string();
            line_str = match line_str.find("//") {
                Some(n) => (&line_str[0..n]).to_string(),
                None => line_str,
            };
            if line_str.len() == 0 {
                continue;
            }
            if line_str.starts_with("//") {
                continue;
            }
            line_str = line_str.trim().to_string();
            self.texts.push(line_str);
        }
    }
    fn command_type(&self, command: &str) -> CommandType {
        match command {
            v if ARITHMETIC_COMMANDS.iter().any(|ac| v.starts_with(ac)) => CommandType::CArithmetic,
            v if v.starts_with("push") => CommandType::CPush,
            v if v.starts_with("pop") => CommandType::CPop,
            _ => CommandType::Invalid,
        }
    }
    // fn arg_first(&self, command: &str, command_type: CommandType) -> &str {
    //     let args: Vec<&str> = command.split(" ").collect();
    //     match command_type {
    //         CommandType::CArithmetic => args[0],
    //         _ => args[1],
    //     }
    // }
    fn arg_second(&self, command: &str) -> &str {
        let args: Vec<&str> = command.split(" ").collect();
        args[2]
    }
}