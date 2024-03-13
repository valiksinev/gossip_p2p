use std::{
    collections::HashSet,
    net::SocketAddrV4,
    sync::{Arc, Mutex},
    time::Duration,
};
use std::fmt::Debug;
use tokio::net::UdpSocket;

use log::{error, info, Log, warn};


type Hosts_shared = Arc<Mutex<HashSet<SocketAddrV4>>>;

pub async  fn listen (sock: Arc<UdpSocket>, hosts: Hosts_shared) {
    loop {
        let mut buf= vec![];

        match sock.recv_buf_from(&mut buf).await {
            Ok((_size, addr)) => info!("Received message [random message] from {}", addr),
            Err(e) => {
                error!("error to receive messasge: {}", e);
                continue
            },
        }

        let mut lock  = match hosts.lock() {
            Ok(lock) => lock,
            Err(e) => {
                error!("mutex.lock() error: {}", e);
                continue
            }
        };

        let new_hosts = match serde_json::from_slice::<HashSet<SocketAddrV4> >(&buf) {
            Ok(a) => a,
            Err(e) => {
                error!("error to deserialize messsage: {}, binary message: {:?}", e, buf);
                continue
            }
        };
        lock.extend(&new_hosts);
    }
}
pub async fn sender(sock: Arc<UdpSocket>, hosts: Hosts_shared, sec : u64, local: SocketAddrV4) {
    let mut interval = tokio::time::interval(Duration::from_secs(sec));

    loop {
        interval.tick().await;

        let hosts  = match hosts.lock() {
            Ok(lock) => lock.clone(),
            Err(e) => {
                error!("mutex.lock() error: {}", e);
                continue
            }
        };

        let mes = match serde_json::to_string(&hosts) {
            Ok(a) => a,
            Err(e) => {
                error!("error to serialize data: {}, data: {:?}", e, &hosts);
                continue
            }
        };

        let hosts = hosts.iter()
            .filter(|&a| *a != local)
            .collect::<HashSet<&SocketAddrV4>>();

        info!("Sending message to {:?}", hosts);
        for addr in hosts {
            if *addr != local {
                sock.send_to(mes.as_bytes(), addr)
                    .await
                    .unwrap_or_else(error!("error to send message to {}", addr));
            }
        }
    }
}
