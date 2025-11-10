#![allow(dead_code, unused)]
use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::{db::DB, interpreter::interpret_tokens, parser::parse_string};

pub struct Cli<'a> {
    db: &'a DB,
}

impl<'a> Cli<'a> {
    pub fn new(db: &'a DB) -> Self {
        Cli { db }
    }

    pub fn new_line(&self) -> Option<String> {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("\nGoodbye (end of file detected)");
                return None;
            }
            Err(e) => {
                eprintln!("Error : {}", e);
                return None;
            }
            Ok(_) => (),
        }
        Some(input)
    }

    pub fn repl(&mut self) {
        loop {
            print!(">> ");
            io::stdout().flush().expect("Failed to flush stdout");

            match self.new_line() {
                Some(val) if val.trim_start().starts_with('.') => {
                    // TODO: make a function that handles .quit ant other cli commands.
                    break;
                }
                Some(input) => {
                    let tokens = parse_string(input);
                    println!("TOKENS = {:?}", tokens);
                    interpret_tokens(&mut self.db, tokens);
                }
                None => break,
            }
        }
    }
}
