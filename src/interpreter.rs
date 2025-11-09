use crate::cli::Cli;
use crate::kv_value::*;
use crate::{db::DB, parser::*};

// SET "ahmet" 42
// SET <string> <KVValue>
// GET <string>
// DEL <string>
// ADD <string>
// ADD <string> <Number>
pub fn interpret_tokens(db: &mut DB, tokens: Vec<Token>) {
    match tokens.get(0) {
        None => {
            println!("ERROR: Command is empty");
            return;
        }
        Some(Token::Value(_)) => {
            println!("ERROR: First token cannot be a Value");
            return;
        }
        Some(Token::Commands(Commands::GET)) => {
            if let Some(Token::Value(KVValue::STRING(s))) = tokens.get(1) {
                // TODO: print the value, not the data structure (eg. 1 instead of Token(Number(1)))
                // Kayra
                println!("= {:?}", db.get(s));
            } else {
                println!("Usage: GET <String>")
            }
        }
        Some(Token::Commands(Commands::SET)) => {
            if let Some(Token::Value(KVValue::STRING(s))) = tokens.get(1)
                && let Some(Token::Value(v)) = tokens.get(2)
            {
                db.insert(s.clone(), v.clone());
                println!("OK");
            } else {
                println!("Usage: SET <String> <Value>")
            }
        }
        Some(Token::Commands(Commands::DEL)) => {
            if tokens.len() > 1 {
                match &tokens[1] {
                    Token::Value(KVValue::STRING(key)) => {
                        db.remove(key);
                        println!("Deleted key: {}", key);
                    }
                    _ => println!("Error: Expected a string value"),
                }
            } else {
                println!("Usage: DEL <key>");
            }
        }
        Some(Token::Commands(Commands::ADD)) => {
            if let Some(Token::Value(KVValue::STRING(s))) = tokens.get(1) {
                let n;
                match tokens.get(2) {
                    Some(Token::Value(KVValue::NUMBER(num))) => n = *num,
                    _ => n = 1,
                };
                if let Some(KVValue::NUMBER(num)) = db.get(s) {
                    db.insert(s.clone(), KVValue::NUMBER(num + n));
                    println!("OK");
                }
            } else {
                println!("Usage: ADD <String> [Number]")
            }
        }
        Some(Token::Commands(Commands::SUB)) => {
            if let Some(Token::Value(KVValue::STRING(s))) = tokens.get(1) {
                let n = match tokens.get(2) {
                    Some(Token::Value(KVValue::NUMBER(num))) => *num,
                    _ => 1,
                };
                if let Some(KVValue::NUMBER(num)) = db.get(s) {
                    db.insert(s.clone(), KVValue::NUMBER(num - n));
                    println!("OK");
                }
            } else {
                println!("Usage: SUB <String> [Number]")
            }
        }
    }
}

