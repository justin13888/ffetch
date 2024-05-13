use console::style;
use tracing::debug;

use crate::{
    colour::primary,
    config::NeofetchRendererConfig,
    probe::{general_readout, ProbeList, ProbeResultValue},
};

use super::RendererError;

pub struct NeofetchRenderer;

impl NeofetchRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(
        &self,
        config: &NeofetchRendererConfig,
        probe_list: &ProbeList,
    ) -> Result<(), RendererError> {
        let max_title_len = probe_list
            .iter()
            .map(|(title, _)| title.len())
            .max()
            .unwrap_or(0);

        // TODO: Render title and underline

        let mut title_len = 0;
        if config.title {
            use libmacchina::traits::GeneralReadout as _;
            let username = general_readout().username()?;
            let hostname = general_readout().hostname()?;
            title_len = username.len() + hostname.len() + 1;
            println!(
                "{}@{}",
                style(username).fg(primary()),
                style(hostname).fg(primary()),
            );
        }

        if config.underline {
            let underline = "-".repeat(title_len);
            println!("{}", underline);
        }

        for (title, probe) in probe_list {
            let title = format!("{:width$}:", title, width = max_title_len);
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
                    vec!["N/A".to_string()]
                }
            };
            results.into_iter().for_each(|result| {
                println!("{} {}", style(title.clone()).fg(primary()), result);
            });
        }

        // TODO: Render neofetch colour block below

        Ok(())
    }
}
