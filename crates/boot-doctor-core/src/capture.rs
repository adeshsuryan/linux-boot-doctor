use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

use thiserror::Error;

use crate::evidence::BootEvidence;
use crate::parse::{parse_blame, parse_critical_chain, parse_time, parse_verify};

#[derive(Debug, Error)]
pub enum CaptureIncomplete {
    #[error("missing file: {0}")]
    MissingFile(String),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Message(String),
}

fn read_optional(path: &Path) -> Result<Option<String>, CaptureIncomplete> {
    if !path.exists() {
        return Ok(None);
    }
    Ok(Some(fs::read_to_string(path)?))
}

fn read_meta_initrd_bytes(dir: &Path) -> Option<u64> {
    let meta = dir.join("meta.json");
    let text = fs::read_to_string(meta).ok()?;
    // minimal parse without requiring a full typed meta schema
    let v: serde_json::Value = serde_json::from_str(&text).ok()?;
    v.get("initrdBytes")
        .and_then(|x| x.as_u64())
        .or_else(|| v.get("initrd_bytes").and_then(|x| x.as_u64()))
}

/// Load evidence from a capture directory (`analyze-time.txt`, etc.).
pub fn load_capture_dir(dir: &Path) -> Result<BootEvidence, CaptureIncomplete> {
    let mut incomplete = Vec::new();
    let time_path = dir.join("analyze-time.txt");
    let blame_path = dir.join("analyze-blame.txt");
    let chain_path = dir.join("analyze-critical-chain.txt");
    let verify_path = dir.join("analyze-verify.txt");

    let time_text = match read_optional(&time_path)? {
        Some(t) => t,
        None => {
            incomplete.push("missing analyze-time.txt".into());
            String::new()
        }
    };
    let blame_text = match read_optional(&blame_path)? {
        Some(t) => t,
        None => {
            incomplete.push("missing analyze-blame.txt".into());
            String::new()
        }
    };
    let chain_text = match read_optional(&chain_path)? {
        Some(t) => t,
        None => {
            incomplete.push("missing analyze-critical-chain.txt".into());
            String::new()
        }
    };
    let verify_text = read_optional(&verify_path)?.unwrap_or_default();

    let evidence = BootEvidence {
        target: "multi-user.target".into(),
        timing: parse_time(&time_text),
        units: parse_blame(&blame_text),
        critical_chain: parse_critical_chain(&chain_text),
        failures: parse_verify(&verify_text),
        initrd_bytes: read_meta_initrd_bytes(dir),
        capture_incomplete: !incomplete.is_empty(),
        incomplete_notes: incomplete,
        ..Default::default()
    };

    if evidence.timing.total_sec.is_none() && evidence.units.is_empty() {
        return Err(CaptureIncomplete::Message(
            "capture produced no timing and no blame units".into(),
        ));
    }

    Ok(evidence)
}

fn run_analyze(args: &[&str]) -> Result<String, String> {
    let mut cmd = Command::new("systemd-analyze");
    cmd.args(args)
        .env("LC_ALL", "C")
        .env("LANG", "C")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let output = cmd
        .output()
        .map_err(|e| format!("failed to run systemd-analyze: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    if !output.status.success() && stdout.trim().is_empty() {
        return Err(format!(
            "systemd-analyze {} failed: {}",
            args.join(" "),
            stderr.trim()
        ));
    }
    if stdout.trim().is_empty() {
        Ok(stderr)
    } else {
        Ok(stdout)
    }
}

fn discover_initrd_bytes() -> Option<u64> {
    let candidates = [
        "/boot/initrd.img",
        "/boot/initramfs.img",
    ];
    for c in candidates {
        if let Ok(meta) = fs::metadata(c) {
            if meta.is_file() {
                return Some(meta.len());
            }
        }
    }
    // glob-like: /boot/initrd.img-* /boot/initramfs-*.img
    let boot = Path::new("/boot");
    if let Ok(rd) = fs::read_dir(boot) {
        for ent in rd.flatten() {
            let name = ent.file_name().to_string_lossy().to_string();
            if name.starts_with("initrd.img-")
                || (name.starts_with("initramfs-") && name.ends_with(".img"))
            {
                if let Ok(meta) = ent.metadata() {
                    return Some(meta.len());
                }
            }
        }
    }
    None
}

/// Capture LIVE evidence from the current system (Linux + systemd).
pub fn capture_live(boot_index: Option<i32>) -> Result<BootEvidence, CaptureIncomplete> {
    if !Path::new("/run/systemd/system").exists()
        && Command::new("systemctl")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| !s.success())
            .unwrap_or(true)
    {
        return Err(CaptureIncomplete::Message(
            "unsupported environment: systemd not detected (exit 3 for CLI)".into(),
        ));
    }

    let mut incomplete = Vec::new();
    let boot_args: Vec<String> = match boot_index {
        Some(n) => vec!["-b".into(), n.to_string()],
        None => Vec::new(),
    };

    let time = {
        let mut a: Vec<&str> = boot_args.iter().map(|s| s.as_str()).collect();
        a.push("time");
        match run_analyze(&a) {
            Ok(t) => t,
            Err(e) => {
                incomplete.push(e);
                String::new()
            }
        }
    };
    let blame = {
        let mut a: Vec<&str> = boot_args.iter().map(|s| s.as_str()).collect();
        a.push("blame");
        match run_analyze(&a) {
            Ok(t) => t,
            Err(e) => {
                incomplete.push(e);
                String::new()
            }
        }
    };
    let chain = {
        let mut a: Vec<&str> = boot_args.iter().map(|s| s.as_str()).collect();
        a.push("critical-chain");
        match run_analyze(&a) {
            Ok(t) => t,
            Err(e) => {
                incomplete.push(e);
                String::new()
            }
        }
    };

    // Soft timeout documentation: Command has no built-in timeout on all platforms;
    // CI should use `--input`. Keep API simple.
    let _ = Duration::from_secs(30);

    let evidence = BootEvidence {
        boot_index,
        target: "multi-user.target".into(),
        timing: parse_time(&time),
        units: parse_blame(&blame),
        critical_chain: parse_critical_chain(&chain),
        failures: Vec::new(),
        initrd_bytes: discover_initrd_bytes(),
        capture_incomplete: !incomplete.is_empty(),
        incomplete_notes: incomplete,
        ..Default::default()
    };

    if evidence.timing.total_sec.is_none() && evidence.units.is_empty() {
        return Err(CaptureIncomplete::Message(
            "LIVE capture produced no usable analyze output; try --input fixtures or check privileges"
                .into(),
        ));
    }

    Ok(evidence)
}
