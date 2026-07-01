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
| [release-plz](https://release-plz.dev) | version bump, `CHANGELOG.md`, git tag `v{version}`, crates.io publish |
| [dist (cargo-dist)](https://opensource.axo.dev/cargo-dist/) | GitHub Release: binaries for 5 targets + `shell`/`powershell`/`homebrew`/`msi` installers; bundles `man/purr.1` + `completions/` into every archive |

Merging the release-plz PR tags `v{version}`, which triggers cargo-dist's
`release.yml`. Everything below keys off that GitHub Release.

> **Note on cargo-dist and Linux packages.** cargo-dist produces archives and
> the `shell`/`powershell`/`homebrew`/`msi` installers, but it does **not** build
> `.deb` or `.rpm`. Those are produced by [`cargo-deb`](https://github.com/kornelski/cargo-deb)
> and [`cargo-generate-rpm`](https://github.com/cat-in-136/cargo-generate-rpm)
> (issue #8's note suggesting cargo-dist can emit them is incorrect).

## Channels

| Channel | Recipe | Built / submitted by | One-time manual setup |
|---------|--------|----------------------|------------------------|
| crates.io | `Cargo.toml` | release-plz | `CARGO_REGISTRY_TOKEN` secret |
| GitHub Release (bins, installers) | `[workspace.metadata.dist]` | cargo-dist `release.yml` | `RELEASE_PLZ_TOKEN` secret |
| Homebrew | cargo-dist `homebrew` installer | cargo-dist → `justin13888/homebrew-tap` | create the `homebrew-tap` repo + `HOMEBREW_TAP_TOKEN` secret |
| winget | `wix/main.wxs` (msi) | `.github/workflows/winget.yml` | fork `microsoft/winget-pkgs` + `WINGET_TOKEN` secret |
| `.deb` / `.rpm` (download) | `[package.metadata.deb]`, `[package.metadata.generate-rpm]` | `.github/workflows/package-linux.yml` | none (attached to the release) |
| Fedora COPR | `packaging/rpm/purrfetch.spec` + `.copr/Makefile` | COPR (build-from-git) | create COPR project `justin13888/purr` + webhook |
| AUR | `packaging/aur/purr-bin`, `packaging/aur/purr-git` | `.github/workflows/aur.yml` | AUR account + `AUR_SSH_PRIVATE_KEY` secret; create the empty packages |
| Alpine | `packaging/alpine/APKBUILD` | manual MR to `alpinelinux/aports` | Alpine developer account |
| Nix | `flake.nix` (repo root) | `nix run github:justin13888/purrfetch` | optional: submit to nixpkgs |

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
.github/workflows/{package-linux,winget,aur}.yml   # all `on: release`
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

Most channels need the **first GitHub Release to exist** before they can submit
(they consume release assets). Order:

1. **Secrets & tap.** Create the empty repo `justin13888/homebrew-tap`. Add repo
   secrets: `CARGO_REGISTRY_TOKEN` (crates.io), `RELEASE_PLZ_TOKEN` (PAT, repo +
   workflow), `HOMEBREW_TAP_TOKEN` (PAT that can push to the tap).
2. **First release.** Merge the release-plz PR → it tags `v1.0.0` → cargo-dist
   builds the release (tarballs, `.msi`, installers, Homebrew formula).
3. **winget.** Fork `microsoft/winget-pkgs`; add `WINGET_TOKEN` (classic PAT,
   `public_repo`). `winget.yml` then opens the PR automatically on each release.
4. **AUR.** Create an AUR account; add your SSH public key; create empty packages
   `purr-bin` and `purr-git`; add `AUR_SSH_PRIVATE_KEY` secret. `aur.yml` deploys
   on release.
5. **COPR.** Create project `justin13888/purr`; add a package built from this Git
   repo (`.copr/Makefile`); add the GitHub webhook for auto-rebuilds.
6. **(Optional) nixpkgs / Alpine aports.** Upstream the `flake.nix` derivation and
   `packaging/alpine/APKBUILD` via their respective review processes.

After the first release, confirm the `on: release` workflows ran (`.deb`/`.rpm`
uploaded, winget PR opened, AUR pushed) and iterate if any failed.
