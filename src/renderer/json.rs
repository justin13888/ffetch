use std::io::Write;

use serde_json::{Value, json};

use crate::{
    config::JsonRendererConfig,
    probe::{ProbeList, ProbeResultValue, general_readout},
};

use super::{RendererError, execute_probes_streaming};

/// Renders the probe results as a JSON document (neofetch `--json`).
pub struct JsonRenderer {
    config: JsonRendererConfig,
    probe_list: ProbeList,
}

impl JsonRenderer {
    pub fn new(config: JsonRendererConfig) -> Self {
        let probe_list = config
            .probes
            .iter()
            .map(|p| p.get_funcs())
            .collect::<Vec<_>>();
        Self { config, probe_list }
    }

    pub fn draw(&self) -> Result<(), RendererError> {
        use libmacchina::traits::GeneralReadout as _;

        let probes = &self.config.probes;
        let mut entries: Vec<Value> = vec![Value::Null; probes.len()];
        execute_probes_streaming(&self.probe_list, |index, label, result| {
            let id = probes[index].id();
            entries[index] = match result {
                Some(ProbeResultValue::Single(v)) => json!({
                    "id": id,
                    "label": label,
                    "value": probes[index].format_value(&v),
                }),
                Some(ProbeResultValue::Multiple(vs)) => {
                    let values: Vec<String> =
                        vs.iter().map(|v| probes[index].format_value(v)).collect();
                    json!({ "id": id, "label": label, "values": values })
                }
                None => json!({ "id": id, "label": label, "error": "unavailable" }),
            };
        });

        let distro = general_readout().distribution().ok();
        let host = match (general_readout().username(), general_readout().hostname()) {
            (Ok(u), Ok(h)) => Some(format!("{u}@{h}")),
            _ => None,
        };

        let out = json!({
            "distro": distro,
            "host": host,
            "probes": entries,
        });

        let s = serde_json::to_string_pretty(&out).map_err(std::io::Error::other)?;
        let mut w = std::io::stdout().lock();
        writeln!(w, "{s}")?;
        Ok(())
    }
}
