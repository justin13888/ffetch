use libmacchina::traits::ReadoutError;
use thiserror::Error;
use tracing::info_span;

use crate::probe::{ProbeList, ProbeResultValue};

pub mod macchina;
pub mod neofetch;

pub fn execute_probes_streaming<F>(probe_list: &ProbeList, mut on_result: F)
where
    F: FnMut(usize, &str, Option<ProbeResultValue>),
{
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::scope(|s| {
        for (index, (label, probe_fn)) in probe_list.iter().enumerate() {
            let tx = tx.clone();
            let label = label.clone();
            s.spawn(move || {
                let _span = info_span!("probe", name = %label).entered();
                let result = probe_fn().ok();
                let _ = tx.send((index, label, result));
            });
        }
        drop(tx);
        for (index, label, result) in rx {
            on_result(index, &label, result);
        }
    });
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
