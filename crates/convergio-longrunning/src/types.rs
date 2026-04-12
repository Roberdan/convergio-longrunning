//! Shared types for the long-running execution protocol.

use serde::{Deserialize, Serialize};

/// Errors specific to long-running executions.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum LongRunError {
    #[error("database: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("pool: {0}")]
    Pool(#[from] r2d2::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("budget exceeded: spent {spent:.4} of {limit:.4} USD")]
    BudgetExceeded { spent: f64, limit: f64 },
    #[error("execution stale: no heartbeat for {elapsed_secs}s (max {max_secs}s)")]
    Stale { elapsed_secs: u64, max_secs: u64 },
    #[error("not found: {0}")]
    NotFound(String),
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("{0}")]
    Internal(String),
}

/// Max allowed length for execution IDs and other string identifiers.
pub const MAX_ID_LEN: usize = 256;
/// Max recursion depth for delegation tree traversal.
pub const MAX_TREE_DEPTH: usize = 64;

/// Validate that an execution_id is non-empty and within length limits.
pub fn validate_execution_id(id: &str) -> LongRunResult<()> {
    if id.is_empty() {
        return Err(LongRunError::InvalidInput(
            "execution_id must not be empty".into(),
        ));
    }
    if id.len() > MAX_ID_LEN {
        return Err(LongRunError::InvalidInput(format!(
            "execution_id exceeds max length of {MAX_ID_LEN}"
        )));
    }
    if !id
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
    {
        return Err(LongRunError::InvalidInput(
            "execution_id contains invalid characters (allowed: alphanumeric, -, _, .)".into(),
        ));
    }
    Ok(())
}

pub type LongRunResult<T> = Result<T, LongRunError>;

/// Current stage of a long-running execution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionStage {
    Starting,
    Running,
    Checkpointing,
    Paused,
    Resuming,
    Completing,
    Failed,
    Reaped,
}

impl ExecutionStage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Starting => "starting",
            Self::Running => "running",
            Self::Checkpointing => "checkpointing",
            Self::Paused => "paused",
            Self::Resuming => "resuming",
            Self::Completing => "completing",
            Self::Failed => "failed",
            Self::Reaped => "reaped",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "starting" => Some(Self::Starting),
            "running" => Some(Self::Running),
            "checkpointing" => Some(Self::Checkpointing),
            "paused" => Some(Self::Paused),
            "resuming" => Some(Self::Resuming),
            "completing" => Some(Self::Completing),
            "failed" => Some(Self::Failed),
            "reaped" => Some(Self::Reaped),
            _ => None,
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Completing | Self::Failed | Self::Reaped)
    }
}

impl std::fmt::Display for ExecutionStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Progress snapshot for a long-running execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressSnapshot {
    pub execution_id: String,
    pub percent: f64,
    pub stage: ExecutionStage,
    pub cost_usd: f64,
    pub eta_secs: Option<u64>,
    pub message: Option<String>,
}

/// A node in the delegation chain tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationNode {
    pub execution_id: String,
    pub parent_id: Option<String>,
    pub agent: String,
    pub node: String,
    pub budget_usd: f64,
    pub deadline: Option<String>,
    pub stage: ExecutionStage,
    pub children: Vec<DelegationNode>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execution_stage_roundtrip() {
        let stages = [
            ExecutionStage::Starting,
            ExecutionStage::Running,
            ExecutionStage::Checkpointing,
            ExecutionStage::Paused,
            ExecutionStage::Resuming,
            ExecutionStage::Completing,
            ExecutionStage::Failed,
            ExecutionStage::Reaped,
        ];
        for stage in &stages {
            let s = stage.as_str();
            let parsed = ExecutionStage::parse(s).unwrap();
            assert_eq!(&parsed, stage);
            assert_eq!(stage.to_string(), s);
        }
    }

    #[test]
    fn execution_stage_parse_invalid() {
        assert!(ExecutionStage::parse("bogus").is_none());
    }

    #[test]
    fn terminal_stages() {
        assert!(!ExecutionStage::Running.is_terminal());
        assert!(ExecutionStage::Completing.is_terminal());
        assert!(ExecutionStage::Failed.is_terminal());
        assert!(ExecutionStage::Reaped.is_terminal());
    }

    #[test]
    fn validate_execution_id_rejects_empty() {
        assert!(validate_execution_id("").is_err());
    }

    #[test]
    fn validate_execution_id_rejects_too_long() {
        let long = "a".repeat(MAX_ID_LEN + 1);
        assert!(validate_execution_id(&long).is_err());
    }

    #[test]
    fn validate_execution_id_rejects_special_chars() {
        assert!(validate_execution_id("exec/../../etc").is_err());
        assert!(validate_execution_id("exec; DROP TABLE").is_err());
    }

    #[test]
    fn validate_execution_id_accepts_valid() {
        assert!(validate_execution_id("exec-1").is_ok());
        assert!(validate_execution_id("my_task.v2").is_ok());
    }

    #[test]
    fn progress_snapshot_serializes() {
        let snap = ProgressSnapshot {
            execution_id: "exec-1".into(),
            percent: 42.5,
            stage: ExecutionStage::Running,
            cost_usd: 0.03,
            eta_secs: Some(120),
            message: Some("processing wave 2".into()),
        };
        let json = serde_json::to_string(&snap).unwrap();
        assert!(json.contains("42.5"));
        assert!(json.contains("running"));
    }
}
