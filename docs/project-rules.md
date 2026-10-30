# Project rules

- Author/committer: Adesh Suryan only. Sole author; no co-author trailers on commits.
- Never commit `access.txt`, tokens, or `.env`.
- Annotated tags `vX.Y.Z` must match `Cargo.toml` workspace version and `CHANGELOG.md`.
- Read-only product: no systemd mutations.
- If an IDE appends unwanted `Co-authored-by` trailers, rewrite the commit with `git commit-tree` (message file without those lines).
