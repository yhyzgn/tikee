//! `TiKV` raft-rs bootstrap integration for future server consensus.
//!
//! This module intentionally validates the crate/config/storage boundary only. It does
//! not start a Raft event loop, campaign, or grant scheduler ownership.

use std::collections::BTreeSet;

use raft::{Config, StateRole, raw_node::RawNode, storage::MemStorage};
use scheduler_config::ClusterConfig;
use sha2::{Digest, Sha256};

/// Crate-level runtime library label exposed in diagnostics and design docs.
pub const RAFT_RS_LIBRARY: &str = "tikv/raft-rs crate raft 0.7.0";

/// Safe bootstrap status produced by constructing a raft-rs `RawNode` without driving it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RaftRsBootstrapStatus {
    /// Scheduler string node id from config.
    pub node_id: String,
    /// Deterministic raft-rs numeric node id derived from `node_id`.
    pub raft_node_id: u64,
    /// Numeric voter ids derived from configured peers.
    pub voter_ids: Vec<u64>,
    /// Initial raft-rs role after construction; must not be treated as scheduler ownership.
    pub initial_role: String,
    /// Whether a freshly constructed node reports ready work before the event loop starts.
    pub has_ready: bool,
}

/// Validate that the current cluster config can construct a raft-rs `RawNode`.
///
/// # Errors
///
/// Returns a human-readable error when the raft-rs config or bootstrap storage is invalid.
pub fn validate_raft_rs_bootstrap(config: &ClusterConfig) -> Result<RaftRsBootstrapStatus, String> {
    let raft_node_id = raft_numeric_id(&config.node_id);
    let voter_ids = voter_ids(config, raft_node_id)?;
    let mut raft_config = Config::new(raft_node_id);
    raft_config.heartbeat_tick = 2;
    raft_config.election_tick = 20;
    raft_config.check_quorum = true;
    raft_config.pre_vote = true;
    raft_config
        .validate()
        .map_err(|error| format!("raft-rs config invalid: {error}"))?;

    let storage = MemStorage::new_with_conf_state((voter_ids.clone(), Vec::new()));
    let node = RawNode::with_default_logger(&raft_config, storage)
        .map_err(|error| format!("raft-rs RawNode bootstrap failed: {error}"))?;
    let status = node.status();
    let initial_role = raft_role_name(status.ss.raft_state).to_owned();
    let has_ready = node.has_ready();

    Ok(RaftRsBootstrapStatus {
        node_id: config.node_id.clone(),
        raft_node_id,
        voter_ids,
        initial_role,
        has_ready,
    })
}

/// Deterministically map existing string node ids to raft-rs `u64` node ids.
#[must_use]
pub fn raft_numeric_id(node_id: &str) -> u64 {
    let digest = Sha256::digest(node_id.as_bytes());
    let mut bytes = [0_u8; 8];
    bytes.copy_from_slice(&digest[..8]);
    let id = u64::from_be_bytes(bytes);
    if id == 0 { 1 } else { id }
}

fn voter_ids(config: &ClusterConfig, local_id: u64) -> Result<Vec<u64>, String> {
    let mut voters = BTreeSet::from([local_id]);
    for peer in &config.peers {
        let peer_id = raft_numeric_id(&peer.node_id);
        if !voters.insert(peer_id) && peer.node_id != config.node_id {
            return Err(format!(
                "raft-rs numeric node id collision for configured peer {}",
                peer.node_id
            ));
        }
    }
    Ok(voters.into_iter().collect())
}

const fn raft_role_name(role: StateRole) -> &'static str {
    match role {
        StateRole::Follower => "follower",
        StateRole::Candidate => "candidate",
        StateRole::Leader => "leader",
        StateRole::PreCandidate => "pre_candidate",
    }
}

#[cfg(test)]
mod tests {
    use scheduler_config::{ClusterConfig, ClusterModeConfig, ClusterPeerConfig};

    use super::{raft_numeric_id, validate_raft_rs_bootstrap};

    #[test]
    fn raft_numeric_id_is_stable_non_zero() {
        let first = raft_numeric_id("scheduler-0");
        let second = raft_numeric_id("scheduler-0");

        assert_ne!(first, 0);
        assert_eq!(first, second);
    }

    #[test]
    fn raft_rs_bootstrap_constructs_raw_node_without_leadership() {
        let config = ClusterConfig {
            mode: ClusterModeConfig::Raft,
            node_id: "scheduler-0".to_owned(),
            peers: vec![
                ClusterPeerConfig {
                    node_id: "scheduler-0".to_owned(),
                    endpoint: "http://scheduler-0.scheduler-headless:9999".to_owned(),
                },
                ClusterPeerConfig {
                    node_id: "scheduler-1".to_owned(),
                    endpoint: "http://scheduler-1.scheduler-headless:9999".to_owned(),
                },
            ],
        };

        let status = validate_raft_rs_bootstrap(&config)
            .unwrap_or_else(|error| panic!("raft-rs bootstrap should validate: {error}"));

        assert_eq!(status.node_id, "scheduler-0");
        assert_eq!(status.voter_ids.len(), 2);
        assert_eq!(status.initial_role, "follower");
    }
}
