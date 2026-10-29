# Project rules

- Author/committer: Adesh Suryan only. No AI co-author trailers.
- Never commit `access.txt`, tokens, or `.env`.
- Annotated tags `vX.Y.Z` must match `Cargo.toml` workspace version and `CHANGELOG.md`.
- Read-only product: no systemd mutations.
- Prefer `git commit-tree` if the IDE injects `Co-authored-by: Cursor`.
