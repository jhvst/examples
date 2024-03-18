//! Example of querying contract storage from the Ethereum network.

use alloy_network::Ethereum;
use alloy_primitives::{address, U256};
use alloy_provider::{HttpProvider, Provider};
use alloy_rpc_client::RpcClient;
use alloy_transport_http::Http;
use eyre::Result;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = init();

    // Get slot0 from USDC-ETH Uniswap V3 pool
    let pool_address = address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640");

    let storage_slot = U256::from(0);

    let storage = provider.get_storage_at(pool_address, storage_slot, None).await?;

    println!("Slot 0: {:?}", storage);

    Ok(())
}

fn init() -> HttpProvider<Ethereum> {
    let http = Http::<Client>::new("https://eth.merkle.io".parse().unwrap());
    HttpProvider::new(RpcClient::new(http, true))
}
