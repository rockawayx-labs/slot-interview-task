use std::error::Error;

use rockawayx_labs_interview::{config::Config, service};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_env()?;

    let mut service_handle = service::start(config).await?;

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("shutting down");
        },
        result = service_handle.run() => {
            eprintln!("service error: {result:?}");
            result?;
        }
    }

    Ok(())
}
