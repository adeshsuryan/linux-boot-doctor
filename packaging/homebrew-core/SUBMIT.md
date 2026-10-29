# Submitting `linux-boot-doctor` to Homebrew Core

**Goal:** appear on https://formulae.brew.sh/formula/linux-boot-doctor  

**Reality:** Core maintainers decide. New / niche tools are often asked to stay in a tap first. This draft is the correct *shape*; acceptance is not guaranteed.

Keep using the tap meanwhile:

```bash
brew tap adeshsuryan/tap
brew trust adeshsuryan/tap
brew install linux-boot-doctor
```

## What this formula does differently from the tap

| | Tap (`adeshsuryan/tap`) | Core draft (this folder) |
| --- | --- | --- |
| Install | Prebuilt GitHub Release tarball | **Build from source** with `cargo` |
| Bottles | Your Release assets | Built by **Homebrew CI** after merge |
| Index | Not on formulae.brew.sh | Listed if merged into homebrew-core |

## Pre-flight checklist

- [x] Public GitHub repo with MIT license
- [x] Annotated stable tag `v0.1.1` (and Release)
- [x] `Cargo.lock` committed
- [x] Formula builds with `cargo install --path crates/boot-doctor-cli`
- [x] `livecheck` via `:github_latest`
- [x] `test` block uses offline `--input` (no systemd required on macOS CI)
- [ ] Local audit passes (run below before opening the PR)
- [ ] You are ready for review comments / possible “use a tap” close

## Local dry-run (on a Mac with working Xcode / CLT)

```bash
# From a clean clone of homebrew-core (or use brew extract / brew create flow)
brew create --set-name linux-boot-doctor \
  https://github.com/adeshsuryan/linux-boot-doctor/archive/refs/tags/v0.1.1.tar.gz

# Replace the generated formula with packaging/homebrew-core/linux-boot-doctor.rb
# Then:
brew install --build-from-source linux-boot-doctor
brew test linux-boot-doctor
brew audit --new --formula linux-boot-doctor
brew style --fix linux-boot-doctor
```

If `brew audit --new` fails, fix every complaint before opening the PR.

## Open the PR

1. Fork https://github.com/Homebrew/homebrew-core  
2. Add `Formula/l/linux-boot-doctor.rb` (Homebrew groups by first letter)  
3. Copy contents from this repo’s `packaging/homebrew-core/linux-boot-doctor.rb`  
4. Commit as yourself only (Adesh Suryan) — no AI co-author trailers  
5. Open PR titled: `linux-boot-doctor 0.1.1 (new formula)`  
6. Body: one short paragraph what it does + homepage + that offline `--input` is the CI path  

Official docs:

- https://docs.brew.sh/Adding-Software-to-Homebrew  
- https://docs.brew.sh/Formula-Cookbook  

## After merge (if accepted)

- formulae.brew.sh will show the page automatically  
- Users install with: `brew install linux-boot-doctor` (no tap)  
- Bump formula on each release with `brew bump-formula-pr linux-boot-doctor --version=X.Y.Z`

## If rejected

Continue the tap. That is normal and still a proper Homebrew distribution channel.
