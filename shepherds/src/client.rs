use anyhow::Result;

pub struct SpacetimeClient {
    // TODO: Implement actual SpacetimeDB SDK client wrapper
    url: String,
    module: String,
}

impl SpacetimeClient {
    pub async fn connect(url: &str, module: &str) -> Result<Self> {
        log::info!("Connecting to SpacetimeDB: {} / {}", url, module);

        // TODO: Use spacetimedb-sdk to create actual connection
        // For now, just a stub

        Ok(Self {
            url: url.to_string(),
            module: module.to_string(),
        })
    }

    pub async fn call_reducer(&self, name: &str, args: serde_json::Value) -> Result<()> {
        log::info!("Calling reducer: {} with args: {:?}", name, args);

        // TODO: Implement actual reducer call using spacetimedb-sdk

        Ok(())
    }
}
