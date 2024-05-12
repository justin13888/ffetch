# ffetch

Fast, featureful, cross-platform fetching tool written in Rust.

Perfect for sharing your [rice](https://www.reddit.com/r/unixporn/) or showing stats on terminal startup.

<!-- TODO: Build Status -->
<!-- TODO: Include preview.png -->

## Why ffetch?

- **Fast**: ffetch is designed to be fast, leverages asynchronous I/O with Rust
- **Cross-platform**: Covers all three major platforms (Linux, macOS, Windows) and more
- **Feature-complete**: Includes all features from other tools like `neofetch`, `pfetch`, etc.
- **Highly customizable**: ffetch provides a wide range of customization options, including themes and ability to replicate various fetch tools.
- **Modern replacement for neofetch**: ffetch is a modern replacement for [neofetch](https://github.com/dylanaraps/neofetch) with more features and better performance
- **Focus on first-class support on all platforms**: ffetch aims to provide first-class support on all desktop platforms, including Windows, macOS, and Linux. (Other platforms may be supported given demand.)

## Installation

### Cargo

```bash
cargo install ffetch
```

<!-- ### Alpine Linux -->

<!-- TODO: Support Alpine -->

### Arch Linux

Use your favorite AUR helper to install `ffetch-git` from the AUR.

```bash
paru -S ffetch-git
```

### Debian/Ubuntu and derivatives

<!-- Install .deb? -->

### Fedora

Install via the COPR repository:

```bash
sudo dnf copr enable justin13888/ffetch
sudo dnf install ffetch
```

<!-- ### NixOS -->

<!-- TODO: Support Nix -->

<!-- ### Homebrew (MacOS)

```bash
brew install justin13888/ffetch
``` -->
<!-- TODO: Setup homebrew -->

<!-- ### Winget (Windows)

```powershell
winget install justin13888.ffetch
``` -->
<!-- TODO: Setup winget -->

### Git

Note: This method is suggested for one of the following reasons:

1. Latest `ffetch` version
2. Native package manager is unsupported or not preferred

To install via Git, follow these steps:
1. Clone this repository.
2. Run `cargo install --path .` in the repository root.

## Development

- Clone this repository
- Run `cargo build` to build the project
- Use `cargo run` to run the project

## Packaging

<!-- TODO: Include repology widget of all repo version states -->

## FAQ

Q: Why did you write another fetch tool?
A: It's feature-rich, fast, and written in a memory-safe language (Rust). The goal is to make it a modern, well-maintained replacement for neofetch and more.

Q: Why not contribute to an existing fetch tool?
A: I want to start from a clean state, including all the features the community wants, and make it truly universally supported and deployable to all common platforms.

Q: What does ffetch use to fetch metrics under the hood?
A: ffetch uses a modified version of `libmacchina` crate for majority of system-related info.

## Issues

If you encounter any issues, please open an issue on the GitHub repository.

## License
