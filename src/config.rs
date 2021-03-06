// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cloud_util::common::read_toml;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PeerConfig {
    pub protocol: String,
    pub port: u16,
    pub domain: String,
}

impl PeerConfig {
    pub fn get_address(&self) -> String {
        format!("{}/{}:{}", self.protocol, self.domain, self.port)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModuleConfig {
    pub module_name: String,
    pub hostname: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct NetworkConfig {
    // server grpc port, as network_port
    pub grpc_port: u16,
    // zenoh protocol
    pub protocol: String,
    // zenoh port
    pub port: u16,
    // domain
    pub domain: String,
    // CA certification, raw string
    pub ca_cert: String,
    // Server certification, raw string
    pub cert: String,
    // Server certification private key
    pub priv_key: String,
    // peers net config info
    pub peers: Vec<PeerConfig>,
    // node address
    pub node_address: String,
    // validator address
    pub validator_address: String,
    // chain id
    pub chain_id: String,
    // QoS
    pub qos: bool,
    // local_routing
    pub local_routing: bool,
    // peers_autoconnect
    pub peers_autoconnect: bool,
    // Link lease duration in milliseconds (default: 10000)
    pub lease: u64,
    // Number fo keep-alive messages in a link lease duration (default: 4)
    pub keep_alive: usize,
    // config hot update interval, in senconds
    pub hot_update_interval: u64,
    // modules config info
    pub modules: Vec<ModuleConfig>,
    /// health check timeout
    pub health_check_timeout: u64,
}

impl NetworkConfig {
    pub fn new(config_str: &str) -> Self {
        read_toml(config_str, "network_zenoh")
    }
    pub fn get_address(&self) -> String {
        format!("{}/{}:{}", self.protocol, self.domain, self.port)
    }
    pub fn get_node_origin(&self) -> u64 {
        let tmp = &self.node_address[0..16];
        let mut decoded = [0; 8];
        hex::decode_to_slice(tmp, &mut decoded).unwrap();
        u64::from_be_bytes(decoded)
    }
    pub fn get_validator_origin(&self) -> u64 {
        let tmp = &self.validator_address[0..16];
        let mut decoded = [0; 8];
        hex::decode_to_slice(tmp, &mut decoded).unwrap();
        u64::from_be_bytes(decoded)
    }
    pub fn get_chain_origin(&self) -> u64 {
        let tmp = &self.chain_id[0..16];
        let mut decoded = [0; 8];
        hex::decode_to_slice(tmp, &mut decoded).unwrap();
        u64::from_be_bytes(decoded)
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            grpc_port: 50000,
            protocol: "tls".to_string(),
            domain: "".to_string(),
            port: 40000,
            peers: vec![],
            ca_cert: "".to_string(),
            cert: "".to_string(),
            priv_key: "".to_string(),
            node_address: "".to_string(),
            validator_address: "".to_string(),
            chain_id: "".to_string(),
            qos: true,
            local_routing: false,
            peers_autoconnect: false,
            lease: 10000,
            keep_alive: 4,
            hot_update_interval: 60,
            modules: vec![],
            health_check_timeout: 300,
        }
    }
}
