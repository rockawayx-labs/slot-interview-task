use futures_util::StreamExt;
use solana_client::{nonblocking::pubsub_client::PubsubClient, rpc_response::SlotUpdate};
use tokio::sync::mpsc;
use url::Url;

use crate::service::ServiceError;

pub async fn start(ws: Url, slot_tx: mpsc::Sender<SlotUpdate>) -> Result<(), ServiceError> {
    let client = PubsubClient::new(ws.as_str()).await?;

    let (mut updates, _unsubscribe) = client.slot_updates_subscribe().await?;

    while let Some(update) = updates.next().await {
        handle_update(update, &slot_tx).await?;
    }

    Ok(())
}

async fn handle_update(
    update: SlotUpdate,
    slot_tx: &mpsc::Sender<SlotUpdate>,
) -> Result<(), ServiceError> {
    slot_tx
        .send(update)
        .await
        .map_err(|_| ServiceError::ChannelClosed)?;

    Ok(())
}
