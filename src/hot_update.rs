use std::sync::Arc;

use log::{debug, info};
use parking_lot::RwLock;
use zenoh::{
    config::{Config, Notifier},
    prelude::ValidatedMap,
};

use crate::{
    config::{NetworkConfig, PeerConfig},
    peer::PeersManger,
};

pub async fn try_hot_update(
    path: &str,
    peers: Arc<RwLock<PeersManger>>,
    zenoh_config: &mut Notifier<Config>,
) {
    let new_config = NetworkConfig::new(path);
    let known_peers = peers
        .read()
        .get_known_peers()
        .iter()
        .map(|(s, _)| s.to_owned())
        .collect::<Vec<String>>();
    debug!("known peers: {:?}", known_peers);
    let new_peers = new_config
        .peers
        .iter()
        .map(|p| p.domain.to_owned())
        .collect::<Vec<String>>();
    debug!("peers in config file: {:?}", new_peers);
    //try to add node
    let mut connect_peers = Vec::new();
    for p in new_config.peers {
        let peer = PeerConfig {
            protocol: "quic".to_string(),
            port: p.port,
            domain: p.domain.clone(),
        };

        connect_peers.push(peer.get_address());

        if !known_peers.contains(&p.domain) {
            let mut guard = peers.write();
            info!("peer added: {}", &peer.get_address());
            guard.add_known_peers(p.domain.clone(), peer);
        }
    }
    // update zenoh nodes to connect to
    zenoh_config
        .insert_json5(
            "connect/endpoints",
            &json5::to_string(&connect_peers).unwrap(),
        )
        .unwrap();
    //try to delete node
    for p in known_peers {
        if !new_peers.contains(&p) {
            let mut guard = peers.write();
            guard.delete_peer(p.as_str());
            info!("peer deleted: {}", p);
        }
    }
}