use crate::cli::Cli;
use crate::kv_value::*;
use crate::{db::DB, parser::*};

// SET "ahmet" 42
// SET <string> <KVValue>
// GET <string>
// DEL <string>
// ADD <string>
// ADD <string> <Number>
// TODO: Return a Result.
pub fn interpret_tokens(db: &DB, tokens: Vec<Token>) -> String {
    match tokens.get(0) {
        None => {
            return "ERROR: Command is empty".to_string();
        }
        Some(Token::Value(_)) => {
            return "ERROR: First token cannot be a Value".to_string();
        }
        Some(Token::Commands(Commands::GET)) => {
            if let Some(Token::Value(KVValue::STRING(s))) = tokens.get(1) {
                // TODO: print the value, not the data structure (eg. 1 instead of Token(Number(1)))
                // Kayra
                return format!("= {:?}", db.get(s));
            } else {
                return "Usage: GET <String>".to_string();
            }
        }
        Some(Token::Commands(Commands::SET)) => {
            if let Some(Token::Value(KVValue::STRING(s))) = tokens.get(1)
                && let Some(Token::Value(v)) = tokens.get(2)
            {
                db.insert(s.clone(), v.clone());
                return "OK".to_string();
            } else {
                return "Usage: SET <String> <Value>".to_string();
            }
        }
        Some(Token::Commands(Commands::DEL)) => {
            if tokens.len() > 1 {
                match &tokens[1] {
                    Token::Value(KVValue::STRING(key)) => {
                        db.remove(key);
                        return format!("Deleted key: {}", key);
                    }
                    _ => return "Error: Expected a string value".to_string(),
                }
            } else {
                return "Usage: DEL <key>".to_string();
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
                    return "OK".to_string();
                } else {
                    return "Usage: ADD <String> [Number]".to_string();
                }
            } else {
                return "Usage: ADD <String> [Number]".to_string();
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
                    return "OK".to_string();
                } else {
                    return "Usage: SUB <String> [Number]".to_string();
                }
            } else {
                return "Usage: SUB <String> [Number]".to_string();
            }
        }
    }
}
