use solana_client::{nonblocking::pubsub_client::PubsubClientError, rpc_response::SlotUpdate};
use thiserror::Error;
use tokio::{sync::mpsc, task::JoinSet};

use crate::config::Config;

mod collector;
mod reader;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("channel closed")]
    ChannelClosed,
    #[error("pubsub client: {0}")]
    Pubsub(#[from] PubsubClientError),
    #[error("join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}

pub struct ServiceHandle {
    slot_update_tx: mpsc::Sender<SlotUpdate>,
    tasks: JoinSet<Result<(), ServiceError>>,
}
impl ServiceHandle {
    pub async fn run(&mut self) -> Result<(), ServiceError> {
        while let Some(result) = self.tasks.join_next().await {
            result??;
        }
        Ok(())
    }
}

pub async fn start(config: Config) -> Result<ServiceHandle, ServiceError> {
    let (slot_update_tx, slot_update_rx) = mpsc::channel(1024);
    let mut tasks = JoinSet::new();

    tasks.spawn(collector::start(config.ws, slot_update_tx.clone()));
    tasks.spawn(reader::start(slot_update_rx));

    Ok(ServiceHandle {
        slot_update_tx,
        tasks,
    })
}
