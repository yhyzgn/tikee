use std::collections::{HashMap, HashSet};

use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

use crate::entities::workflow_node_instance;

use super::types::{
    WorkflowDefinition, WorkflowEdgeSpec, WorkflowNodeSpec, WorkflowValidationResult,
};

pub fn validate_workflow_definition(definition: &WorkflowDefinition) -> WorkflowValidationResult {
    let mut errors = Vec::new();
    if definition.nodes.is_empty() {
        errors.push("workflow must contain at least one node".to_owned());
    }
    let mut keys = HashSet::new();
    for node in &definition.nodes {
        if node.key.trim().is_empty() {
            errors.push("node key cannot be empty".to_owned());
        }
        if !keys.insert(node.key.clone()) {
            errors.push(format!("duplicate node key: {}", node.key));
        }
    }
    let allowed_kinds = [
        "start",
        "end",
        "job",
        "script",
        "http",
        "grpc",
        "sql",
        "condition",
        "parallel",
        "join",
        "delay",
        "approval",
        "notification",
        "compensation",
        "file_cleanup",
        "map",
        "map_reduce",
        "sub_workflow",
    ];
    for node in &definition.nodes {
        let kind = node.kind.as_deref().unwrap_or("job");
        if !allowed_kinds.contains(&kind) {
            errors.push(format!("unsupported node kind: {kind}"));
        }
        if kind == "job" && node.job_id.as_deref().unwrap_or("").is_empty() {
            errors.push(format!("job node {} requires job_id", node.key));
        }
        if kind == "condition" && workflow_config_string(node, "expression").is_none() {
            errors.push(format!(
                "condition node {} requires config.expression",
                node.key
            ));
        }
        if kind == "http" && workflow_config_string(node, "url").is_none() {
            errors.push(format!("http node {} requires config.url", node.key));
        }
        if kind == "grpc" {
            for field in ["endpoint", "service", "method"] {
                if workflow_config_string(node, field).is_none() {
                    errors.push(format!("grpc node {} requires config.{field}", node.key));
                }
            }
        }
        if kind == "sql" {
            for field in ["databaseUrl", "sql"] {
                if workflow_config_string(node, field).is_none() {
                    errors.push(format!("sql node {} requires config.{field}", node.key));
                }
            }
            let has_allowed_database_urls = node
                .config
                .as_ref()
                .and_then(|value| value.get("allowedDatabaseUrls"))
                .and_then(serde_json::Value::as_array)
                .is_some_and(|urls| !urls.is_empty());
            if !has_allowed_database_urls {
                errors.push(format!(
                    "sql node {} requires config.allowedDatabaseUrls",
                    node.key
                ));
            }
        }
        if kind == "script" && workflow_config_string(node, "source").is_none() {
            errors.push(format!("script node {} requires config.source", node.key));
        }
        if kind == "approval" && workflow_config_string(node, "approvers").is_none() {
            errors.push(format!(
                "approval node {} requires config.approvers",
                node.key
            ));
        }
        if kind == "compensation" && workflow_config_string(node, "compensates").is_none() {
            errors.push(format!(
                "compensation node {} requires config.compensates",
                node.key
            ));
        }
        if kind == "file_cleanup" {
            let has_paths = node
                .config
                .as_ref()
                .and_then(|value| value.get("paths"))
                .and_then(serde_json::Value::as_array)
                .is_some_and(|paths| !paths.is_empty())
                || workflow_config_string(node, "path").is_some();
            let has_allowed_roots = node
                .config
                .as_ref()
                .and_then(|value| value.get("allowedRoots"))
                .and_then(serde_json::Value::as_array)
                .is_some_and(|roots| !roots.is_empty());
            if !has_paths {
                errors.push(format!("file_cleanup node {} requires config.paths", node.key));
            }
            if !has_allowed_roots {
                errors.push(format!(
                    "file_cleanup node {} requires config.allowedRoots",
                    node.key
                ));
            }
        }
        if kind == "sub_workflow" && node.child_workflow_id.as_deref().unwrap_or("").is_empty() {
            errors.push(format!(
                "sub_workflow node {} requires child_workflow_id",
                node.key
            ));
        }
        if (kind == "map" || kind == "map_reduce")
            && node.map_items.as_ref().is_none_or(Vec::is_empty)
        {
            errors.push(format!("{kind} node {} requires map_items", node.key));
        }
    }
    let allowed_conditions = ["always", "on_success", "on_failure"];
    for edge in &definition.edges {
        if !keys.contains(&edge.from) {
            errors.push(format!("edge references missing from node: {}", edge.from));
        }
        if !keys.contains(&edge.to) {
            errors.push(format!("edge references missing to node: {}", edge.to));
        }
        let condition = edge.condition.as_deref().unwrap_or("always");
        if !allowed_conditions.contains(&condition) {
            errors.push(format!("unsupported edge condition: {condition}"));
        }
    }
    if !definition.nodes.is_empty() && start_node_keys(definition).is_empty() {
        errors.push("workflow must contain at least one start node".to_owned());
    }
    if has_cycle(definition) {
        errors.push("workflow graph must be acyclic".to_owned());
    }
    WorkflowValidationResult {
        valid: errors.is_empty(),
        errors,
    }
}

