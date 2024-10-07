use std::net::{TcpStream};
use std::time::Duration;
use std::io::{self};

fn scan_port(ip: &str, port: u16) -> bool {
    // Set a connection timeout of 3 seconds
    let timeout = Duration::from_secs(3);

    match TcpStream::connect_timeout(&format!("{}:{}", ip, port).parse().unwrap(), timeout) {
        Ok(_) => true,  // If connection is successful, port is open
        Err(_) => false, // If connection fails, port is closed/filtered
    }
}

fn main() -> io::Result<()> {
    let ip = "127.0.0.1"; // Target IP
    let start_port = 1;
    let end_port = 1024;

    println!("Starting port scan on {}...", ip);

    for port in start_port..=end_port {
        if scan_port(ip, port) {
            println!("Port {} is open", port);
        }
    }

    println!("Scan complete.");
    Ok(())
}
