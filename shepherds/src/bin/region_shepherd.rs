use anyhow::Result;
use dogmud_shepherds::SpacetimeClient;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let region_id = std::env::var("REGION_ID")
        .unwrap_or_else(|_| "1".to_string())
        .parse::<u64>()
        .expect("REGION_ID must be a number");

    let spacetime_url =
        std::env::var("SPACETIME_URL").unwrap_or_else(|_| "ws://localhost:3000".to_string());

    log::info!("RegionShepherd starting for region {}", region_id);

    let client = SpacetimeClient::connect(&spacetime_url, "dogmud").await?;

    let mut fast_tick = interval(Duration::from_secs(1));
    let mut medium_tick = interval(Duration::from_secs(5));

    log::info!("RegionShepherd running. Press Ctrl+C to stop.");

    loop {
        tokio::select! {
            _ = fast_tick.tick() => {
                log::debug!("Fast tick (1s) for region {}", region_id);
                // TODO: Call tick_conditions reducer
                // client.call_reducer(\"tick_conditions\", json!({ \"region_id\": region_id })).await?;
            }
            _ = medium_tick.tick() => {
                log::debug!("Medium tick (5s) for region {}", region_id);
                // TODO: Call tick_npcs, tick_weather reducers
            }
        }
    }
}
