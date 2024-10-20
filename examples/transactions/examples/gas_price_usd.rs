//! Example of how to get the gas price in USD using the Chainlink ETH/USD feed.

use alloy::{
    network::TransactionBuilder,
    primitives::{address, utils::format_units, Address, Bytes, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    sol,
    sol_types::SolCall,
    eips::BlockId,
};
use std::error::Error;
use std::str::FromStr;

const ETH_USD_FEED: Address = address!("5f4eC3Df9cbd43714FE2740f5E3616155c5b8419");
const ETH_USD_FEED_DECIMALS: u8 = 8;

// Codegen from excerpt of Chainlink Aggregator interface.
// See: https://etherscan.io/address/0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419#code
sol!(
    #[allow(missing_docs)]
    function latestAnswer() external view returns (int256);
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_url = "http://192.168.100.10:8545".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let tx = TransactionRequest::default()
      .with_to(ETH_USD_FEED)
      .with_input(Bytes::from(latestAnswerCall {}.abi_encode()));

    let response = provider.call(&tx).block(BlockId::latest()).await?;
    let result = U256::from_str(&response.to_string())?;
    let usd = format_units(result, ETH_USD_FEED_DECIMALS)?.parse::<f64>()?;

    print!("{:.2}", usd);

    Ok(())
}

