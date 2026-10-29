use std::path::PathBuf;

use boot_doctor_core::{evaluate, load_capture_dir, Budgets};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(name)
}

#[test]
fn clean_boot_silent_at_default_budgets() {
    let evidence = load_capture_dir(&fixture("clean-boot")).expect("load");
    let findings = evaluate(&evidence, &Budgets::default());
    let high = findings
        .iter()
        .filter(|f| f.id != "capture-incomplete")
        .filter(|f| matches!(f.severity, boot_doctor_core::Severity::High))
        .count();
    assert_eq!(high, 0, "unexpected high findings: {findings:?}");
}

#[test]
fn slow_total_fires_budget() {
    let evidence = load_capture_dir(&fixture("slow-total")).expect("load");
    let findings = evaluate(&evidence, &Budgets::default());
    assert!(
        findings.iter().any(|f| f.id == "boot-time-budget-exceeded"),
        "{findings:?}"
    );
}

#[test]
fn fat_initrd_uses_meta_bytes() {
    let evidence = load_capture_dir(&fixture("fat-initrd")).expect("load");
    assert_eq!(evidence.initrd_bytes, Some(52_428_800));
    let findings = evaluate(&evidence, &Budgets::default());
    assert!(
        findings.iter().any(|f| f.id == "initramfs-size-budget"),
        "{findings:?}"
    );
}

#[test]
fn failed_unit_fires() {
    let evidence = load_capture_dir(&fixture("failed-unit")).expect("load");
    let findings = evaluate(&evidence, &Budgets::default());
    assert!(
        findings.iter().any(|f| f.id == "unit-failed-before-target"),
        "{findings:?}"
    );
}

#[test]
fn many_slow_units_fires() {
    let evidence = load_capture_dir(&fixture("many-slow-units")).expect("load");
    let findings = evaluate(&evidence, &Budgets::default());
    assert!(
        findings
            .iter()
            .any(|f| f.id == "excessive-unit-count-slow"),
        "{findings:?}"
    );
}

#[test]
fn raised_budget_silences_slow_total() {
    let evidence = load_capture_dir(&fixture("slow-total")).expect("load");
    let mut budgets = Budgets::default();
    budgets.max_total = 120.0;
    budgets.max_unit_sec = 60.0;
    let findings = evaluate(&evidence, &budgets);
    assert!(
        !findings.iter().any(|f| f.id == "boot-time-budget-exceeded"),
        "{findings:?}"
    );
}
