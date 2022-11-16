use std::usize;

use anyhow::{anyhow, Result};
use ethers::types::H160;
use serde::{Deserialize, Serialize};

use crate::url::URLBuilder;

static ETHERSCAN_BASE_URL: &str = "api.etherscan.io/api";

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
struct EtherscanApiResponse<T> {
    message: String,
    status: String,
    result: Option<T>,
}

#[derive(Default, Debug)]
pub struct Etherscan {
    api_key: String,
}

impl Etherscan {
    pub fn new(api_key: String) -> Etherscan {
        Etherscan { api_key }
    }

    pub fn get_gas(&self) -> Result<GasResult> {
        let url = URLBuilder::new()
            .set_protocol("https")
            .set_host(ETHERSCAN_BASE_URL)
            .add_param("module", "gastracker")
            .add_param("action", "gasoracle")
            .add_param("apikey", &self.api_key)
            .build();

        let res = reqwest::blocking::get(url)?.json::<EtherscanApiResponse<GasResult>>()?;

        match res.result {
            Some(gas_info) => Ok(gas_info),
            None => Err(anyhow!("Could not fetch gas info")),
        }
    }

    pub fn get_eth_price(&self) -> Result<PriceResult> {
        let url = URLBuilder::new()
            .set_protocol("http")
            .set_host(ETHERSCAN_BASE_URL)
            .add_param("module", "stats")
            .add_param("action", "ethprice")
            .add_param("apikey", &self.api_key)
            .build();

        let res = reqwest::blocking::get(url)?.json::<EtherscanApiResponse<PriceResult>>()?;

        match res.result {
            Some(price_info) => Ok(price_info),
            None => Err(anyhow!("Could not fetch price info")),
        }
    }

    pub fn get_balance(&self, address: H160, token_address: Option<H160>) -> Result<String> {
        let url = match token_address {
            None => URLBuilder::new()
                .set_protocol("https")
                .set_host(ETHERSCAN_BASE_URL)
                .add_param("module", "account")
                .add_param("action", "balance")
                .add_param(
                    "address",
                    &format!("0x{}", hex::encode(address.to_fixed_bytes())),
                )
                .add_param("tag", "latest")
                .add_param("apikey", &self.api_key)
                .build(),
            Some(token_address) => URLBuilder::new()
                .set_protocol("https")
                .set_host(ETHERSCAN_BASE_URL)
                .add_param("module", "account")
                .add_param("action", "tokenbalance")
                .add_param(
                    "address",
                    &format!("0x{}", hex::encode(address.to_fixed_bytes())),
                )
                .add_param(
                    "contractaddress",
                    &format!("0x{}", hex::encode(token_address.to_fixed_bytes())),
                )
                .add_param("tag", "latest")
                .add_param("apikey", &self.api_key)
                .build(),
        };

        let res = reqwest::blocking::get(url)?.json::<EtherscanApiResponse<String>>()?;

        match res.result {
            Some(balance) => Ok(balance),
            None => Err(anyhow!("Could not fetch balance for the account specified")),
        }
    }
}
