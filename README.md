# purr

Fast, universal, cross-platform fetching tool written in Rust.

Perfect for sharing your [rice](https://www.reddit.com/r/unixporn/) or showing stats on terminal startup.

<!-- TODO: Build Status -->
<!-- TODO: Include preview.png -->

## Why purr?

- **Fast**: purr is designed to be fast, leverages asynchronous I/O with Rust
- **Cross-platform**: Covers all three major platforms (Linux, macOS, Windows) and more
- **Feature-complete**: Includes all features from other tools like `neofetch`, `pfetch`, etc.
- **Highly customizable**: purr provides a wide range of customization options, including themes and ability to replicate various fetch tools.
- **Modern replacement for neofetch**: purr is a modern replacement for [neofetch](https://github.com/dylanaraps/neofetch) with more features and better performance (negligible for fetch tools but nice to know)
- **Focus on first-class support on all platforms**: purr aims to provide first-class support on all desktop platforms, including Windows, macOS, and Linux. It is distributed as many native package managers.

## Installation

### Cargo

```bash
cargo install --locked purrfetch
```

<!-- ### Alpine Linux -->

<!-- TODO: Support Alpine -->

### Arch Linux

Use your favorite AUR helper to install `purr-git` from the AUR.

```bash
paru -S purr-git
```

### Debian/Ubuntu and derivatives

<!-- Install .deb? -->

### Fedora

Install via the COPR repository:

```bash
sudo dnf copr enable justin13888/purr
sudo dnf install purr
```

<!-- ### NixOS -->

<!-- TODO: Support Nix -->

<!-- ### Homebrew (MacOS)

```bash
brew install justin13888/purr
``` -->
<!-- TODO: Setup homebrew -->

<!-- ### Winget (Windows)

```powershell
winget install justin13888.purr
``` -->
<!-- TODO: Setup winget -->

### Git

Note: This method is suggested for one of the following reasons:

1. Latest `purr` version
2. Native package manager is unsupported or not preferred

To install via Git, follow these steps:
1. Clone this repository.
2. Run `cargo install --path .` in the repository root.

## Development

- Clone this repository
- Run `cargo build` to build the project
- Use `cargo run` to run the project

### Git Hooks

This project uses [lefthook](https://github.com/evilmartians/lefthook) to manage git hooks.

Install lefthook and set up the hooks:

```bash
lefthook install
```

### Benchmarking

#### End-to-end comparison (hyperfine)

Requires [hyperfine](https://github.com/sharkdp/hyperfine). Compares purr against any of neofetch, macchina, and fastfetch that are installed.

```bash
bash scripts/bench-compare.sh           # warm benchmark
bash scripts/bench-compare.sh --cold    # also cold-cache (requires sudo)
```

Results are written to `bench-results.json` and `bench-results.md`.

#### Probe microbenchmarks (criterion)

Benchmarks each probe individually, grouped by expected cost (fast, I/O-heavy, subprocess). Also measures the cold construction cost of each `libmacchina` readout.

```bash
cargo bench
```

HTML reports are written to `target/criterion/`.

#### Runtime profiling

**Tracing spans** — prints per-probe and per-subprocess timing at `debug` level:

```bash
RUST_LOG=debug cargo run --release -- --all
```

**Chrome trace** — produces `purr-trace.json` viewable in [Perfetto](https://ui.perfetto.dev):

```bash
cargo run --release --features profile -- --all
```

**Flamegraph** — requires `cargo install flamegraph` and `perf` (Linux):

```bash
cargo flamegraph --profile profiling -- --all
```

## Packaging

<!-- TODO: Include repology widget of all repo version states -->

## FAQ

Q: Why did you write another fetch tool?
A: It's feature-rich, fast, and written in a memory-safe language (Rust). The goal is to make it a modern, well-maintained replacement for neofetch and more.

Q: Why not contribute to an existing fetch tool?
A: I want to start from a clean state, including all the features the community wants, and make it truly universally supported and deployable to all common platforms.

Q: What does purr use to fetch metrics under the hood?
A: purr uses a modified version of `libmacchina` crate for majority of system-related info.

## Issues

If you encounter any issues, please open an issue on the GitHub repository.

## Contributing

Feel free to submit an issue or PR on GitHub.

> Notice: Looking for submissions/suggestions of new ASCII arts: <https://github.com/justin13888/purr/issues/1>

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
