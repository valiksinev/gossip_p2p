use std::{
    collections::HashSet,
    net::SocketAddrV4,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{net::UdpSocket, select};
use tokio_util::sync::CancellationToken;
use log::{error, info,};


type HostsShared = Arc<Mutex<HashSet<SocketAddrV4>>>;

pub async  fn listen (
    sock: Arc<UdpSocket>,
    hosts: HostsShared,
    token: CancellationToken
) {
    let recv = || async {
        let mut buf= vec![];

        match sock.recv_buf_from(&mut buf).await {
            Ok((_size, addr)) => info!("Received message from {}", addr),
            Err(e) => {
                error!("error to receive messasge: {}", e);
                return;
            }
        }

        let mut lock  = match hosts.lock() {
            Ok(lock) => lock,
            Err(e) => {
                error!("mutex.lock() error: {}", e);
                return;
            }
        };

        let new_hosts = match serde_json::from_slice::<HashSet<SocketAddrV4> >(&buf) {
            Ok(a) => a,
            Err(e) => {
                error!("error to deserialize messsage: {}, binary message: {:?}", e, buf);
                return;
            }
        };
        lock.extend(&new_hosts);
    };

    loop {
        select! {
            _ = token.cancelled() => {
                    info!("Shutdown listener");
                    break;
                },
            _ = recv() => {}
        }
    }
}
pub async fn sender(
    sock: Arc<UdpSocket>,
    hosts: HostsShared,
    sec : u64,
    local: SocketAddrV4,
    token: CancellationToken
) {
    let mut interval = tokio::time::interval(Duration::from_secs(sec));

    let send = || async {
        let hosts  = match hosts.lock() {
            Ok(lock) => lock.clone(),
            Err(e) => {
                error!("mutex.lock() error: {}", e);
                return;
            }
        };

        let mes = match serde_json::to_string(&hosts) {
            Ok(a) => a,
            Err(e) => {
                error!("error to serialize data: {}, data: {:?}", e, &hosts);
                return;
            }
        };

        let hosts = hosts.iter()
            .filter(|&a| *a != local)
            .collect::<HashSet<&SocketAddrV4>>();

        if !hosts.is_empty() {
            info!("Sending message to {:?}", hosts);
        }
        for addr in hosts {
            if sock.send_to(mes.as_bytes(), addr).await.is_err() {
                error!("error to send message to {}", addr);
            }
        }
    };

    loop {
        select! {
            _ = token.cancelled() => {
                    info!("Shutdown sender");
                    break;
                },
            _ = interval.tick() => send().await
        }
    }
}
