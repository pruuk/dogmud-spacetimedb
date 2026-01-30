use spacetimedb::{reducer, ReducerContext};

// Module declarations
mod reducers;
mod tables;
mod utils;

// Re-export everything
pub use reducers::*;
pub use tables::*;

// Lifecycle hooks
#[reducer(init)]
pub fn init(_ctx: &ReducerContext) {
    log::info!("DOGMUD server module initialized");
}

#[reducer(client_connected)]
pub fn client_connected(_ctx: &ReducerContext) {
    log::info!("Client connected");
}

#[reducer(client_disconnected)]
pub fn client_disconnected(_ctx: &ReducerContext) {
    log::info!("Client disconnected");
}
