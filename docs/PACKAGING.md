# Distribution channels

| Channel | Status in 0.1.0 | Path |
| --- | --- | --- |
| GitHub Releases | Primary — CI `release.yml` on tag `v*` | `.github/workflows/release.yml` |
| Homebrew tap | Formula template + tap instructions | `packaging/homebrew/` |
| AUR | PKGBUILD ready for submit | `packaging/aur/` |
| Fedora COPR | RPM spec ready | `packaging/copr/` |
| Ubuntu/Debian PPA | debian/ skeleton | `packaging/debian/` |

All channels install the same Rust-built `linux-boot-doctor` binary (not a shell script).

After tagging `v0.1.0`:

1. Confirm Release assets + SHA256SUMS.
2. Fill Homebrew sha256 and push `adeshsuryan/homebrew-tap`.
3. Set AUR `sha256sums` and publish.
4. Trigger COPR SCM build.
5. Upload Launchpad source when PPA is created.
