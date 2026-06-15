//! Runtime expansion of `${c1}`..`${c6}` colour markers in distro ASCII art.
//!
//! Markers are kept verbatim in the build-time-generated art (see `build.rs`)
//! and expanded here against a 6-slot palette, mirroring neofetch's `color()`
//! so a logo can be recoloured at render time (`ascii_colors`, `ascii_bold`).

/// ANSI SGR for a neofetch palette value, mirroring neofetch's `color()`:
/// 0-6 → standard foreground (`\e[3Nm`), 7 → default white (`\e[37m`),
/// 8+ → 256-colour (`\e[38;5;Nm`). Prepends bold when requested.
fn ansi_for(value: u8, bold: bool) -> String {
    let b = if bold { "\x1b[1m" } else { "" };
    match value {
        0..=6 => format!("{b}\x1b[3{value}m"),
        7 => format!("{b}\x1b[37m"),
        _ => format!("{b}\x1b[38;5;{value}m"),
    }
}

/// Expand `${c1}`..`${c6}` in `line` using `palette` and `bold`, returning the
/// line with ANSI escapes and a trailing reset. Lines without markers are
/// returned unchanged (the renderer colours them with the base logo colour).
pub fn expand(line: &str, palette: &[u8; 6], bold: bool) -> String {
    if !line.contains("${c") {
        return line.to_string();
    }
    let mut out = String::with_capacity(line.len() + 16);
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '$' && chars.peek() == Some(&'{') {
            chars.next(); // consume '{'
            let mut tag = String::new();
            for n in chars.by_ref() {
                if n == '}' {
                    break;
                }
                tag.push(n);
            }
            if let Some(idx) = tag.strip_prefix('c').and_then(|s| s.parse::<usize>().ok())
                && (1..=6).contains(&idx)
            {
                out.push_str(&ansi_for(palette[idx - 1], bold));
            }
        } else {
            out.push(c);
        }
    }
    out.push_str("\x1b[0m");
    out
}

/// Remove `${c1}`..`${c6}` markers without emitting any colour (for NO_COLOR).
pub fn strip(line: &str) -> String {
    if !line.contains("${c") {
        return line.to_string();
    }
    let mut out = String::with_capacity(line.len());
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '$' && chars.peek() == Some(&'{') {
            chars.next();
            for n in chars.by_ref() {
                if n == '}' {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_markers_to_plain() {
        assert_eq!(strip("${c1}A${c2}B"), "AB");
    }

    #[test]
    fn no_markers_unchanged() {
        assert_eq!(
            expand("  plain art  ", &[4, 7, 4, 4, 4, 4], false),
            "  plain art  "
        );
    }

    #[test]
    fn expands_markers_with_palette() {
        // c1 = slot 0 (value 12 -> 256-colour), c2 = slot 1 (value 7 -> \e[37m).
        let out = expand("${c1}A${c2}B", &[12, 7, 4, 4, 4, 4], false);
        assert_eq!(out, "\x1b[38;5;12mA\x1b[37mB\x1b[0m");
    }

    #[test]
    fn low_values_use_standard_fg() {
        let out = expand("${c1}X", &[4, 7, 4, 4, 4, 4], false);
        assert_eq!(out, "\x1b[34mX\x1b[0m");
    }
}
