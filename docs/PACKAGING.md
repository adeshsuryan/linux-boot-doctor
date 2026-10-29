# Distribution channels

| Channel | Status | Path |
| --- | --- | --- |
| GitHub Releases | Primary — CI `release.yml` on tag `v*` | `.github/workflows/release.yml` |
| Homebrew tap | Live — `adeshsuryan/tap` (prebuilt) | `packaging/homebrew/` |
| Homebrew Core | Draft for formulae.brew.sh PR (not merged) | `packaging/homebrew-core/` |
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
