mod program_option;
mod tasks;

use std::{
    io,
    sync::{Arc, Mutex},
    net::{SocketAddr, SocketAddrV4, Ipv4Addr},
    collections::HashSet,
};
use tokio::{net::UdpSocket, signal,};
use tokio_util::sync::CancellationToken;
use tasks::*;
use log::{error, info, Log};
use fast_log::{Config, Logger};


#[tokio::main]
async fn main() -> io::Result<()> {

    let cli = program_option::parse();
    let local = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), cli.port);

    let logger : &'static Logger = fast_log::init(Config::new().console())
        .expect("log init error");
    info!("Start service: local address {}, period {} sec", local, cli.period);

    logger.flush();

    let sock =  UdpSocket::bind(SocketAddr::from(local)).await?;

    let hosts = if let Some(addr) = cli.connect {
        HashSet::from([local, addr])
    } else {
        HashSet::from([local])
    };

    let token = CancellationToken::new();

    let socket_shared = Arc::new(sock);
    let hosts_shared = Arc::new(Mutex::new(hosts));

    let listener_jhandle = tokio::spawn(
        listen(Arc::clone(&socket_shared), Arc::clone(&hosts_shared), token.clone())
    );

    let sender_jhandle = tokio::spawn (
        sender(Arc::clone(&socket_shared), Arc::clone(&hosts_shared), cli.period, local, token.clone())
    );

    match signal::ctrl_c().await {
        Ok(()) => {
            info!("Shutdown..");
            token.cancel();
        },
        Err(e) => error!("Unable to listen for shutdown signal: {}", e),
    };

    let _ = listener_jhandle.await;
    let _ = sender_jhandle.await;
    Ok(())
}
