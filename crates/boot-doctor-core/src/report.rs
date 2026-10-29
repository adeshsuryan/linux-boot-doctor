use serde::Serialize;

use crate::evidence::{BootEvidence, Finding};
use crate::rules::Budgets;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportDocument {
    pub schema_version: u32,
    pub tool: String,
    pub version: String,
    pub budgets: BudgetView,
    pub boot: BootEvidence,
    pub findings: Vec<Finding>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BudgetView {
    pub max_total: f64,
    pub max_unit_sec: f64,
    pub max_initrd_mb: f64,
    pub max_initrd_sec: f64,
}

pub fn render_text(evidence: &BootEvidence, findings: &[Finding], budgets: &Budgets) -> String {
    let mut out = String::new();
    out.push_str("Linux boot doctor\n\n");
    out.push_str(&format!("BOOT  {} reached\n", evidence.target));
    let t = &evidence.timing;
    out.push_str(&format!(
        "Total: {}   firmware: {}   loader: {}   kernel: {}   initrd: {}   userspace: {}\n\n",
        fmt_opt(t.total_sec),
        fmt_opt(t.firmware_sec),
        fmt_opt(t.loader_sec),
        fmt_opt(t.kernel_sec),
        fmt_opt(t.initrd_sec),
        fmt_opt(t.userspace_sec),
    ));
    out.push_str(&format!(
        "Budgets: max-total={:.1}s max-unit={:.1}s max-initrd-mb={:.1}\n\n",
        budgets.max_total, budgets.max_unit_sec, budgets.max_initrd_mb
    ));
    out.push_str("FINDINGS\n");
    if findings.is_empty() {
        out.push_str("(none)\n");
    } else {
        for f in findings {
            out.push_str(&format!(
                "{:<4}  {:<28}  {}   confidence: {}\n",
                f.severity.as_str().to_ascii_uppercase(),
                f.id,
                f.evidence,
                f.confidence.as_str()
            ));
            out.push_str(&format!("      why: {}\n", f.why));
        }
    }
    out.push_str("\nINVESTIGATE\n");
    let mut n = 1;
    for f in findings {
        for tip in &f.investigate {
            out.push_str(&format!("{n}. {tip}\n"));
            n += 1;
        }
    }
    if findings.is_empty() {
        out.push_str("1. No findings at current budgets.\n");
    }
    out
}

fn fmt_opt(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.3}s"),
        None => "n/a".into(),
    }
}

pub fn render_json(
    evidence: &BootEvidence,
    findings: &[Finding],
    budgets: &Budgets,
    version: &str,
) -> Result<String, serde_json::Error> {
    let doc = ReportDocument {
        schema_version: 1,
        tool: "linux-boot-doctor".into(),
        version: version.into(),
        budgets: BudgetView {
            max_total: budgets.max_total,
            max_unit_sec: budgets.max_unit_sec,
            max_initrd_mb: budgets.max_initrd_mb,
            max_initrd_sec: budgets.max_initrd_sec,
        },
        boot: evidence.clone(),
        findings: findings.to_vec(),
    };
    serde_json::to_string_pretty(&doc)
}
