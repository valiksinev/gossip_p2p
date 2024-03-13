mod program_option;
mod tasks;

use std::{
    io,
    sync::{Arc, Mutex},
    time::Duration,
    net::{SocketAddr, SocketAddrV4, IpAddr, Ipv4Addr},
    collections::HashSet,
    fmt::Display,
    str::FromStr,
};
use tokio::net::UdpSocket;
use serde::{Deserialize, Serialize};
use tasks::*;
use log::{error, info, Log, warn};
use fast_log::{Config, Logger};


#[tokio::main]
async fn main() -> io::Result<()> {

    let cli = program_option::parse();
    let local = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), cli.port);

    let logger : &'static Logger = fast_log::init(Config::new().console()).unwrap();
    info!("serivice started, local address: {}, period: {} sec", local, cli.period);

    logger.flush();

    let sock =  UdpSocket::bind(SocketAddr::from(local)).await?;

    let hosts = if let Some(addr) = cli.connect {
        HashSet::from([local, addr])
    } else {
        HashSet::from([local])
    };

    let socket_shared = Arc::new(sock);
    let hosts_shared = Arc::new(Mutex::new(hosts));

    let listener_jhandle = tokio::spawn(
        listen(Arc::clone(&socket_shared), Arc::clone(&hosts_shared) )
    );

    let sender_jhandle = tokio::spawn (
        sender(Arc::clone(&socket_shared), Arc::clone(&hosts_shared), cli.period, local)
    );

    listener_jhandle.await;
    sender_jhandle.await;
    Ok(())
}
