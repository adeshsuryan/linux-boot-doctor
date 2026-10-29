use crate::evidence::{BootEvidence, Confidence, Finding, Severity};

#[derive(Debug, Clone)]
pub struct Budgets {
    pub max_total: f64,
    pub max_unit_sec: f64,
    pub max_initrd_mb: f64,
    pub max_initrd_sec: f64,
    pub excessive_unit_count: usize,
    pub slow_unit_floor_sec: f64,
    pub chain_depth_n: usize,
    pub ignore: Vec<String>,
}

impl Default for Budgets {
    fn default() -> Self {
        Self {
            max_total: 30.0,
            max_unit_sec: 5.0,
            max_initrd_mb: 32.0,
            max_initrd_sec: 8.0,
            excessive_unit_count: 25,
            slow_unit_floor_sec: 0.5,
            chain_depth_n: 5,
            ignore: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailOn {
    None,
    Low,
    Medium,
    High,
}

impl FailOn {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "none" => Some(Self::None),
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            _ => None,
        }
    }

    pub fn should_fail(self, severity: Severity) -> bool {
        match self {
            Self::None => false,
            Self::High => matches!(severity, Severity::High),
            Self::Medium => matches!(severity, Severity::High | Severity::Medium),
            Self::Low => true,
        }
    }
}

fn ignored(budgets: &Budgets, id: &str) -> bool {
    budgets.ignore.iter().any(|x| x == id)
}

fn push(out: &mut Vec<Finding>, budgets: &Budgets, f: Finding) {
    if !ignored(budgets, &f.id) {
        out.push(f);
    }
}

pub fn evaluate(evidence: &BootEvidence, budgets: &Budgets) -> Vec<Finding> {
    let mut findings = Vec::new();

    if evidence.capture_incomplete {
        push(
            &mut findings,
            budgets,
            Finding {
                id: "capture-incomplete".into(),
                severity: Severity::Low,
                confidence: Confidence::Exact,
                evidence: evidence.incomplete_notes.join("; "),
                why: "One or more analyze inputs were missing or failed.".into(),
                investigate: vec![
                    "Re-run with a complete --input capture directory.".into(),
                    "On LIVE mode, check systemd-analyze privileges and PATH.".into(),
                ],
            },
        );
    }

    if let Some(total) = evidence.timing.total_sec {
        if total > budgets.max_total {
            push(
                &mut findings,
                budgets,
                Finding {
                    id: "boot-time-budget-exceeded".into(),
                    severity: Severity::High,
                    confidence: Confidence::Exact,
                    evidence: format!("total {total:.3}s > budget {:.3}s", budgets.max_total),
                    why: "Measured boot total exceeds the configured budget.".into(),
                    investigate: vec![
                        "Review critical-chain and blame for deferrable units.".into(),
                        "For cloud images, consider --max-total 8 in CI.".into(),
                    ],
                },
            );
        }

        if let Some(userspace) = evidence.timing.userspace_sec {
            if total > 10.0 && userspace / total > 0.70 {
                push(
                    &mut findings,
                    budgets,
                    Finding {
                        id: "userspace-boot-dominated".into(),
                        severity: Severity::Medium,
                        confidence: Confidence::Heuristic,
                        evidence: format!(
                            "userspace {userspace:.3}s is {:.0}% of total {total:.3}s",
                            100.0 * userspace / total
                        ),
                        why: "Userspace systemd activation dominates wall boot time.".into(),
                        investigate: vec![
                            "Defer non-critical services after multi-user.target.".into(),
                            "Inspect NetworkManager-wait-online and similar blockers.".into(),
                        ],
                    },
                );
            }
        }

        if let Some(fw) = evidence.timing.firmware_sec {
            if fw / total > 0.25 {
                push(
                    &mut findings,
                    budgets,
                    Finding {
                        id: "firmware-boot-dominated".into(),
                        severity: Severity::Low,
                        confidence: Confidence::Heuristic,
                        evidence: format!(
                            "firmware {fw:.3}s is {:.0}% of total {total:.3}s",
                            100.0 * fw / total
                        ),
                        why: "Firmware phase is a large fraction of boot.".into(),
                        investigate: vec![
                            "Review firmware/BIOS settings and hardware POST.".into(),
                        ],
                    },
                );
            }
        }
    }

    // initramfs-size-budget cascade
    if let Some(bytes) = evidence.initrd_bytes {
        let mb = bytes as f64 / (1024.0 * 1024.0);
        if mb > budgets.max_initrd_mb {
            push(
                &mut findings,
                budgets,
                Finding {
                    id: "initramfs-size-budget".into(),
                    severity: Severity::Medium,
                    confidence: Confidence::Exact,
                    evidence: format!("initrd {mb:.1} MB > budget {:.1} MB", budgets.max_initrd_mb),
                    why: "Initramfs image size exceeds budget.".into(),
                    investigate: vec![
                        "Trim modules (dracut --hostonly / mkinitcpio review).".into(),
                    ],
                },
            );
        }
    } else if let Some(initrd) = evidence.timing.initrd_sec {
        if initrd > budgets.max_initrd_sec {
            push(
                &mut findings,
                budgets,
                Finding {
                    id: "initramfs-size-budget".into(),
                    severity: Severity::Medium,
                    confidence: Confidence::Heuristic,
                    evidence: format!(
                        "initrd phase {initrd:.3}s > --max-initrd-sec {:.3}s (size unknown)",
                        budgets.max_initrd_sec
                    ),
                    why: "Initrd phase time is high; size could not be measured.".into(),
                    investigate: vec![
                        "Pass --initrd-bytes or ensure /boot initrd is readable.".into(),
                        "Review initramfs contents for unused modules.".into(),
                    ],
                },
            );
        }
    }

    let chain_names: Vec<&str> = evidence.critical_chain.iter().map(|c| c.unit.as_str()).collect();
    for node in &evidence.critical_chain {
        let dur = node.start_sec.unwrap_or(0.0);
        // If start_sec missing, approximate from blame
        let dur = if dur > 0.0 {
            dur
        } else {
            evidence
                .units
                .iter()
                .find(|u| u.name == node.unit)
                .map(|u| u.time_sec)
                .unwrap_or(0.0)
        };
        if dur > budgets.max_unit_sec {
            push(
                &mut findings,
                budgets,
                Finding {
                    id: "slow-critical-chain-unit".into(),
                    severity: Severity::High,
                    confidence: Confidence::Exact,
                    evidence: format!("{} {:.3}s on critical chain", node.unit, dur),
                    why: "A unit on the critical chain exceeds --max-unit-sec.".into(),
                    investigate: vec![
                        format!("Review whether {} must block default.target.", node.unit),
                        "Consider TimeoutStartSec or making the dependency optional.".into(),
                    ],
                },
            );
        }
        let lname = node.unit.to_ascii_lowercase();
        if lname.contains("wait-online") && dur > 3.0 {
            push(
                &mut findings,
                budgets,
                Finding {
                    id: "network-wait-online-suspect".into(),
                    severity: Severity::Medium,
                    confidence: Confidence::Heuristic,
                    evidence: format!("{} {:.3}s on chain", node.unit, dur),
                    why: "network-online wait unit is slowing the critical path.".into(),
                    investigate: vec![
                        "Disable or timeout wait-online on headless/cloud images if safe.".into(),
                    ],
                },
            );
        }
        if lname.contains("getty") {
            push(
                &mut findings,
                budgets,
                Finding {
                    id: "serial-getty-on-critical-path".into(),
                    severity: Severity::Low,
                    confidence: Confidence::Heuristic,
                    evidence: format!("{} on critical chain", node.unit),
                    why: "A getty appears on the critical path.".into(),
                    investigate: vec![
                        "Confirm whether console getty must block multi-user.target.".into(),
                    ],
                },
            );
        }
    }

    let slow_chain_count = evidence
        .critical_chain
        .iter()
        .filter(|n| {
            let dur = n.start_sec.unwrap_or_else(|| {
                evidence
                    .units
                    .iter()
                    .find(|u| u.name == n.unit)
                    .map(|u| u.time_sec)
                    .unwrap_or(0.0)
            });
            dur > 1.0
        })
        .count();
    if slow_chain_count > budgets.chain_depth_n {
        push(
            &mut findings,
            budgets,
            Finding {
                id: "critical-chain-depth-high".into(),
                severity: Severity::Medium,
                confidence: Confidence::Heuristic,
                evidence: format!("{slow_chain_count} chain units each > 1s"),
                why: "Critical chain has many sequential slow units.".into(),
                investigate: vec!["Look for artificial After= ordering that serializes boot.".into()],
            },
        );
    }

    if let Some(top) = evidence.units.first() {
        if top.time_sec > budgets.max_unit_sec && !chain_names.contains(&top.name.as_str()) {
            push(
                &mut findings,
                budgets,
                Finding {
                    id: "slow-blame-unit".into(),
                    severity: Severity::Medium,
                    confidence: Confidence::Exact,
                    evidence: format!("{} {:.3}s (top blame, not on chain)", top.name, top.time_sec),
                    why: "A unit is slow in blame but not on the critical chain.".into(),
                    investigate: vec![
                        format!("Still consider deferring or optimizing {}.", top.name),
                    ],
                },
            );
        }
    }

    for fail in &evidence.failures {
        push(
            &mut findings,
            budgets,
            Finding {
                id: "unit-failed-before-target".into(),
                severity: Severity::High,
                confidence: Confidence::Exact,
                evidence: fail
                    .message
                    .clone()
                    .unwrap_or_else(|| format!("{} {}", fail.unit, fail.result)),
                why: "A unit failure was recorded before the boot target.".into(),
                investigate: vec![
                    format!("Inspect logs for {}.", fail.unit),
                    "Fix the unit or remove it from the image.".into(),
                ],
            },
        );
    }

    let slow_many = evidence
        .units
        .iter()
        .filter(|u| u.time_sec > budgets.slow_unit_floor_sec)
        .count();
    if slow_many > budgets.excessive_unit_count {
        push(
            &mut findings,
            budgets,
            Finding {
                id: "excessive-unit-count-slow".into(),
                severity: Severity::Medium,
                confidence: Confidence::Heuristic,
                evidence: format!(
                    "{slow_many} units each > {:.0}ms",
                    budgets.slow_unit_floor_sec * 1000.0
                ),
                why: "Many units are individually slow, inflating userspace boot.".into(),
                investigate: vec![
                    "Reduce enabled services on appliance images.".into(),
                    "Profile the top blame units.".into(),
                ],
            },
        );
    }

    findings.sort_by(|a, b| {
        a.severity
            .rank()
            .cmp(&b.severity.rank())
            .then_with(|| a.id.cmp(&b.id))
            .then_with(|| a.evidence.cmp(&b.evidence))
    });
    findings
}
