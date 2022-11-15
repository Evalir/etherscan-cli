use std::usize;

use anyhow::{anyhow, Result};
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
pub struct EthBalanceResult {
    #[serde(default, rename(deserialize = "result"))]
    pub balance: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum EtherscanResult {
    GasResult(GasResult),
    PriceResult(PriceResult),
    EthBalance(EthBalanceResult),
}

#[derive(Serialize, Deserialize, Debug)]
struct EtherscanApiResponse {
    message: String,
    status: String,
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

#[derive(Serialize, Deserialize, Debug)]
struct EtherscanEthBalanceResponse {
    #[serde(flatten)]
    status: EtherscanApiResponse,
    #[serde(flatten)]
    result: Option<EthBalanceResult>,
}

enum EtherscanApiRoute {
    Gas,
    Price,
}

#[derive(Default, Debug)]
pub struct Etherscan {
    api_key: String,
}

impl Etherscan {
    pub fn new(api_key: String) -> Etherscan {
        Etherscan { api_key }
    }

    fn build_etherscan_route(&self, route: EtherscanApiRoute) -> String {
        match route {
            EtherscanApiRoute::Gas => format!(
                "{}?module=gastracker&action=gasoracle&apikey={}",
                ETHERSCAN_BASE_URL, self.api_key
            ),
            EtherscanApiRoute::Price => format!(
                "{}?module=stats&action=ethprice&apikey={}",
                ETHERSCAN_BASE_URL, self.api_key
            ),
        }
    }

    pub fn get_gas(&self) -> Result<GasResult> {
        let url = self.build_etherscan_route(EtherscanApiRoute::Gas);

        let res = reqwest::blocking::get(url)?.json::<EtherscanGasResponse>()?;

        match res.result {
            Some(gas_info) => Ok(gas_info),
            None => Err(anyhow!("Could not fetch gas info")),
        }
    }

    pub fn get_eth_price(&self) -> Result<PriceResult> {
        let url = self.build_etherscan_route(EtherscanApiRoute::Price);

        let res = reqwest::blocking::get(url)?.json::<EtherscanPriceResponse>()?;

        match res.result {
            Some(price_info) => Ok(price_info),
            None => Err(anyhow!("Could not fetch price info")),
        }
    }
}
