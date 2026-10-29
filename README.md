# linux-boot-doctor

Read-only CLI that turns `systemd-analyze` evidence into **ranked findings**, **budgets**, and **CI JSON** — so slow boots fail the build before the image ships.

Not a shell wrapper. Proper Rust package (`boot-doctor-core` + `linux-boot-doctor` binary), MIT licensed.

## Install

### GitHub Releases (primary)

Download the linux `amd64` or `arm64` tarball from  
https://github.com/adeshsuryan/linux-boot-doctor/releases

```bash
tar -xzf linux-boot-doctor-*-linux-amd64.tar.gz
sudo install -m 755 linux-boot-doctor /usr/local/bin/
linux-boot-doctor --version
```

### Homebrew (tap)

```bash
brew tap adeshsuryan/tap
brew trust adeshsuryan/tap   # required once on newer Homebrew
brew install linux-boot-doctor
```

Prebuilt tarballs for Linux and macOS from GitHub Releases (no local compile). If Homebrew refuses because of an outdated Xcode install on the Mac, update/remove Xcode or install the `darwin-arm64` / `darwin-amd64` Release tarball manually. LIVE systemd capture still requires Linux.

### AUR (Arch)

```bash
# after package acceptance / local PKGBUILD
yay -S linux-boot-doctor
# or
makepkg -si   # from packaging/aur/
```

### Fedora COPR

```bash
# after COPR project is enabled
sudo dnf copr enable adeshsuryan/linux-boot-doctor
sudo dnf install linux-boot-doctor
```

Spec: `packaging/copr/linux-boot-doctor.spec`

### Ubuntu / Debian (PPA or .deb)

```bash
# Launchpad PPA (when published)
sudo add-apt-repository ppa:adeshsuryan/linux-boot-doctor
sudo apt update
sudo apt install linux-boot-doctor
```

Debian packaging sources: `packaging/debian/`

### Build from source

```bash
cargo install --path crates/boot-doctor-cli
```

## Quick start

```bash
# Offline / CI (any OS with the binary)
linux-boot-doctor check --input ./fixtures/slow-total --format text
linux-boot-doctor check --input ./fixtures/slow-total --format json --fail-on high

# LIVE on a Linux host with systemd
linux-boot-doctor check --max-total 8 --fail-on high
```

Cloud / appliance tip: use `--max-total 8` in image CI. Product default remains **30s** for general servers.

## Exit codes

| Code | Meaning |
| --- | --- |
| 0 | OK under `--fail-on` |
| 1 | Findings exceeded fail threshold |
| 2 | Usage / capture error |
| 3 | Unsupported environment (no systemd) for LIVE mode |

## What it is not

- Does not edit units or call `systemctl enable/disable`
- Does not replace `systemd-analyze` — it interprets it
- Does not diagnose PSI/OOM (separate tools)

## License

MIT — see [LICENSE](LICENSE)
