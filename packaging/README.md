# Packaging

How `purr` is built and distributed, what is automated, and the one-time
owner-only steps each external channel needs. This is the source of truth for
the packaging system; the recipes themselves live alongside this file (and a few
at the repo root where the tools require it).

The crate is **`purrfetch`** (crates.io); the installed **binary/command is
`purr`**. ASCII art is baked into the binary at compile time, so packages only
need to ship the `purr` binary, `man/purr.1`, the `completions/`, and `LICENSE`.

## Release pipeline (already in place)

| Tool | Owns |
|------|------|
| [release-plz](https://release-plz.dev) | version bump, `CHANGELOG.md`, and the "release" PR (runs on the default `GITHUB_TOKEN` — no PAT, no crates.io token) |
| [dist (cargo-dist)](https://opensource.axo.dev/cargo-dist/) | GitHub Release: binaries for 5 targets + `shell`/`powershell`/`homebrew`/`msi` installers; bundles `man/purr.1` + `completions/` into every archive |

The release is **cut manually** after merging the release-plz PR: the owner pushes the
`v{version}` tag and publishes to crates.io locally (see the runbook below). The tag
push triggers cargo-dist's `release.yml`; `.deb`/`.rpm`/winget then chain off that
workflow **completing** — a cargo-dist release is created with `GITHUB_TOKEN`, which
does **not** trigger `on: release`, so those workflows use `on: workflow_run` instead.

> **Note on cargo-dist and Linux packages.** cargo-dist produces archives and
> the `shell`/`powershell`/`homebrew`/`msi` installers, but it does **not** build
> `.deb` or `.rpm`. Those are produced by [`cargo-deb`](https://github.com/kornelski/cargo-deb)
> and [`cargo-generate-rpm`](https://github.com/cat-in-136/cargo-generate-rpm)
> (issue #8's note suggesting cargo-dist can emit them is incorrect).

## Channels

| Channel | Recipe | Built / submitted by | One-time manual setup |
|---------|--------|----------------------|------------------------|
| crates.io | `Cargo.toml` | manual local `cargo publish` (→ Trusted Publishing) | `cargo login` locally — **no CI secret** |
| GitHub Release (bins, installers) | `[workspace.metadata.dist]` | cargo-dist `release.yml` | none — owner pushes the tag locally |
| Homebrew | cargo-dist `homebrew` installer | cargo-dist → `justin13888/homebrew-tap` | ✅ tap repo + `HOMEBREW_TAP_TOKEN` (done) |
| winget | `wix/main.wxs` (msi) | `winget.yml` (chains on `workflow_run: Release`) | ✅ `winget-pkgs` fork + `WINGET_TOKEN` (done) |
| `.deb` / `.rpm` (download) | `[package.metadata.deb]`, `[package.metadata.generate-rpm]` | `package-linux.yml` (chains on `workflow_run: Release`) | none (attached to the release) |
| Fedora COPR — *deferred* | `packaging/rpm/purrfetch.spec` + `.copr/Makefile` | COPR (build-from-git) | create COPR project `justin13888/purr` + webhook |
| AUR — *deferred* | `packaging/aur/purr-bin`, `packaging/aur/purr-git` | `aur.yml` (chains on `workflow_run: Release`) | AUR account + `AUR_SSH_PRIVATE_KEY` secret; create the empty packages |
| Alpine — *deferred* | `packaging/alpine/APKBUILD` | manual MR to `alpinelinux/aports` | Alpine developer account |
| Nix | `flake.nix` (repo root) | `nix run github:justin13888/purrfetch` | ✅ works today; optional: submit to nixpkgs |

## Layout

```
Cargo.toml                       # [workspace.metadata.dist], [package.metadata.{deb,generate-rpm,wix}]
wix/main.wxs                     # generated msi definition (dist generate)
flake.nix, flake.lock            # repo root (flakes must be at root)
.copr/Makefile                   # repo root (COPR convention)
completions/{purr.bash,_purr,purr.fish}
packaging/
  README.md                      # this file
  aur/purr-bin/{PKGBUILD,.SRCINFO}
  aur/purr-git/{PKGBUILD,.SRCINFO}
  alpine/APKBUILD
  rpm/purrfetch.spec
.github/workflows/{package-linux,winget,aur}.yml   # chain on `workflow_run: Release` (aur deferred)
```

## Self-verification (podman)

Each recipe is validated locally in a throwaway container — no distro tooling on
the host required. From the repo root:

```bash
# .deb / .rpm — build with cargo subcommands, install-test in containers
cargo install cargo-deb cargo-generate-rpm
cargo build --release
cargo deb --no-build && cargo generate-rpm
podman run --rm -v "$PWD":/w:Z -w /w debian:stable \
  sh -c 'dpkg -i target/debian/*.deb && purr --version && man -w purr'
podman run --rm -v "$PWD":/w:Z -w /w fedora:latest \
  sh -c 'dnf install -y ./target/generate-rpm/*.rpm && purr --version'

# Fedora COPR spec
podman run --rm -v "$PWD":/w:Z -w /w fedora:latest \
  sh -c 'dnf install -y rpmlint rpm-build && rpmlint packaging/rpm/purrfetch.spec'

# AUR (non-root build user; namcap + shellcheck + .SRCINFO drift)
podman run --rm -v "$PWD":/w:Z -w /w archlinux:base-devel \
  sh -c 'pacman -Sy --noconfirm namcap shellcheck >/dev/null &&
         useradd -m build && cp -r packaging/aur/purr-git /tmp/p && chown -R build /tmp/p &&
         su build -c "cd /tmp/p && shellcheck PKGBUILD && namcap PKGBUILD &&
                      makepkg --printsrcinfo | diff - .SRCINFO"'

# Alpine APKBUILD
podman run --rm -v "$PWD":/w:Z -w /w alpine:edge \
  sh -c 'apk add atools shellcheck >/dev/null && cd packaging/alpine &&
         shellcheck -s ash APKBUILD; apkbuild-lint APKBUILD'

# Nix flake
podman run --rm -v "$PWD":/w:Z -w /w nixos/nix \
  sh -c 'nix --extra-experimental-features "nix-command flakes" flake check &&
         nix --extra-experimental-features "nix-command flakes" build .#default &&
         ./result/bin/purr --version'

# Workflows
podman run --rm -v "$PWD":/repo:Z -w /repo rhysd/actionlint:latest
```

## Manual runbook (owner-only)

The pipeline is **token-free**: no PAT and no crates.io token live in CI. The owner
cuts each release locally.

**One-time setup**
- ✅ `justin13888/homebrew-tap` repo + `HOMEBREW_TAP_TOKEN` secret — done.
- ✅ `microsoft/winget-pkgs` fork + `WINGET_TOKEN` secret — done.
  - ⚠️ **First submission is manual.** `winget.yml` uses `winget-releaser`, which only
    *updates* a package that already exists in `winget-pkgs`. Submit the initial
    `justin13888.purr` manifest once (e.g. `wingetcreate new`) so future releases can
    auto-update. Until that lands upstream, `winget.yml` self-skips instead of failing
    the release.
- ✅ Repo setting *Settings → Actions → General → "Allow GitHub Actions to create and
  approve pull requests"* — enabled (lets release-plz open the PR on `GITHUB_TOKEN`).
- `cargo login` on your machine with a crates.io token so the manual `cargo publish`
  works. The token stays local; it is never added to CI.

**Cutting a release**
1. A push to `master` opens/updates the release-plz PR (version bump + `CHANGELOG.md`).
2. Merge that PR.
3. Locally, cut the release:
   ```bash
   git checkout master && git pull
   git tag v1.0.0
   git push origin v1.0.0   # human push → triggers cargo-dist's release.yml
   cargo publish            # crates.io (last; until Trusted Publishing)
   ```
   cargo-dist builds the GitHub Release (tarballs, `.msi`, installers, Homebrew
   formula → tap). When it finishes, `package-linux.yml` (`.deb`/`.rpm`) and
   `winget.yml` chain off it via `workflow_run`.
4. Confirm the cascade: `.deb`/`.rpm` attached to the release, winget PR opened, tap
   updated. If a chained job didn't run, re-run it via its `workflow_dispatch`, e.g.
   `gh workflow run package-linux.yml -f release-tag=v1.0.0`.

**Follow-ups (optional / deferred)**
- **Trusted Publishing.** Enable crates.io Trusted Publishing (OIDC) for the repo, then
  add the tokenless publish job at the `TODO(trusted-publishing)` marker in
  `release-plz.yml` and drop the manual `cargo publish`.
- **AUR.** Create an AUR account; add your SSH public key; create empty packages
  `purr-bin` and `purr-git`; add the `AUR_SSH_PRIVATE_KEY` secret. `aur.yml` then
  deploys on each release (no workflow edit needed — it already chains on Release).
- **COPR.** Create project `justin13888/purr`; add a package built from this Git repo
  (`.copr/Makefile`); add the GitHub webhook for auto-rebuilds.
- **nixpkgs / Alpine aports.** Upstream the `flake.nix` derivation and
  `packaging/alpine/APKBUILD` via their respective review processes.
