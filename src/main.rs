use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};
use std::error::Error;
use tokio::io::copy_bidirectional;
use clap::Parser;

/// Simple HTTP proxy that forwards traffic between source and destination
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Source address to listen on (e.g., "localhost:8080")
    #[arg(short, long)]
    source: String,

    /// Destination address to forward to (e.g., "example.com:80")
    #[arg(short, long)]
    destination: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args = Args::parse();
    
    // Listen on the specified source address
    let listener = TcpListener::bind(&args.source).await?;
    println!("Listening on {}", args.source);

    loop {
        let (mut client_conn, _) = listener.accept().await?;
        
        // Clone destination for the async task
        let destination = args.destination.clone();
        
        // Spawn a new task for each connection
        tokio::spawn(async move {
            let mut buffer = [0; 8192];
            
            // Read the client request
            match client_conn.read(&mut buffer).await {
                Ok(0) => (),
                Ok(n) => {
                    // Connect to the target server
                    match tokio::net::TcpStream::connect(&destination).await {
                        Ok(mut server_conn) => {
                            // Forward the client request to the server
                            if let Err(e) = server_conn.write_all(&buffer[..n]).await {
                                eprintln!("Failed to write to server: {}", e);
                                return;
                            }

                            // Use copy_bidirectional to handle the duplex connection
                            match copy_bidirectional(&mut client_conn, &mut server_conn).await {
                                Ok((from_client, from_server)) => {
                                    println!("Connection closed. Bytes from client: {}, from server: {}", 
                                        from_client, from_server);
                                }
                                Err(e) => eprintln!("Error in bidirectional copy: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Failed to connect to target server: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to read from client: {}", e),
            }
        });
    }
}