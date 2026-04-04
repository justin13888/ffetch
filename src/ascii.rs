//! ASCII art module for distro logos.
//!
//! This module provides access to ASCII art logos for various distributions.
//! The art is generated at build time from text files in `ascii/distros/`.

// Include the generated ASCII art module
include!(concat!(env!("OUT_DIR"), "/ascii_art.rs"));

/// Get the primary color for a given distro.
/// Returns a crossterm::style::Color for styling the ASCII art.
#[allow(clippy::if_same_then_else)]
pub fn get_distro_color(distro: &str) -> crossterm::style::Color {
    use crossterm::style::Color;
    let distro_lower = distro.to_lowercase();

    if distro_lower.contains("ubuntu") {
        Color::AnsiValue(208) // Orange
    } else if distro_lower.contains("arch") {
        Color::DarkCyan
    } else if distro_lower.contains("debian") {
        Color::DarkRed
    } else if distro_lower.contains("fedora") {
        Color::DarkBlue
    } else if distro_lower.contains("nixos") {
        Color::DarkCyan
    } else if distro_lower.contains("macos") || distro_lower.contains("darwin") {
        Color::White
    } else if distro_lower.contains("windows") {
        Color::DarkCyan
    } else if distro_lower.contains("manjaro") {
        Color::DarkGreen
    } else if distro_lower.contains("opensuse") || distro_lower.contains("suse") {
        Color::DarkGreen
    } else if distro_lower.contains("mint") {
        Color::DarkGreen
    } else if distro_lower.contains("gentoo") {
        Color::DarkMagenta
    } else if distro_lower.contains("void") {
        Color::DarkGreen
    } else if distro_lower.contains("alpine") {
        Color::DarkBlue
    } else if distro_lower.contains("endeavour") {
        Color::DarkMagenta
    } else if distro_lower.contains("pop") {
        Color::DarkCyan
    } else {
        Color::DarkBlue // Default
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
        use crossterm::style::Color;
        assert!(matches!(
            get_distro_color("Ubuntu"),
            Color::AnsiValue(208)
        ));
        assert!(matches!(
            get_distro_color("Arch Linux"),
            Color::DarkCyan
        ));
    }
}
