use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use crate::entities::{raft_member, raft_metadata};

use super::util::{new_id, now_rfc3339};

/// Raft metadata upsert input.
#[derive(Debug, Clone)]
pub struct UpsertRaftMetadata {
    /// Logical cluster identifier.
    pub cluster_id: String,
    /// Stable local node id.
    pub node_id: String,
    /// Last known Raft term.
    pub current_term: i64,
    /// Vote target in the current term.
    pub voted_for: Option<String>,
    /// Last committed index.
    pub commit_index: i64,
    /// Last applied index.
    pub applied_index: i64,
}

/// Raft member upsert input.
#[derive(Debug, Clone)]
pub struct UpsertRaftMember {
    /// Stable member node id.
    pub node_id: String,
    /// Peer endpoint reachable through container/K8s networking.
    pub endpoint: String,
    /// Member lifecycle status.
    pub status: String,
}

/// Stored Raft metadata summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RaftMetadataSummary {
    /// Metadata row identifier.
    pub id: String,
    /// Logical cluster identifier.
    pub cluster_id: String,
    /// Stable local node id.
    pub node_id: String,
    /// Last known Raft term.
    pub current_term: i64,
    /// Vote target in the current term.
    pub voted_for: Option<String>,
    /// Last committed index.
    pub commit_index: i64,
    /// Last applied index.
    pub applied_index: i64,
    /// Last update timestamp.
    pub updated_at: String,
}

/// Stored Raft member summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RaftMemberSummary {
    /// Member row identifier.
    pub id: String,
    /// Stable member node id.
    pub node_id: String,
    /// Peer endpoint reachable through container/K8s networking.
    pub endpoint: String,
    /// Member lifecycle status.
    pub status: String,
    /// Creation timestamp.
    pub created_at: String,
    /// Last update timestamp.
    pub updated_at: String,
}

/// Repository for Raft metadata and static member bootstrap records.
#[derive(Debug, Clone)]
pub struct RaftRepository {
    db: DatabaseConnection,
}

impl RaftRepository {
    /// Create a repository using the provided database connection.
    #[must_use]
    pub const fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Upsert local Raft metadata by `node_id`.
    pub async fn upsert_metadata(
        &self,
        input: UpsertRaftMetadata,
    ) -> Result<RaftMetadataSummary, sea_orm::DbErr> {
        let now = now_rfc3339();
        if let Some(existing) = raft_metadata::Entity::find()
            .filter(raft_metadata::Column::NodeId.eq(input.node_id.clone()))
            .one(&self.db)
            .await?
        {
            let mut active: raft_metadata::ActiveModel = existing.into();
            active.cluster_id = Set(input.cluster_id);
            active.current_term = Set(input.current_term);
            active.voted_for = Set(input.voted_for);
            active.commit_index = Set(input.commit_index);
            active.applied_index = Set(input.applied_index);
            active.updated_at = Set(now);
            return active.update(&self.db).await.map(RaftMetadataSummary::from);
        }

        raft_metadata::ActiveModel {
            id: Set(new_id("raft_meta")),
            cluster_id: Set(input.cluster_id),
            node_id: Set(input.node_id),
            current_term: Set(input.current_term),
            voted_for: Set(input.voted_for),
            commit_index: Set(input.commit_index),
            applied_index: Set(input.applied_index),
            updated_at: Set(now),
        }
        .insert(&self.db)
        .await
        .map(RaftMetadataSummary::from)
    }

    /// Return local Raft metadata by `node_id`.
    pub async fn get_metadata(
        &self,
        node_id: &str,
    ) -> Result<Option<RaftMetadataSummary>, sea_orm::DbErr> {
        raft_metadata::Entity::find()
            .filter(raft_metadata::Column::NodeId.eq(node_id.to_owned()))
            .one(&self.db)
            .await
            .map(|row| row.map(RaftMetadataSummary::from))
    }

    /// Upsert a configured Raft member by `node_id`.
    pub async fn upsert_member(
        &self,
        input: UpsertRaftMember,
    ) -> Result<RaftMemberSummary, sea_orm::DbErr> {
        let now = now_rfc3339();
        if let Some(existing) = raft_member::Entity::find()
            .filter(raft_member::Column::NodeId.eq(input.node_id.clone()))
            .one(&self.db)
            .await?
        {
            let mut active: raft_member::ActiveModel = existing.into();
            active.endpoint = Set(input.endpoint);
            active.status = Set(input.status);
            active.updated_at = Set(now);
            return active.update(&self.db).await.map(RaftMemberSummary::from);
        }

        raft_member::ActiveModel {
            id: Set(new_id("raft_member")),
            node_id: Set(input.node_id),
            endpoint: Set(input.endpoint),
            status: Set(input.status),
            created_at: Set(now.clone()),
            updated_at: Set(now),
        }
        .insert(&self.db)
        .await
        .map(RaftMemberSummary::from)
    }

    /// List all configured Raft members.
    pub async fn list_members(&self) -> Result<Vec<RaftMemberSummary>, sea_orm::DbErr> {
        raft_member::Entity::find()
            .all(&self.db)
            .await
            .map(|rows| rows.into_iter().map(RaftMemberSummary::from).collect())
    }
}

impl From<raft_metadata::Model> for RaftMetadataSummary {
    fn from(value: raft_metadata::Model) -> Self {
        Self {
            id: value.id,
            cluster_id: value.cluster_id,
            node_id: value.node_id,
            current_term: value.current_term,
            voted_for: value.voted_for,
            commit_index: value.commit_index,
            applied_index: value.applied_index,
            updated_at: value.updated_at,
        }
    }
}

impl From<raft_member::Model> for RaftMemberSummary {
    fn from(value: raft_member::Model) -> Self {
        Self {
            id: value.id,
            node_id: value.node_id,
            endpoint: value.endpoint,
            status: value.status,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
