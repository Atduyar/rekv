#![allow(dead_code, unused)]
mod cli;
mod db;
mod kv_value;
mod parser;
mod interpreter;

use crate::cli::Cli;
use crate::kv_value::*;
use crate::{db::DB, parser::*};

fn main() {
    let mut db = DB::new();

    let mut cli = Cli::new(db);
    cli.repl();
}
