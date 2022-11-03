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
struct PriceResult {
    #[serde(default, rename(deserialize = "ethusd"))]
    eth_usd_price: String,
    #[serde(default, rename(deserialize = "ethbtc"))]
    eth_btc_price: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum EtherscanResult {
    GasResult(GasResult),
    PriceResult(PriceResult),
}

#[derive(Serialize, Deserialize, Debug)]
struct EtherscanApiResponse {
    status: String,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct EtherscanGasResponse {
    #[serde(flatten)]
    status: EtherscanApiResponse,
    result: Option<GasResult>,
}

#[derive(Serialize, Deserialize, Debug)]
struct EtherscanPriceResponse {
    #[serde(flatten)]
    status: EtherscanApiResponse,
    result: Option<PriceResult>,
}

enum EtherscanApiRoute {
    Gas,
    Price,
}

fn build_etherscan_route(route: EtherscanApiRoute) -> String {
    let etherscan_api_key = dotenv::var("ETHERSCAN_API_KEY").unwrap();

    match route {
        EtherscanApiRoute::Gas => format!(
            "{}?module=gastracker&action=gasoracle&apikey={}",
            ETHERSCAN_BASE_URL, etherscan_api_key
        ),
        EtherscanApiRoute::Price => format!(
            "{}?module=stats&action=ethprice&apikey={}",
            ETHERSCAN_BASE_URL, etherscan_api_key
        ),
    }
}

pub fn get_gas() -> Result<()> {
    let url = build_etherscan_route(EtherscanApiRoute::Gas);

    let res = reqwest::blocking::get(url)?.json::<EtherscanGasResponse>()?;

    match res.result {
        Some(gas_info) => {
            println!("Gas info:");
            println!("Safe Gas Price: {}", gas_info.safe_gas_price);
            println!("Propose Gas Price: {}", gas_info.propose_gas_price);
            println!("Fast Gas Price: {}", gas_info.fast_gas_price);
        }
        _ => {
            println!("Error while fetching info.");
        }
    }

    Ok(())
}

pub fn get_eth_price() -> Result<()> {
    let url = build_etherscan_route(EtherscanApiRoute::Price);

    let res = reqwest::blocking::get(url)?.json::<EtherscanPriceResponse>()?;

    match res.result {
        Some(price_info) => {
            println!("price info:");
            println!("ETH/USD Price: {}", price_info.eth_usd_price);
            println!("ETH/BTC Price: {}", price_info.eth_btc_price);
        }
        _ => {
            println!("Error while fetching info.");
        }
    }

    Ok(())
}
