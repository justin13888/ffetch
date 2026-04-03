use criterion::{Criterion, criterion_group, criterion_main};
use ffetch_lib::probe::{ProbeResultFunction, ProbeType};

fn bench_probe(c: &mut Criterion, name: &str, probe_type: ProbeType) {
    let probe: ProbeResultFunction = probe_type.into();
    c.bench_function(name, |b| b.iter(&probe));
}

// --- Fast probes (env vars, simple libmacchina reads) ---
fn fast_probes(c: &mut Criterion) {
    bench_probe(c, "probe/host", ProbeType::Host);
    bench_probe(c, "probe/os", ProbeType::OS);
    bench_probe(c, "probe/kernel", ProbeType::Kernel);
    bench_probe(c, "probe/distro", ProbeType::Distro);
    bench_probe(c, "probe/uptime", ProbeType::Uptime);
    bench_probe(c, "probe/shell", ProbeType::Shell);
    bench_probe(c, "probe/cpu", ProbeType::CPU);
    bench_probe(c, "probe/memory", ProbeType::Memory);
    bench_probe(c, "probe/locale", ProbeType::Locale);
    bench_probe(c, "probe/editor", ProbeType::Editor);
    bench_probe(c, "probe/terminal", ProbeType::Terminal);
    bench_probe(c, "probe/battery", ProbeType::Battery);
    bench_probe(c, "probe/model", ProbeType::Model);
    bench_probe(c, "probe/resolution", ProbeType::Resolution);
    bench_probe(c, "probe/de", ProbeType::DE);
    bench_probe(c, "probe/wm", ProbeType::WM);
}

// --- I/O-heavy probes (disk scans, user enumeration, network, package counting) ---
fn io_heavy_probes(c: &mut Criterion) {
    bench_probe(c, "probe/packages", ProbeType::Packages);
    bench_probe(c, "probe/disk", ProbeType::Disk);
    bench_probe(c, "probe/cpu_usage", ProbeType::CPUUsage);
    bench_probe(c, "probe/local_ip", ProbeType::LocalIP);
    bench_probe(c, "probe/gpu", ProbeType::GPU);
    bench_probe(c, "probe/users", ProbeType::Users);
}

// --- Subprocess probes (spawn external commands) ---
fn subprocess_probes(c: &mut Criterion) {
    bench_probe(c, "probe/java", ProbeType::Java);
    bench_probe(c, "probe/python", ProbeType::Python);
    bench_probe(c, "probe/node", ProbeType::Node);
    bench_probe(c, "probe/rust", ProbeType::Rust);
    bench_probe(c, "probe/theme", ProbeType::Theme);
    bench_probe(c, "probe/wm_theme", ProbeType::WMTheme);
    bench_probe(c, "probe/icons", ProbeType::Icons);
    bench_probe(c, "probe/terminal_font", ProbeType::TerminalFont);
}

// --- Readout initialization cost (one-time per process; these measure cold construction) ---
fn readout_init(c: &mut Criterion) {
    use libmacchina::traits::{
        BatteryReadout as _, GeneralReadout as _, KernelReadout as _, MemoryReadout as _,
        NetworkReadout as _, PackageReadout as _, ProductReadout as _,
    };
    c.bench_function("readout_init/general", |b| {
        b.iter(libmacchina::GeneralReadout::new)
    });
    c.bench_function("readout_init/kernel", |b| {
        b.iter(libmacchina::KernelReadout::new)
    });
    c.bench_function("readout_init/memory", |b| {
        b.iter(libmacchina::MemoryReadout::new)
    });
    c.bench_function("readout_init/package", |b| {
        b.iter(libmacchina::PackageReadout::new)
    });
    c.bench_function("readout_init/network", |b| {
        b.iter(libmacchina::NetworkReadout::new)
    });
    c.bench_function("readout_init/battery", |b| {
        b.iter(libmacchina::BatteryReadout::new)
    });
    c.bench_function("readout_init/product", |b| {
        b.iter(libmacchina::ProductReadout::new)
    });
}

criterion_group!(bench_fast, fast_probes);
criterion_group!(bench_io, io_heavy_probes);
criterion_group!(bench_subprocess, subprocess_probes);
criterion_group!(bench_readout_init, readout_init);
criterion_main!(bench_fast, bench_io, bench_subprocess, bench_readout_init);
