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

use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use cita_cloud_proto::health_check::{
    health_check_response::ServingStatus, health_server::Health, HealthCheckRequest,
    HealthCheckResponse,
};
use cloud_util::unix_now;
use parking_lot::RwLock;
use tonic::{Request, Response, Status};

use crate::peer::PeersManger;

// grpc server of Health Check
pub struct HealthCheckServer {
    peers: Arc<RwLock<PeersManger>>,
    timestamp: AtomicU64,
    timeout: u64,
}

impl HealthCheckServer {
    pub fn new(peers: Arc<RwLock<PeersManger>>, timeout: u64) -> Self {
        HealthCheckServer {
            peers,
            timestamp: AtomicU64::new(unix_now()),
            timeout,
        }
    }
}

#[tonic::async_trait]
impl Health for HealthCheckServer {
    async fn check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        let timestamp = unix_now();
        let old_timestamp = self.timestamp.load(Ordering::Relaxed);
        let peer_count;
        {
            peer_count = self.peers.read().get_connected_peers().len() as u64;
        }
        let status = if peer_count > 0 {
            self.timestamp.store(timestamp, Ordering::Relaxed);
            ServingStatus::Serving.into()
        } else {
            // peer is offline for a long time
            if timestamp - old_timestamp > self.timeout * 1000 {
                ServingStatus::NotServing.into()
            } else {
                ServingStatus::Serving.into()
            }
        };

        let reply = Response::new(HealthCheckResponse { status });
        Ok(reply)
    }
}
