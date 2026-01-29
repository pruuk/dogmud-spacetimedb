use anyhow::Result;
use dogmud_shepherds::SpacetimeClient;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let spacetime_url =
        std::env::var("SPACETIME_URL").unwrap_or_else(|_| "ws://localhost:3000".to_string());

    log::info!("DecayShepherd starting");

    let client = SpacetimeClient::connect(&spacetime_url, "dogmud").await?;

    let mut slow_tick = interval(Duration::from_secs(60));

    log::info!("DecayShepherd running. Press Ctrl+C to stop.");

    loop {
        slow_tick.tick().await;
        log::debug!("Slow tick (60s)");
        // TODO: Call cleanup_old_events, decay_corpses reducers
    }
}
