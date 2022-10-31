use near_jsonrpc_client::JsonRpcClient;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod account_details;
mod circulating_supply;
mod lockup;
mod lockup_types;

const AGGREGATED: &str = "aggregated";

#[actix::main]
async fn main() {
    dotenv::dotenv().ok();

    let env_filter = EnvFilter::new("aggregated=info");

    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
        .init();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let pool = database::models::establish_connection(&database_url);

    let rpc_client = JsonRpcClient::connect("http://127.0.0.1:3030");
    info!(target: AGGREGATED, "starting");

    // if indexer.near_config().genesis.config.chain_id == "localnet" {
    circulating_supply::run_circulating_supply_computation(rpc_client, pool).await;
    // }
}
