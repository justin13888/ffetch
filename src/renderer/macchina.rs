use console::style;
use tracing::debug;

use crate::{
    colour::primary,
    config::MacchinaRendererConfig,
    probe::{general_readout, ProbeList, ProbeResultValue},
};

use super::RendererError;

pub struct MacchinaRenderer;

impl Default for MacchinaRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl MacchinaRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(
        &self,
        config: &MacchinaRendererConfig,
        probe_list: &ProbeList,
    ) -> Result<(), RendererError> {
        let title_width = std::cmp::max(
            probe_list
                .iter()
                .map(|(title, _)| title.len())
                .max()
                .unwrap_or(0)
                + 2,
            12,
        );
        println!();
        // TODO: Implement ASCII macchina logos

        for (title, probe) in probe_list {
            let results = match probe() {
                Ok(result) => match result {
                    ProbeResultValue::Single(value) => vec![value.to_string()],
                    ProbeResultValue::Multiple(values) => values
                        .into_iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>(),
                },
                Err(err) => {
                    debug!("Error while probing {}: {}", title, err);
                    continue;
                }
            };
            results.into_iter().for_each(|result| {
                println!(
                    "{:title_width$}{}  {}",
                    style(title.clone()).blue(),
                    style("-").yellow(),
                    result
                );
            });
        }

        Ok(())
    }
}
