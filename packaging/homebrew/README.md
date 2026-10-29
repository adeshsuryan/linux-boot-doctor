# Homebrew packaging

1. Publish GitHub Release `vX.Y.Z` with linux tarballs.
2. Compute `sha256` for each asset and for the source tag archive.
3. Copy `linux-boot-doctor.rb` into tap repo `adeshsuryan/homebrew-tap` as `Formula/linux-boot-doctor.rb` with sha256 filled in.
4. Users:

```bash
brew tap adeshsuryan/tap
brew install linux-boot-doctor
```

Create the tap once:

```bash
# empty public repo adeshsuryan/homebrew-tap with Formula/
```
