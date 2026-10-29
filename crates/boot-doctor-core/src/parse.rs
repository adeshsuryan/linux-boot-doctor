use regex::Regex;
use std::sync::OnceLock;

use crate::evidence::{BootTiming, ChainNode, UnitFailure, UnitTiming};

fn duration_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?i)(\d+(?:\.\d+)?)(s|ms)").expect("duration regex"))
}

fn phase_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)(?:(\d+(?:\.\d+)?)s\s*)?(?:(\d+)ms\s*)?\((firmware|loader|kernel|initrd|userspace)\)")
            .expect("phase regex")
    })
}

fn blame_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?m)^\s*((?:\d+(?:\.\d+)?s)?(?:\d+ms)?)\s+(\S+)\s*$").expect("blame"))
}

fn chain_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"([A-Za-z0-9_@.:\\-]+)\s+@([0-9]+(?:\.[0-9]+)?(?:s|ms))(?:\s+\+([0-9]+(?:\.[0-9]+)?(?:s|ms)))?")
            .expect("chain")
    })
}

/// Parse a systemd duration token like `1.2s`, `512ms`, or `1s200ms`.
pub fn parse_duration(token: &str) -> f64 {
    let mut sec = 0.0;
    for caps in duration_re().captures_iter(token) {
        let n: f64 = caps[1].parse().unwrap_or(0.0);
        match &caps[2].to_ascii_lowercase()[..] {
            "ms" => sec += n / 1000.0,
            _ => sec += n,
        }
    }
    sec
}

fn set_phase(timing: &mut BootTiming, name: &str, value: f64) {
    match name.to_ascii_lowercase().as_str() {
        "firmware" => timing.firmware_sec = Some(value),
        "loader" => timing.loader_sec = Some(value),
        "kernel" => timing.kernel_sec = Some(value),
        "initrd" => timing.initrd_sec = Some(value),
        "userspace" => timing.userspace_sec = Some(value),
        _ => {}
    }
}

pub fn parse_time(text: &str) -> BootTiming {
    let mut timing = BootTiming::default();

    // Full: Startup finished in A + B = TOTAL
    if let Some(caps) = Regex::new(
        r"(?is)Startup finished in\s+(.+?)\s*=\s*((?:\d+(?:\.\d+)?s\s*)?(?:\d+ms\s*)*)",
    )
    .ok()
    .and_then(|re| re.captures(text))
    {
        timing.total_sec = Some(parse_duration(&caps[2]));
        for m in phase_re().captures_iter(&caps[1]) {
            let mut part = String::new();
            if let Some(s) = m.get(1) {
                part.push_str(s.as_str());
                part.push('s');
            }
            if let Some(ms) = m.get(2) {
                part.push_str(ms.as_str());
                part.push_str("ms");
            }
            set_phase(&mut timing, &m[3], parse_duration(&part));
        }
    }

    // Container-only: Startup finished in 267ms (userspace)
    if timing.userspace_sec.is_none() {
        if let Some(caps) = Regex::new(
            r"(?im)Startup finished in\s+((?:\d+(?:\.\d+)?s\s*)?(?:\d+ms\s*)*)\((userspace)\)\s*$",
        )
        .ok()
        .and_then(|re| re.captures(text))
        {
            let v = parse_duration(&caps[1]);
            timing.userspace_sec = Some(v);
            if timing.total_sec.is_none() {
                timing.total_sec = Some(v);
            }
        }
    }

    // Fallback: apply any phase tokens in the whole text
    if timing.firmware_sec.is_none()
        && timing.loader_sec.is_none()
        && timing.kernel_sec.is_none()
        && timing.initrd_sec.is_none()
        && timing.userspace_sec.is_none()
    {
        for m in phase_re().captures_iter(text) {
            let mut part = String::new();
            if let Some(s) = m.get(1) {
                part.push_str(s.as_str());
                part.push('s');
            }
            if let Some(ms) = m.get(2) {
                part.push_str(ms.as_str());
                part.push_str("ms");
            }
            set_phase(&mut timing, &m[3], parse_duration(&part));
        }
    }

    if timing.total_sec.is_none() {
        let sum = [
            timing.firmware_sec,
            timing.loader_sec,
            timing.kernel_sec,
            timing.initrd_sec,
            timing.userspace_sec,
        ]
        .into_iter()
        .flatten()
        .sum::<f64>();
        if sum > 0.0 {
            timing.total_sec = Some(sum);
        }
    }

    timing
}

pub fn parse_blame(text: &str) -> Vec<UnitTiming> {
    let mut units = Vec::new();
    for caps in blame_re().captures_iter(text) {
        let time = parse_duration(&caps[1]);
        let name = caps[2].to_string();
        if !name.is_empty() {
            units.push(UnitTiming {
                name,
                time_sec: time,
            });
        }
    }
    units
}

pub fn parse_critical_chain(text: &str) -> Vec<ChainNode> {
    let mut nodes = Vec::new();
    for caps in chain_re().captures_iter(text) {
        let start = caps.get(3).map(|m| parse_duration(m.as_str()));
        nodes.push(ChainNode {
            unit: caps[1].to_string(),
            active_at_sec: parse_duration(&caps[2]),
            start_sec: start,
        });
    }
    nodes
}

pub fn parse_verify(text: &str) -> Vec<UnitFailure> {
    let mut failures = Vec::new();
    for line in text.lines() {
        let lower = line.to_ascii_lowercase();
        if !(lower.contains("failed") || lower.contains("error")) {
            continue;
        }
        let unit = line
            .split(':')
            .next()
            .unwrap_or("unknown")
            .trim()
            .to_string();
        failures.push(UnitFailure {
            unit,
            result: "failed".to_string(),
            message: Some(line.trim().to_string()),
        });
    }
    failures
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_ms_and_s() {
        assert!((parse_duration("267ms") - 0.267).abs() < 1e-9);
        assert!((parse_duration("1.5s") - 1.5).abs() < 1e-9);
        assert!((parse_duration("1s200ms") - 1.2).abs() < 1e-9);
    }

    #[test]
    fn parse_full_time_line() {
        let t = parse_time(
            "Startup finished in 3.142s (firmware) + 412ms (loader) + 1.891s (kernel) + 2.104s (initrd) + 28.442s (userspace) = 35.991s\n",
        );
        assert!((t.total_sec.unwrap() - 35.991).abs() < 1e-6);
        assert!((t.userspace_sec.unwrap() - 28.442).abs() < 1e-6);
        assert!((t.firmware_sec.unwrap() - 3.142).abs() < 1e-6);
    }

    #[test]
    fn parse_container_userspace_only() {
        let t = parse_time("Startup finished in 267ms (userspace)\n");
        assert!((t.total_sec.unwrap() - 0.267).abs() < 1e-9);
        assert!((t.userspace_sec.unwrap() - 0.267).abs() < 1e-9);
    }
}
