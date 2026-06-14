use std::io::{IsTerminal, Write};

use crossterm::{
    cursor, execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal,
};
use tracing::debug;

use crate::{
    ascii::{get_ascii_art, get_distro_color, get_filler},
    config::NeofetchRendererConfig,
    probe::{ProbeList, ProbeResultValue, general_readout},
};

use super::{RendererError, execute_probes_streaming};

pub struct NeofetchRenderer {
    config: NeofetchRendererConfig,
    probe_list: ProbeList,
}

impl Default for NeofetchRenderer {
    fn default() -> Self {
        Self::new(NeofetchRendererConfig::default())
    }
}

impl NeofetchRenderer {
    pub fn new(config: NeofetchRendererConfig) -> Self {
        let probe_list = config
            .probes
            .iter()
            .map(|p| p.get_funcs())
            .collect::<Vec<_>>();
        Self { config, probe_list }
    }

    pub fn draw(&self) -> Result<(), RendererError> {
        use libmacchina::traits::GeneralReadout as _;

        let distro = general_readout()
            .distribution()
            .or_else(|_| general_readout().os_name())
            .unwrap_or_else(|_| "Linux".to_string());

        debug!("Detected distro: {}", distro);

        let (ascii_art, ascii_width) = get_ascii_art(&distro);
        let primary_color = get_distro_color(&distro);
        let filler = get_filler(ascii_width);
        let get_art =
            |idx: usize| -> &str { ascii_art.get(idx).copied().unwrap_or(filler.as_str()) };

        let max_title_len = self
            .probe_list
            .iter()
            .map(|(title, _)| title.len())
            .max()
            .unwrap_or(0);

        let stdout = std::io::stdout();
        let is_tty = stdout.is_terminal();
        let mut w = std::io::BufWriter::new(stdout.lock());

        let mut art_idx = 0usize;
        let mut title_len = 0;

        // Print title (username@hostname)
        if self.config.title {
            let username = general_readout().username()?;
            let hostname = general_readout().hostname()?;
            title_len = username.len() + hostname.len() + 1;
            queue!(
                w,
                SetForegroundColor(primary_color),
                Print(get_art(art_idx)),
                ResetColor,
                Print("   "),
                SetForegroundColor(primary_color),
                Print(&username),
                ResetColor,
                Print("@"),
                SetForegroundColor(primary_color),
                Print(&hostname),
                ResetColor,
                Print("\n"),
            )?;
            art_idx += 1;
        }

        // Print underline
        if self.config.underline {
            queue!(
                w,
                SetForegroundColor(primary_color),
                Print(get_art(art_idx)),
                ResetColor,
                Print("   "),
                Print("-".repeat(title_len)),
                Print("\n"),
            )?;
            art_idx += 1;
        }

        let probe_art_start = art_idx;
        let n_probes = self.probe_list.len();
        // Column (0-based) where values start: ascii + "   " + padded_title + " "
        let value_col = (ascii_width + 3 + max_title_len + 2) as u16;
        // Per-probe config, aligned with `probe_list` by index, for option-aware formatting.
        let probes = &self.config.probes;

        if !is_tty {
            // Non-TTY: run probes in parallel, print results sequentially
            let mut all_results: Vec<Option<Vec<String>>> = vec![None; n_probes];
            execute_probes_streaming(&self.probe_list, |index, _, result| {
                all_results[index] = Some(match result {
                    Some(ProbeResultValue::Single(v)) => vec![probes[index].format_value(&v)],
                    Some(ProbeResultValue::Multiple(vs)) => {
                        vs.iter().map(|v| probes[index].format_value(v)).collect()
                    }
                    None => vec![],
                });
            });

            for (i, (title, _)) in self.probe_list.iter().enumerate() {
                let strings = match all_results[i].as_deref() {
                    Some([]) | None => {
                        debug!("Error while probing {}", title);
                        continue;
                    }
                    Some(ss) => ss.to_vec(),
                };
                let padded_title = format!("{:width$}:", title, width = max_title_len);
                for s in strings.iter() {
                    // Repeat the label on every line (e.g. one "GPU:" per GPU),
                    // matching neofetch rather than leaving orphaned values.
                    queue!(
                        w,
                        SetForegroundColor(primary_color),
                        Print(get_art(art_idx)),
                        ResetColor,
                        Print("   "),
                        SetForegroundColor(primary_color),
                        Print(&padded_title),
                        ResetColor,
                        Print(" "),
                        Print(s),
                        Print("\n"),
                    )?;
                    art_idx += 1;
                }
            }
        } else {
            // TTY: progressive rendering with cursor movement

            // Phase 1: print all placeholder lines immediately
            for (i, (title, _)) in self.probe_list.iter().enumerate() {
                let padded_title = format!("{:width$}:", title, width = max_title_len);
                queue!(
                    w,
                    SetForegroundColor(primary_color),
                    Print(get_art(probe_art_start + i)),
                    ResetColor,
                    Print("   "),
                    SetForegroundColor(primary_color),
                    Print(padded_title),
                    ResetColor,
                    Print(" \n"),
                )?;
            }
            w.flush()?;
            // Save cursor position at the bottom of the probe section
            execute!(w, cursor::SavePosition)?;

            // Phase 2: fill in values as each probe completes
            let mut results: Vec<Option<Vec<String>>> = vec![None; n_probes];
            let mut needs_rerender = false;

            execute_probes_streaming(&self.probe_list, |index, _label, result| {
                let strings: Vec<String> = match result {
                    Some(ProbeResultValue::Single(v)) => vec![probes[index].format_value(&v)],
                    Some(ProbeResultValue::Multiple(vs)) => {
                        vs.iter().map(|v| probes[index].format_value(v)).collect()
                    }
                    None => vec![],
                };

                if strings.len() == 1 {
                    // Single value: move cursor to the right line and fill in
                    let lines_up = (n_probes - index) as u16;
                    let _ = execute!(
                        w,
                        cursor::RestorePosition,
                        cursor::MoveUp(lines_up),
                        cursor::MoveToColumn(value_col),
                        Print(&strings[0]),
                        cursor::RestorePosition,
                    );
                } else {
                    // Zero (failure) or multiple values: needs a re-render pass
                    needs_rerender = true;
                }

                results[index] = Some(strings);
            });

            // Phase 3: if any probe had 0 or multiple lines, re-render the probe section
            if needs_rerender {
                execute!(
                    w,
                    cursor::RestorePosition,
                    cursor::MoveUp(n_probes as u16),
                    cursor::MoveToColumn(0),
                    terminal::Clear(terminal::ClearType::FromCursorDown),
                )?;
                let mut ra_idx = probe_art_start;
                for (i, (title, _)) in self.probe_list.iter().enumerate() {
                    let strings = match results[i].as_deref() {
                        Some([]) | None => {
                            debug!("Error while probing {}", title);
                            continue;
                        }
                        Some(ss) => ss.to_vec(),
                    };
                    let padded_title = format!("{:width$}:", title, width = max_title_len);
                    for s in strings.iter() {
                        // Repeat the label on every line (e.g. one "GPU:" per GPU),
                        // matching neofetch rather than leaving orphaned values.
                        queue!(
                            w,
                            SetForegroundColor(primary_color),
                            Print(get_art(ra_idx)),
                            ResetColor,
                            Print("   "),
                            SetForegroundColor(primary_color),
                            Print(&padded_title),
                            ResetColor,
                            Print(" "),
                            Print(s),
                            Print("\n"),
                        )?;
                        ra_idx += 1;
                    }
                }
                art_idx = ra_idx;
                w.flush()?;
            } else {
                // Ensure cursor is at the bottom of the probe section
                execute!(w, cursor::RestorePosition)?;
                art_idx = probe_art_start + n_probes;
            }
        }

        // Print color blocks
        if self.config.col {
            // Spacer line between probes and color blocks
            queue!(
                w,
                SetForegroundColor(primary_color),
                Print(get_art(art_idx)),
                ResetColor,
                Print("\n"),
            )?;
            art_idx += 1;

            // Row 1: dark/standard colors (equivalent to ANSI background 40-47)
            queue!(
                w,
                SetForegroundColor(primary_color),
                Print(get_art(art_idx)),
                ResetColor,
                Print("   "),
            )?;
            for color in [
                Color::Black,
                Color::DarkRed,
                Color::DarkGreen,
                Color::DarkYellow,
                Color::DarkBlue,
                Color::DarkMagenta,
                Color::DarkCyan,
                Color::Grey,
            ] {
                queue!(w, SetBackgroundColor(color), Print("   "), ResetColor)?;
            }
            queue!(w, Print("\n"))?;
            art_idx += 1;

            // Row 2: bright colors (equivalent to ANSI background 100-107)
            queue!(
                w,
                SetForegroundColor(primary_color),
                Print(get_art(art_idx)),
                ResetColor,
                Print("   "),
            )?;
            for color in [
                Color::DarkGrey,
                Color::Red,
                Color::Green,
                Color::Yellow,
                Color::Blue,
                Color::Magenta,
                Color::Cyan,
                Color::White,
            ] {
                queue!(w, SetBackgroundColor(color), Print("   "), ResetColor)?;
            }
            queue!(w, Print("\n"))?;
            art_idx += 1;
        }

        // Print remaining ASCII art lines
        for line in ascii_art.iter().skip(art_idx) {
            queue!(
                w,
                SetForegroundColor(primary_color),
                Print(line),
                ResetColor,
                Print("\n"),
            )?;
        }

        w.flush()?;
        Ok(())
    }
}
