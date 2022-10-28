mod account_details;
mod circulating_supply;
mod lockup;
mod lockup_types;

const AGGREGATED: &str = "aggregated";

fn main() {
    dotenv::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let pool = database::models::establish_connection(&database_url);

    // if indexer.near_config().genesis.config.chain_id == "localnet" {
    actix::spawn(circulating_supply::run_circulating_supply_computation(
        view_client,
    ));
    // }
}
