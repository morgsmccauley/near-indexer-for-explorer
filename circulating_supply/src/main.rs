use near_indexer::Indexer;

use db::models;

mod account_details;
mod circulating_supply;
mod lockup;
mod lockup_types;

const AGGREGATED: &str = "aggregated";

fn main(indexer: &Indexer) {
    let view_client = indexer.client_actors().0;
    let pool = models::establish_connection();
    if indexer.near_config().genesis.config.chain_id == "localnet" {
        actix::spawn(circulating_supply::run_circulating_supply_computation(
            view_client,
            pool,
        ));
    }
}
