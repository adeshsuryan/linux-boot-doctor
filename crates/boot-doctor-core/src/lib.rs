//! Evidence, parsers, rules, and report rendering for linux-boot-doctor.

pub mod capture;
mod evidence;
mod parse;
mod report;
mod rules;

pub use capture::{capture_live, load_capture_dir, CaptureIncomplete};
pub use evidence::{
    BootEvidence, BootTiming, ChainNode, Confidence, Finding, Severity, UnitFailure, UnitTiming,
};
pub use parse::{parse_blame, parse_critical_chain, parse_duration, parse_time, parse_verify};
pub use report::{render_json, render_text, ReportDocument};
pub use rules::{evaluate, Budgets, FailOn};
