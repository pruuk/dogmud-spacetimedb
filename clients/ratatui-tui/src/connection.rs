use anyhow::Result;
use spacetimedb_sdk::{
    disconnect, identity::load_credentials, on_connect, on_disconnect, subscribe, Table,
};
use std::sync::{Arc, Mutex};

pub struct Connection {
    pub connected: Arc<Mutex<bool>>,
}

impl Connection {
    pub async fn new(host: &str, db_name: &str) -> Result<Self> {
        let connected = Arc::new(Mutex::new(false));
        let connected_clone = connected.clone();

        // Set up connection callbacks
        on_connect(move || {
            log::info!("Connected to SpacetimeDB");
            *connected_clone.lock().unwrap() = true;
        });

        on_disconnect(move || {
            log::info!("Disconnected from SpacetimeDB");
        });

        // Load or create credentials
        let creds = load_credentials(db_name)?;

        // Connect to SpacetimeDB
        spacetimedb_sdk::connect(host, db_name, creds.clone()).await?;

        // Subscribe to all tables we care about
        subscribe(&["SELECT * FROM entity"]).await?;
        subscribe(&["SELECT * FROM room"]).await?;
        subscribe(&["SELECT * FROM player_session"]).await?;

        Ok(Self { connected })
    }

    pub fn is_connected(&self) -> bool {
        *self.connected.lock().unwrap()
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        let _ = disconnect();
    }
}
