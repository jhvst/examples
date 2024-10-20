//! Example of how to decode the input of a transaction.

use alloy::{primitives::hex, sol, sol_types::SolCall};
use eyre::Result;

// Codegen from excerpt of Uniswap V2 Router interface.
// See: https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02
sol!(
    #[allow(missing_docs)]
    #[derive(Debug)]

    struct Cluster {
        /// @dev The number of validators in the cluster
        uint32 validatorCount;
        /// @dev The index of network fees related to this cluster
        uint64 networkFeeIndex;
        /// @dev The last index calculated for the cluster
        uint64 index;
        /// @dev Flag indicating whether the cluster is active
        bool active;
        /// @dev The balance of the cluster
        uint256 balance;
    }

    function registerValidator(
        bytes calldata publicKey,
        uint64[] calldata operatorIds,
        bytes calldata sharesData,
        uint256 amount
    );
);

#[tokio::main]
async fn main() -> Result<()> {
    println!("Decoding https://etherscan.io/tx/0xd1b449d8b1552156957309bffb988924569de34fbf21b51e7af31070cc80fe9a");

    let input = hex::decode(input)?;

    // Decode the input using the generated `swapExactTokensForTokens` bindings.
    let decoded = registerValidatorCall::abi_decode(&input, false);

    match decoded {
        Ok(decoded) => {
            println!("{:?}", decoded);
        }
        Err(e) => {
            println!("Error decoding input: {e:?}");
        }
    }

    Ok(())
}
