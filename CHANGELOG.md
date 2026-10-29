# Changelog

## 0.1.1 — 2026-10-29

- Ship macOS GitHub Release artifacts (`darwin-arm64`, `darwin-amd64`) so Homebrew installs a binary without a local Rust/Xcode build
- Homebrew formula uses prebuilt tarballs on macOS and Linux

## 0.1.0 — 2026-10-29

- Initial release: Rust CLI `linux-boot-doctor check`
- Offline `--input` capture directories and LIVE systemd-analyze capture
- Text + JSON reports (`schemaVersion` 1) with budgets and `--fail-on`
- v1 rules for boot budgets, critical-chain, initrd, failures, and related heuristics
- Packaging: GitHub Releases (linux amd64/arm64), Homebrew tap formula, AUR PKGBUILD, COPR spec, Debian/PPA packaging skeletons