pub(super) fn workflow_config_string<'a>(node: &'a WorkflowNodeSpec, key: &str) -> Option<&'a str> {
    node.config
        .as_ref()
        .and_then(|value| value.get(key))
        .and_then(serde_json::Value::as_str)
        .filter(|value| !value.trim().is_empty())
}


pub(super) fn workflow_config_i64(node: &WorkflowNodeSpec, key: &str) -> Option<i64> {
    node.config.as_ref().and_then(|value| value.get(key)).and_then(|value| {
        value.as_i64().or_else(|| value.as_str()?.trim().parse::<i64>().ok())
    })
}

pub(super) fn workflow_config_bool(node: &WorkflowNodeSpec, key: &str) -> Option<bool> {
    node.config.as_ref().and_then(|value| value.get(key)).and_then(|value| {
        value.as_bool().or_else(|| match value.as_str()?.trim().to_ascii_lowercase().as_str() {
            "true" | "yes" | "1" | "success" | "succeeded" => Some(true),
            "false" | "no" | "0" | "failure" | "failed" => Some(false),
            _ => None,
        })
    })
}

pub(super) fn next_nodes_for_status(
    definition: &WorkflowDefinition,
    node_key: &str,
    status: &str,
) -> Vec<String> {
    definition
        .edges
        .iter()
        .filter(|edge| edge.from == node_key)
        .filter(|edge| {
            edge_condition_satisfied(status, edge.condition.as_deref().unwrap_or("always"))
        })
        .map(|edge| edge.to.clone())
        .collect()
}

pub(super) async fn all_predecessors_satisfied<C>(
    definition: &WorkflowDefinition,
    node_key: &str,
    instance_id: &str,
    db: &C,
) -> Result<bool, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let incoming: Vec<&WorkflowEdgeSpec> = definition
        .edges
        .iter()
        .filter(|edge| edge.to == node_key)
        .collect();
    for edge in incoming {
        let predecessor = workflow_node_instance::Entity::find()
            .filter(workflow_node_instance::Column::WorkflowInstanceId.eq(instance_id.to_owned()))
            .filter(workflow_node_instance::Column::NodeKey.eq(edge.from.clone()))
            .one(db)
            .await?
            .ok_or_else(|| sea_orm::DbErr::RecordNotFound(edge.from.clone()))?;
        if !edge_condition_satisfied(
            &predecessor.status,
            edge.condition.as_deref().unwrap_or("always"),
        ) {
            return Ok(false);
        }
    }
    Ok(true)
}

fn edge_condition_satisfied(status: &str, condition: &str) -> bool {
    match condition {
        "always" => matches!(status, "succeeded" | "failed" | "skipped"),
        "on_failure" => status == "failed",
        _ => status == "succeeded",
    }
}

pub(super) fn start_node_keys(definition: &WorkflowDefinition) -> HashSet<String> {
    let targets: HashSet<_> = definition
        .edges
        .iter()
        .map(|edge| edge.to.clone())
        .collect();
    definition
        .nodes
        .iter()
        .filter(|node| !targets.contains(&node.key))
        .map(|node| node.key.clone())
        .collect()
}

fn has_cycle(definition: &WorkflowDefinition) -> bool {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for node in &definition.nodes {
        graph.entry(&node.key).or_default();
    }
    for edge in &definition.edges {
        graph.entry(&edge.from).or_default().push(&edge.to);
    }
    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    definition
        .nodes
        .iter()
        .any(|node| dfs_cycle(&node.key, &graph, &mut visiting, &mut visited))
}

fn dfs_cycle<'a>(
    node: &'a str,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    visiting: &mut HashSet<&'a str>,
    visited: &mut HashSet<&'a str>,
) -> bool {
    if visited.contains(node) {
        return false;
    }
    if !visiting.insert(node) {
        return true;
    }
    if let Some(next) = graph.get(node) {
        for child in next {
            if dfs_cycle(child, graph, visiting, visited) {
                return true;
            }
        }
    }
    visiting.remove(node);
    visited.insert(node);
    false
}
