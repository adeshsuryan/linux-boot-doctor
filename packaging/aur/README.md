# AUR

1. After tag `v0.1.0` exists, set `sha256sums` in `PKGBUILD` from the GitHub source archive.
2. Publish to AUR as `linux-boot-doctor` (ssh to `aur.archlinux.org`).
3. Optional: also ship a binary package `linux-boot-doctor-bin` that downloads Release tarballs.

Local test:

```bash
makepkg -si
```
