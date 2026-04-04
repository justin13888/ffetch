use libmacchina::traits::ReadoutError;
use thiserror::Error;
use tracing::info_span;

use crate::probe::{ProbeList, ProbeResultValue};

pub mod macchina;
pub mod neofetch;

pub fn execute_probes_parallel(probe_list: &ProbeList) -> Vec<(String, Option<ProbeResultValue>)> {
    std::thread::scope(|s| {
        let handles: Vec<_> = probe_list
            .iter()
            .map(|(label, probe_fn)| {
                let label = label.clone();
                s.spawn(move || {
                    let _span = info_span!("probe", name = %label).entered();
                    (label, probe_fn().ok())
                })
            })
            .collect();

        handles.into_iter().map(|h| h.join().unwrap()).collect()
    })
}

#[derive(Error, Debug)]
pub enum RendererError {
    #[error("Failed to read config")]
    ReadoutError(ReadoutError),
    #[error("Failed to print")]
    PrintError(#[from] std::io::Error),
}

impl From<ReadoutError> for RendererError {
    fn from(err: ReadoutError) -> Self {
        RendererError::ReadoutError(err)
    }
}
