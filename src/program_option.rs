use clap::Parser;
use std::net::SocketAddrV4;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(about = "a simple p2p gossiping application", long_about = None) ]
pub struct Cli {
    /// period for broadcasting messages in seconds
    #[arg(long)]
    pub period: u64,

    /// port of this service
    #[arg(long)]
    pub port: u16,

    /// address to connect, format: 127.0.0.1:8080
    #[arg(short, long)]
    pub connect: Option<SocketAddrV4>,
}
pub fn parse() -> Cli {
    Cli::parse()
}