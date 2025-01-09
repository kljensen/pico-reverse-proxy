use clap::Parser;
use std::error::Error;
use tokio::io::copy_bidirectional;
use tokio::net::TcpListener;
use tokio::sync::Semaphore;
use tokio::time::{timeout, Duration};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    source: String,

    #[arg(short, long)]
    destination: String,
}

async fn connect_with_timeout(destination: &str, timeout_duration: Duration) -> Option<tokio::net::TcpStream> {
    match timeout(timeout_duration, tokio::net::TcpStream::connect(destination)).await {
        Ok(connect_result) => match connect_result {
            Ok(conn) => Some(conn),
            Err(e) => {
                eprintln!("Connection error: {}", e);
                None
            }
        },
        Err(_) => {
            eprintln!("Connection timed out");
            None
        }
    }
}

async fn handle_connection(mut client_conn: tokio::net::TcpStream, destination: &str) {
    // Set socket options to minimize memory
    if let Err(e) = client_conn.set_nodelay(true) {
        eprintln!("Failed to set nodelay: {}", e);
    }

    if client_conn.readable().await.is_err() {
        eprintln!("Failed to read from client connection");
        return;
    }
    let timeout_duration = Duration::from_secs(5);
    let mut server_conn = match connect_with_timeout(destination, timeout_duration).await {
        Some(conn) => conn,
        None => return,
    };

    // Set socket options for server connection too
    if let Err(e) = server_conn.set_nodelay(true) {
        eprintln!("Failed to set nodelay: {}", e);
    }

    // Use splice if available on Linux systems
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::io::AsRawFd;
        if let Err(e) =
            tokio::io::copy_bidirectional_zero_copy(&mut client_conn, &mut server_conn).await
        {
            eprintln!("Error in zero-copy: {}", e);
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        if let Err(e) = copy_bidirectional(&mut client_conn, &mut server_conn).await {
            eprintln!("Error in copy: {}", e);
        }
    }
}

static MAX_CONNECTIONS: usize = 100;
static CONNECTION_SEMAPHORE: Semaphore = Semaphore::const_new(MAX_CONNECTIONS);

#[tokio::main(flavor = "current_thread")] // Single-threaded runtime
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
