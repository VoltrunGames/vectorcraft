# Offline builds and backups

Goal: keep building Tend's Vector even if upstream repositories or crates.io disappear.

1. **Own forks.** The code lives in `VoltrunGames/vectorcraft`; its git dependency `craft-libs` points at
   `VoltrunGames/craft-libs` (same pinned commits as upstream). Optional `craft-fonts` is forked as
   `VoltrunGames/craft-fonts`.
2. **Vendored dependencies.** Run the *Vendor dependencies* workflow (Actions tab). It publishes
   `tends-vector-vendor-<date>.tar.zst` as a release. To build offline:
   ```
   tar --zstd -xf tends-vector-vendor-<date>.tar.zst
   mkdir -p .cargo && cat vendor-config.toml >> .cargo/config.toml
   cargo build --release --offline -p vectorcraft
   ```
   Re-run the workflow whenever `Cargo.lock` changes.
3. **Backup outside GitHub.** On your own machine: `git clone --mirror https://github.com/VoltrunGames/vectorcraft`
   (and the same for `craft-libs`), refresh with `git remote update`. Keep the mirrors and the vendor
   archive on a drive and, if you like, push the mirrors to a second host (Codeberg, GitLab).
