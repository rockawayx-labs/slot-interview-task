use solana_client::rpc_response::SlotUpdate;
use tokio::sync::mpsc;

use crate::service::ServiceError;

pub async fn start(mut slot_rx: mpsc::Receiver<SlotUpdate>) -> Result<(), ServiceError> {
    while let Some(update) = slot_rx.recv().await {
        handle_update(update).await;
    }

    Ok(())
}

async fn handle_update(update: SlotUpdate) {
    println!("{:?}", update);
}
