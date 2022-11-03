use std::usize;

use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};

static ETHERSCAN_BASE_URL: &str = "https://api.etherscan.io/api";

#[derive(Serialize, Deserialize, Debug)]
struct GasResult {
    #[serde(default, rename(deserialize = "LastBlock"))]
    last_block: String,
    #[serde(default, rename(deserialize = "SafeGasPrice"))]
    safe_gas_price: String,
    #[serde(default, rename(deserialize = "ProposeGasPrice"))]
    propose_gas_price: String,
    #[serde(default, rename(deserialize = "FastGasPrice"))]
    fast_gas_price: String,
    #[serde(default, rename(deserialize = "suggestBaseFee"))]
    suggested_base_fee: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct EtherscanApiResponse {
    status: String,
    message: String,
    result: Option<GasResult>,
}

pub fn get_gas() -> Result<()> {
    let etherscan_api_key = dotenv::var("ETHERSCAN_API_KEY").unwrap();

    let url = format!(
        "{}?module=gastracker&action=gasoracle&apikey={}",
        ETHERSCAN_BASE_URL, etherscan_api_key
    );

    let res = reqwest::blocking::get(url)?.json::<EtherscanApiResponse>()?;

    match res.result {
        Some(gas_info) => {
            println!("Gas info:");
            println!("Safe Gas Price: {}", gas_info.safe_gas_price);
            println!("Propose Gas Price: {}", gas_info.propose_gas_price);
            println!("Fast Gas Price: {}", gas_info.fast_gas_price);
        }
        None => {
            println!("Error while fetching gas price.")
        }
    }

    Ok(())
}
