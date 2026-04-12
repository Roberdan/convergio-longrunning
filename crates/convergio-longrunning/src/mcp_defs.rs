//! MCP tool definitions for the long-running task extension.

use convergio_types::extension::McpToolDef;
use serde_json::json;

pub fn longrunning_tools() -> Vec<McpToolDef> {
    vec![
        McpToolDef {
            name: "cvg_register_heartbeat".into(),
            description: "Register a heartbeat for a long-running task.".into(),
            method: "POST".into(),
            path: "/api/longrunning/heartbeat".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "task_id": {"type": "string"},
                    "agent_id": {"type": "string"}
                },
                "required": ["task_id", "agent_id"]
            }),
            min_ring: "trusted".into(),
            path_params: vec![],
        },
        McpToolDef {
            name: "cvg_heartbeat_beat".into(),
            description: "Send a heartbeat beat for a long-running task.".into(),
            method: "POST".into(),
            path: "/api/longrunning/heartbeat/beat".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "task_id": {"type": "string"},
                    "status": {"type": "string"}
                },
                "required": ["task_id"]
            }),
            min_ring: "trusted".into(),
            path_params: vec![],
        },
        McpToolDef {
            name: "cvg_stale_heartbeats".into(),
            description: "Get stale heartbeats (tasks that stopped reporting).".into(),
            method: "GET".into(),
            path: "/api/longrunning/heartbeat/stale".into(),
            input_schema: json!({"type": "object", "properties": {}}),
            min_ring: "community".into(),
            path_params: vec![],
        },
        McpToolDef {
            name: "cvg_get_checkpoint".into(),
            description: "Get checkpoint data for a task.".into(),
            method: "GET".into(),
            path: "/api/longrunning/checkpoint/:id".into(),
            input_schema: json!({
                "type": "object",
                "properties": {"id": {"type": "string"}},
                "required": ["id"]
            }),
            min_ring: "community".into(),
            path_params: vec!["id".into()],
        },
        McpToolDef {
            name: "cvg_save_checkpoint".into(),
            description: "Save a checkpoint for a long-running task.".into(),
            method: "POST".into(),
            path: "/api/longrunning/checkpoint/:id".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "id": {"type": "string"},
                    "data": {"type": "object", "description": "Checkpoint data"}
                },
                "required": ["id"]
            }),
            min_ring: "trusted".into(),
            path_params: vec!["id".into()],
        },
        McpToolDef {
            name: "cvg_clear_checkpoint".into(),
            description: "Clear checkpoint data for a task.".into(),
            method: "DELETE".into(),
            path: "/api/longrunning/checkpoint/:id/clear".into(),
            input_schema: json!({
                "type": "object",
                "properties": {"id": {"type": "string"}},
                "required": ["id"]
            }),
            min_ring: "trusted".into(),
            path_params: vec!["id".into()],
        },
        McpToolDef {
            name: "cvg_task_progress".into(),
            description: "Get progress of a long-running task.".into(),
            method: "GET".into(),
            path: "/api/longrunning/progress/:id".into(),
            input_schema: json!({
                "type": "object",
                "properties": {"id": {"type": "string"}},
                "required": ["id"]
            }),
            min_ring: "community".into(),
            path_params: vec!["id".into()],
        },
        McpToolDef {
            name: "cvg_get_delegation".into(),
            description: "Get delegation info for a long-running task.".into(),
            method: "GET".into(),
            path: "/api/longrunning/delegation/:id".into(),
            input_schema: json!({
                "type": "object",
                "properties": {"id": {"type": "string"}},
                "required": ["id"]
            }),
            min_ring: "community".into(),
            path_params: vec!["id".into()],
        },
        McpToolDef {
            name: "cvg_delegation_children".into(),
            description: "Get child tasks of a delegation.".into(),
            method: "GET".into(),
            path: "/api/longrunning/delegation/:id/children".into(),
            input_schema: json!({
                "type": "object",
                "properties": {"id": {"type": "string"}},
                "required": ["id"]
            }),
            min_ring: "community".into(),
            path_params: vec!["id".into()],
        },
        McpToolDef {
            name: "cvg_task_budget".into(),
            description: "Get budget info for a long-running task.".into(),
            method: "GET".into(),
            path: "/api/longrunning/budget/:id".into(),
            input_schema: json!({
                "type": "object",
                "properties": {"id": {"type": "string"}},
                "required": ["id"]
            }),
            min_ring: "community".into(),
            path_params: vec!["id".into()],
        },
    ]
}
