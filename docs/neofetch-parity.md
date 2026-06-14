# neofetch feature parity

Parity assessed against neofetch [`ccd5d9f`][nf] — **dated 2026-06-14**.

[nf]: https://github.com/dylanaraps/neofetch/blob/ccd5d9f52609bbdcd5d8fa78c4fdb0f12954125f/neofetch

purr aims for feature-parity with neofetch's **commonly-used** surface. A
handful of dated or niche features are intentionally **deferred** and recorded
below with rationale, so the parity claim stays honest and auditable. Legend:
✅ implemented · 🟡 partial · ⏸ deferred.

## Info fields

| neofetch | purr | notes |
|---|---|---|
| title (user@host) | ✅ | `title_fqdn` supported |
| OS / distro | ✅ | `distro_shorthand`, `os_arch` |
| host / model | ✅ | |
| kernel | ✅ | `kernel_shorthand` |
| uptime | ✅ | `uptime_shorthand` on/tiny/off |
| packages | 🟡 | `package_managers` on/tiny/off; manager coverage = libmacchina's set (pacman, dpkg, rpm, dnf, apk, xbps, portage, nix, flatpak, snap, cargo, brew, pkg, …), not neofetch's full 60+ |
| shell | ✅ | `shell_path`, `shell_version` |
| resolution | 🟡 | shown; `refresh_rate` option exists but the Hz data source is not yet wired |
| DE | ✅ | `de_version` (GNOME/Plasma/Xfce/MATE/Cinnamon/Budgie/LXQt) |
| WM / WM theme | ✅ | |
| theme / icons / font | ✅ | GTK via gsettings + GTK3 ini; `gtk2`/`gtk3` split not separately exposed |
| terminal / terminal font | ✅ | |
| CPU | ✅ | `cpu_brand`, `cpu_cores` (logical/physical/off), `cpu_speed`, `speed_type`, `speed_shorthand` |
| CPU temperature | ⏸ | deferred — off by default in neofetch, platform-fiddly (hwmon/thermal) |
| GPU | ✅ | `gpu_brand`, `gpu_type` (all/dedicated/integrated, best-effort heuristic) |
| GPU driver | ✅ | Linux PCI sysfs |
| memory | ✅ | `memory_unit` (kib/mib/gib), `memory_percent` |
| disk | ✅ | `disk_show`, `disk_subtitle` (mount/name/dir/none), `disk_percent` |
| battery / power adapter | ✅ | |
| song (now playing) | 🟡 | MPRIS via zbus (covers modern MPRIS players); `song_format` honored, `song_shorthand` simplified to one line (drops album). neofetch's 45-player matrix not replicated |
| local IP | ✅ | |
| public IP | ⏸ | deferred — outbound network lookup; privacy/offline concerns |
| users / locale | ✅ | |
| color blocks (cols) | ✅ | `block_range`, `block_width`, `block_height`, `col_offset` |
| birthday (install date) | ⏸ | deferred — niche |
| GPU/CPU/disk/battery usage **bars** | ⏸ | deferred — `bar`/`infobar`/`barinfo` displays not implemented |

purr also ships extra fields neofetch lacks: **editor**, **CPU usage**, and
language versions (**Java / Python / Node / Rust**). The `Cursor`, `Network`,
`Bluetooth`, and `BIOS` probe slots exist but are **unimplemented** (not
neofetch features).

## Styling & config

| neofetch | purr | notes |
|---|---|---|
| separator, bold, underline_char | ✅ | |
| colors (6 text slots) | ✅ | `[title, @, underline, subtitle, colon, info]` |
| ascii_distro / ascii_colors / ascii_bold | ✅ | runtime `${c1}`..`${c6}` expansion |
| custom ASCII art format | ✅ | neofetch `${cN}` + `# set_colors` headers (drop-in) |
| `--stdout` (no colour) | ✅ | honours `NO_COLOR` |
| `-L`/`--logo`, `--off` | ✅ | |
| `--json` | ✅ | structured `{distro, host, probes[]}` |
| `print_info()` arbitrary-bash customization | 🟡 | purr is config-driven (TOML + CLI), not a bash script. Reordering/relabel/enable-disable/per-probe options are covered; arbitrary inline commands (`prin`, `$(...)`) and a free-form custom line are **not** (a `custom` probe could be added later) |

## Image rendering

| backend | purr | notes |
|---|---|---|
| ascii (default) | ✅ | |
| kitty (graphics protocol) | 🟡 | implemented (PNG-only, dimensions from IHDR, base64 APC). Falls back to ASCII when not a Kitty TTY. Side-by-side info block uses basic styling; visual layout verified only on Kitty terminals |
| w3m, sixel, iterm2, chafa, caca, catimg, jp2a, pixterm, termpix, tycat, ueberzug, viu, pot | ⏸ | deferred — niche backends |
| ANSI half-block fallback | ⏸ | deferred |
| wallpaper as source, `--loop` redraw | ⏸ | deferred |

## CLI / misc

| neofetch | purr | notes |
|---|---|---|
| `--config`, `--config none`/`--no_config` | ✅ | |
| per-field flags | 🟡 | representative set wired (`--memory_unit`, `--uptime_shorthand`, `--cpu_cores`, plus all logo/text flags). Every option is reachable via the config file; not every neofetch flag has a CLI alias |
| `--gen-man` | ⏸ | deferred |
| `--clean` (cache/thumbnails) | ⏸ | n/a — purr keeps no image cache |
| image crop/offset/gap, `--xoffset`/`--bg_color` | ⏸ | deferred (w3m-era options) |

## OS / distro coverage

See [`os-support.md`](os-support.md). purr ships 50 logos for mainstream +
maintained distros and prunes neofetch's discontinued/obscure long tail.
