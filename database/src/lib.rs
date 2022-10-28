#[macro_use]
extern crate diesel;

pub mod adapters;
pub mod cache;
pub mod models;
pub mod schema;

#[macro_use]
mod retriable;

pub use actix_diesel::Database;
pub use diesel::PgConnection;

const MAX_DELAY_TIME: std::time::Duration = std::time::Duration::from_secs(120);
const INTERVAL: std::time::Duration = std::time::Duration::from_millis(100);
// TODO should be configurable
const INDEXER_FOR_EXPLORER: &str = "indexer_for_explorer";
const AGGREGATED: &str = "aggregated";
