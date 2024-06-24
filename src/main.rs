mod parser;
mod db;

use serde_json::Value;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{interval, Duration};
use parser::{parse_command, RedisCommand};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind the TCP listener to an address
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    println!("Server running on 127.0.0.1:6379");

    let db = Arc::new(Mutex::new(db::Db::new()));


    let db = db.clone();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            let mut db = db.lock().unwrap();
            db.remove_expired();
        }
    });


    loop {
        // Accept new incoming connections
        let (socket, _) = listener.accept().await?;
        let db = db.clone();
        // Spawn a new task for each connection
        tokio::spawn(async move {
            handle_connection(socket, db).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream, db: Arc<Mutex<db::Db>>) {
    let mut buffer = [0; 1024];

    loop {
        let n = match socket.read(&mut buffer).await {
            Ok(n) if n == 0 => return, // Connection closed
            Ok(n) => n,
            Err(e) => {
                println!("Failed to read from socket; err = {:?}", e);
                return;
            }
        };

        let command = parse_command(&buffer[0..n]);
        let response = {
            let mut db = db.lock().unwrap();
            match command {
                RedisCommand::Get(key) => db.get(&key),
                RedisCommand::Set(key, value) => db.set(key, value, Some(Duration::from_secs(10))), // Set TTL to 60 seconds for example
                RedisCommand::Unknown => Err("Unknown command\n".to_string()),
            }
        };

        let response = response.unwrap_or_else(|err| err);

        if socket.write_all(response.as_bytes()).await.is_err() {
            println!("Failed to write to socket");
            return;
        }
    }
}
