use std::usize;

use anyhow::{anyhow, Result};
use reqwest;
use serde::{Deserialize, Serialize};

static ETHERSCAN_BASE_URL: &str = "https://api.etherscan.io/api";

#[derive(Serialize, Deserialize, Debug)]
pub struct GasResult {
    #[serde(default, rename(deserialize = "LastBlock"))]
    pub last_block: String,
    #[serde(default, rename(deserialize = "SafeGasPrice"))]
    pub safe_gas_price: String,
    #[serde(default, rename(deserialize = "ProposeGasPrice"))]
    pub propose_gas_price: String,
    #[serde(default, rename(deserialize = "FastGasPrice"))]
    pub fast_gas_price: String,
    #[serde(default, rename(deserialize = "suggestBaseFee"))]
    pub suggested_base_fee: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceResult {
    #[serde(default, rename(deserialize = "ethusd"))]
    pub eth_usd_price: String,
    #[serde(default, rename(deserialize = "ethbtc"))]
    pub eth_btc_price: String,
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

pub fn get_gas() -> Result<GasResult> {
    let url = build_etherscan_route(EtherscanApiRoute::Gas);

    let res = reqwest::blocking::get(url)?.json::<EtherscanGasResponse>()?;

    match res.result {
        Some(gas_info) => Ok(gas_info),
        None => Err(anyhow!("Could not fetch gas info")),
    }
}

pub fn get_eth_price() -> Result<PriceResult> {
    let url = build_etherscan_route(EtherscanApiRoute::Price);

    let res = reqwest::blocking::get(url)?.json::<EtherscanPriceResponse>()?;

    match res.result {
        Some(price_info) => Ok(price_info),
        None => Err(anyhow!("Could not fetch price info")),
    }
}
