//! Kitty graphics protocol image output.
//!
//! Self-contained: PNG bytes are base64-encoded and transmitted in chunked APC
//! escapes. No image-decoding dependency — only PNG sources are supported, and
//! their dimensions are read straight from the IHDR chunk.

use std::io::{IsTerminal, Write};

use base64::Engine;

/// Whether the terminal is a Kitty-graphics-capable TTY.
pub fn kitty_supported() -> bool {
    if !std::io::stdout().is_terminal() {
        return false;
    }
    if std::env::var_os("KITTY_WINDOW_ID").is_some() {
        return true;
    }
    std::env::var("TERM")
        .map(|t| t.contains("kitty") || t.contains("ghostty"))
        .unwrap_or(false)
}

/// Read a PNG's pixel dimensions from its IHDR chunk, or `None` if `data` is
/// not a PNG.
pub fn png_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    // 8-byte signature, then IHDR: length(4) + "IHDR"(4) + width(4) + height(4).
    if data.len() < 24 || &data[1..4] != b"PNG" || &data[12..16] != b"IHDR" {
        return None;
    }
    let w = u32::from_be_bytes(data[16..20].try_into().ok()?);
    let h = u32::from_be_bytes(data[20..24].try_into().ok()?);
    Some((w, h))
}

/// Transmit and display `png` via the Kitty graphics protocol, sized to
/// `cols`x`rows` terminal cells at the current cursor position. The payload is
/// base64-encoded and split into 4 KiB chunks per the protocol.
pub fn display_png(w: &mut impl Write, png: &[u8], cols: u32, rows: u32) -> std::io::Result<()> {
    let b64 = base64::engine::general_purpose::STANDARD.encode(png);
    let bytes = b64.as_bytes();
    let n = bytes.chunks(4096).len();
    for (i, chunk) in bytes.chunks(4096).enumerate() {
        let more = u8::from(i + 1 < n);
        if i == 0 {
            write!(w, "\x1b_Ga=T,f=100,c={cols},r={rows},m={more};")?;
        } else {
            write!(w, "\x1b_Gm={more};")?;
        }
        w.write_all(chunk)?;
        write!(w, "\x1b\\")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_png() {
        assert!(png_dimensions(b"not a png at all really").is_none());
    }

    #[test]
    fn reads_ihdr_dimensions() {
        // Minimal PNG signature + IHDR header for a 16x9 image.
        let mut data = vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];
        data.extend_from_slice(&[0, 0, 0, 13]); // IHDR length
        data.extend_from_slice(b"IHDR");
        data.extend_from_slice(&16u32.to_be_bytes());
        data.extend_from_slice(&9u32.to_be_bytes());
        assert_eq!(png_dimensions(&data), Some((16, 9)));
    }
}
