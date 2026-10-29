use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    High,
    Medium,
    Low,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }

    pub fn rank(self) -> u8 {
        match self {
            Self::High => 0,
            Self::Medium => 1,
            Self::Low => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    Exact,
    Heuristic,
}

impl Confidence {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Heuristic => "heuristic",
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootTiming {
    pub total_sec: Option<f64>,
    pub firmware_sec: Option<f64>,
    pub loader_sec: Option<f64>,
    pub kernel_sec: Option<f64>,
    pub initrd_sec: Option<f64>,
    pub userspace_sec: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitTiming {
    pub name: String,
    pub time_sec: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainNode {
    pub unit: String,
    pub active_at_sec: f64,
    pub start_sec: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitFailure {
    pub unit: String,
    pub result: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Finding {
    pub id: String,
    pub severity: Severity,
    pub confidence: Confidence,
    pub evidence: String,
    pub why: String,
    pub investigate: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootEvidence {
    pub boot_id: Option<String>,
    pub boot_index: Option<i32>,
    pub target: String,
    pub timing: BootTiming,
    pub units: Vec<UnitTiming>,
    pub critical_chain: Vec<ChainNode>,
    pub failures: Vec<UnitFailure>,
    pub initrd_bytes: Option<u64>,
    pub capture_incomplete: bool,
    pub incomplete_notes: Vec<String>,
}
