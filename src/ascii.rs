//! ASCII art module for distro logos.
//!
//! This module provides access to ASCII art logos for various distributions.
//! The art is generated at build time from text files in `ascii/distros/`.

// Include the generated ASCII art module
include!(concat!(env!("OUT_DIR"), "/ascii_art.rs"));

/// Get the primary color for a given distro.
/// Returns a console::Color for styling the ASCII art.
#[allow(clippy::if_same_then_else)]
pub fn get_distro_color(distro: &str) -> console::Color {
    let distro_lower = distro.to_lowercase();

    if distro_lower.contains("ubuntu") {
        console::Color::Color256(208) // Orange
    } else if distro_lower.contains("arch") {
        console::Color::Cyan
    } else if distro_lower.contains("debian") {
        console::Color::Red
    } else if distro_lower.contains("fedora") {
        console::Color::Blue
    } else if distro_lower.contains("nixos") {
        console::Color::Cyan
    } else if distro_lower.contains("macos") || distro_lower.contains("darwin") {
        console::Color::White
    } else if distro_lower.contains("windows") {
        console::Color::Cyan
    } else if distro_lower.contains("manjaro") {
        console::Color::Green
    } else if distro_lower.contains("opensuse") || distro_lower.contains("suse") {
        console::Color::Green
    } else if distro_lower.contains("mint") {
        console::Color::Green
    } else if distro_lower.contains("gentoo") {
        console::Color::Magenta
    } else if distro_lower.contains("void") {
        console::Color::Green
    } else if distro_lower.contains("alpine") {
        console::Color::Blue
    } else if distro_lower.contains("endeavour") {
        console::Color::Magenta
    } else if distro_lower.contains("pop") {
        console::Color::Cyan
    } else {
        console::Color::Blue // Default
    }
}

/// Generate a filler line with the same width as the ASCII art.
pub fn get_filler(width: usize) -> String {
    " ".repeat(width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ascii_art_ubuntu() {
        let (art, width) = get_ascii_art("Ubuntu");
        assert!(!art.is_empty());
        assert!(width > 0);
    }

    #[test]
    fn test_get_ascii_art_fallback() {
        let (art, width) = get_ascii_art("unknown_distro_xyz");
        assert!(!art.is_empty());
        assert!(width > 0);
    }

    #[test]
    fn test_get_distro_color() {
        assert!(matches!(
            get_distro_color("Ubuntu"),
            console::Color::Color256(208)
        ));
        assert!(matches!(
            get_distro_color("Arch Linux"),
            console::Color::Cyan
        ));
    }
}
