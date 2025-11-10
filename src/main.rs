#![allow(dead_code, unused)]
mod cli;
mod db;
mod interpreter;
mod kv_value;
mod parser;

use std::net::SocketAddr;
use std::sync::Arc;
use std::{env, io};

use smol::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use smol::net::{TcpListener, TcpStream};
use smol::stream::StreamExt;

use crate::cli::Cli;
use crate::interpreter::interpret_tokens;
use crate::kv_value::*;
use crate::{db::DB, parser::*};

async fn handle_client(stream: TcpStream, addr: SocketAddr, db: Arc<DB>) -> io::Result<()> {
    let reader = BufReader::new(stream.clone());
    let mut writer = stream;
    let mut lines = reader.lines();

    while let Some(line) = lines.next().await {
        let line = line?;

        let tokens = parse_string(line);
        let response = interpret_tokens(&db, tokens);

        writer.write_all(response.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let db = Arc::new(DB::new());

    if (args.len() > 1 && args[1] == "--cli") {
        let mut cli = Cli::new(db.as_ref());
        cli.repl();
    } else {
        // TODO: Move this and handle client to different file,
        smol::block_on(async {
            let listener = TcpListener::bind("127.0.0.1:4242").await.unwrap();
            println!("Server started at 127.0.0.1:4242");

            loop {
                let (stream, addr) = listener.accept().await.unwrap();
                println!("New connection from {}", addr);

                let db = db.clone();
                smol::spawn(async move {
                    if let Err(e) = handle_client(stream, addr, db).await {
                        eprintln!("Error handling client {}: {}", addr, e);
                    }
                })
                .detach();
            }
        })
    }
}
