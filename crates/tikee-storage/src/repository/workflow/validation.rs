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
        "condition",
        "parallel",
        "join",
        "delay",
        "approval",
        "notification",
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
        if kind == "condition" && node_config_string(node, "expression").is_none() {
            errors.push(format!(
                "condition node {} requires config.expression",
                node.key
            ));
        }
        if kind == "http" && node_config_string(node, "url").is_none() {
            errors.push(format!("http node {} requires config.url", node.key));
        }
        if kind == "script" && node_config_string(node, "source").is_none() {
            errors.push(format!("script node {} requires config.source", node.key));
        }
        if kind == "approval" && node_config_string(node, "approvers").is_none() {
            errors.push(format!(
                "approval node {} requires config.approvers",
                node.key
            ));
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

fn node_config_string<'a>(node: &'a WorkflowNodeSpec, key: &str) -> Option<&'a str> {
    node.config
        .as_ref()
        .and_then(|value| value.get(key))
        .and_then(serde_json::Value::as_str)
        .filter(|value| !value.trim().is_empty())
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
