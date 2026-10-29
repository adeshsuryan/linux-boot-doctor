# Security

`linux-boot-doctor` is a **read-only** CLI. It runs `systemd-analyze` (and reads optional capture files) and prints findings. It does **not**:

- modify unit files or systemd state
- enable, disable, start, or stop units
- install packages
- phone home or send telemetry
- parse unit file bodies / `Environment=` secrets

## Privileges

| Mode | Typical privilege | Notes |
| --- | --- | --- |
| `--input DIR` | none | Preferred for CI on any OS |
| LIVE `check` | often unprivileged | Distro-dependent |
| `--boot N` for older boots | may need root | Never auto-elevates |

## Reports

JSON/text may include unit names, timings, and failure messages. Treat CI artifacts from customer images as sensitive.

## Dependencies

Runtime crates: `clap`, `serde`, `serde_json`, `regex`, `thiserror` (see `Cargo.lock`). New dependencies require a CHANGELOG line and an update here.

## Reporting

Open a private GitHub security advisory on https://github.com/adeshsuryan/linux-boot-doctor
