use std::net::{IpAddr, SocketAddr};
use tokio::net::TcpStream;
use tokio::time::{Duration, timeout};
use clap::{App, Arg};

const TIMEOUT_SECONDS: u64 = 5;

async fn scan_port(ip: IpAddr, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = SocketAddr::new(ip, port);
    let timeout_duration = Duration::from_secs(TIMEOUT_SECONDS);

    match timeout(timeout_duration, TcpStream::connect(socket_addr)).await {
        Ok(Ok(_)) => {
            println!("Port {} is open", port);
        }
        _ => {}
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Port Scanner")
        .version("1.0")
        .author("Your Name")
        .about("Scans ports on a target IP address")
        .arg(
            Arg::with_name("target_ip")
                .help("Target IP address to scan")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("start_port")
                .help("Starting port number")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("end_port")
                .help("Ending port number")
                .required(true)
                .index(3),
        )
        .get_matches();

    let target_ip = matches
        .value_of("target_ip")
        .unwrap()
        .parse::<IpAddr>()?;
    let start_port = matches
        .value_of("start_port")
        .unwrap()
        .parse::<u16>()?;
    let end_port = matches
        .value_of("end_port")
        .unwrap()
        .parse::<u16>()?;

    println!("Scanning ports on {}...", target_ip);

    for port in start_port..=end_port {
        scan_port(target_ip, port).await?;
    }

    Ok(())
}
