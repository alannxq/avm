use std::{
    self, 
    io::{
        stdout,
        stdin, 
        Write
    },
    num::{
        ParseIntError
    }
};

use nom::types::CompleteStr;

use crate::vm::VM;
use crate::assembler::program_parsers::program; 

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM
}

impl REPL {
    pub fn new() -> Self {
        REPL {
            command_buffer: vec![],
            vm: VM::new()
        }
    }

    fn parse_hex(&mut self, str_hex: &str) -> Result<Vec<u8>, ParseIntError> {
        let split: Vec<&str> = str_hex.split(" ").collect();
        let mut results = Vec::<u8>::new();
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                },
                Err(e) => {
                    return Err(e)
                }
            }
        }
        Ok(results)
    }

    pub fn run(&mut self) {
        println!("Welcome to the AVM REPL");
        loop {
            let mut buffer = String::new();

            print!(">>> ");
            stdout().flush().expect("Could not flush stdout");

            stdin().read_line(&mut buffer).expect("Could not read line from stdin");
            let buffer = buffer.trim();

            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".quit" => {
                    println!("Goodbye!");
                    std::process::exit(0);
                },
                ".history" => {
                    for command in &self.command_buffer {
                        println!("- {command}");
                    }
                },
                ".program" => {
                    println!("Listing instructions currently in the VM's program vector");
                    for instruction in &self.vm.program {
                        println!("{instruction}")
                    }
                    println!("End of instructions")
                },
                ".registers" => {
                    println!("Listing the contents of the VM's registers");
                    println!("{:#?}", self.vm.registers);
                    println!("End of registers");
                },
                _ => {
                    let parsed_program = program(CompleteStr(buffer));
                    if !parsed_program.is_ok() {
                        println!("Unable to parse input");
                        continue;
                    }
                    let (_, result) = parsed_program.unwrap();
                    let bytecode = result.to_bytes();
                    for byte in bytecode {
                        self.vm.add_byte(byte);
                    }
                    self.vm.run_once();
                }
            }
        }
    
    }
}