use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};
use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on 127.0.0.1:7878");

    for stream_result in listener.incoming() {
        let stream = match stream_result {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
                continue;
            }
        };

        rt.spawn_blocking(move || {
            if let Err(e) = handle_connection(stream) {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }

    Ok(())
}

#[allow(dead_code)]
fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let msg = "ajay";
    let mut buffer = [0; 512];

    loop {
        let bytes_read = stream.read(&mut buffer)?;

        if bytes_read == 0 {
            println!("Client disconnected.");
            return Ok(());
        }

        let received_data = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received: {}", received_data);

        stream.write_all(msg.as_bytes())?;
        println!("Sent: {}", msg);
    }
}
