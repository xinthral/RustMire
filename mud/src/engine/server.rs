use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::broadcast;
use std::fs;

pub fn read_in_ascii_logo(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
  let contents = fs::read_to_string(filename)?;
  Ok(contents)
}

pub async fn parse_command(line: &str) -> Option<&str> {
  if line.trim().is_empty() {
    return None;
  }
  let parts: Vec<&str> = line.splitn(2, " ").collect();
  if parts.len() < 2 {
    return None;
  }
  Some(parts[1])
}

#[tokio::main]
pub async fn launch_server() -> Result<(), Box<dyn std::error::Error>> {
  let listener = TcpListener::bind("tardis.it.mtu.edu:1234").await?;
  println!("Server running on port 1234. Connect with: telnet localhost 1234");

  let (shutdown_tx, _) = broadcast::channel::<()>(1);

  loop {
    tokio::select! {
      Ok((mut socket, _)) = listener.accept() => {
        let mut shutdown_rx = shutdown_tx.subscribe();
        let shutdown_tx = shutdown_tx.clone();
        let logo = read_in_ascii_logo("media/ascii_logo.txt").unwrap();

        tokio::spawn(async move {
          let (reader, mut writer) = socket.split();
          let mut reader = BufReader::new(reader);
          let mut line = String::new();

          writer.write_all(logo.as_bytes()).await.unwrap();
          writer.write_all(b"Welcome to the game!\nType 'help' or 'quit'\n> ").await.unwrap();

          loop {
            tokio::select! {
              _ = shutdown_rx.recv() => {
                let _ = writer.write_all(b"\nServer is shutting down...\n").await;
                break;
              }
              result = reader.read_line(&mut line) => {
                if result.unwrap_or(0) == 0 {
                  break;
                }

                match line.trim() {
                  "help" => {
                    writer.write_all(b"Available commands: help, quit\n> ").await.unwrap();
                  }
                  "quit" => {
                    writer.write_all(b"Goodbye!\n").await.unwrap();
                    let _ = shutdown_tx.send(()); // signal shutdown
                    break;
                  }
                  _ => {
                    // Simulate a response
                    let response = parse_command(&line).await;
                    if let Some(response) = response {
                      let output = format!("{}\n> ", response);
                      writer.write_all(output.as_bytes()).await.unwrap();
                    }
                  }
                }
                line.clear();
              }
            }
          }
        });
      }
    }
  }
}
