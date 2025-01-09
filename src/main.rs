use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};
use tokio::sync::Semaphore;
use std::error::Error;
use tokio::io::copy_bidirectional;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    source: String,

    #[arg(short, long)]
    destination: String,
}

// Use smaller buffer size - 4KB is often sufficient for HTTP
const BUFFER_SIZE: usize = 4096;

async fn handle_connection(
    mut client_conn: tokio::net::TcpStream,
    destination: &str,
) {
    // Stack-allocated buffer instead of heap
    let mut buffer = [0u8; BUFFER_SIZE];
    
    // Set socket options to minimize memory
    if let Err(e) = client_conn.set_nodelay(true) {
        eprintln!("Failed to set nodelay: {}", e);
    }

    let n = match client_conn.read(&mut buffer).await {
        Ok(0) => return,
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to read from client: {}", e);
            return;
        }
    };

    let mut server_conn = match tokio::net::TcpStream::connect(destination).await {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to connect to target server: {}", e);
            return;
        }
    };

    // Set socket options for server connection too
    if let Err(e) = server_conn.set_nodelay(true) {
        eprintln!("Failed to set nodelay: {}", e);
    }

    if let Err(e) = server_conn.write_all(&buffer[..n]).await {
        eprintln!("Failed to write to server: {}", e);
        return;
    }

    match copy_bidirectional(&mut client_conn, &mut server_conn).await {
        Ok(_) => (),  // Don't allocate strings for logging
        Err(e) => eprintln!("Error in copy: {}", e),
    }
}

static MAX_CONNECTIONS: usize = 100;
static CONNECTION_SEMAPHORE: Semaphore = Semaphore::const_new(MAX_CONNECTIONS);


#[tokio::main(flavor = "current_thread")]  // Single-threaded runtime
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let listener = TcpListener::bind(&args.source).await?;

    while let Ok((client_conn, _)) = listener.accept().await {
        // Pass destination as reference to avoid cloning
        let dest_clone = args.destination.clone();
        
        let permit = CONNECTION_SEMAPHORE.acquire().await?;

        tokio::spawn(async move {
            handle_connection(client_conn, &dest_clone).await;
            drop(permit);
        });
    }

    Ok(())
}