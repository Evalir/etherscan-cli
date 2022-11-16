use clap::{Args, Parser, Subcommand};
use dotenv::{dotenv, var};
use ethers::{core::types::H160, types::U256};

mod etherscan;
mod url;

#[derive(Debug, Parser)]
#[command(name = "etherscan")]
#[command(about = "A CLI to interact with etherscan", long_about = None)]
struct Cli {
    #[arg(short, long, global = true)]
    api_key: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Account(Account),
    Gas,
    Price,
}

#[derive(Debug, Args)]
struct Account {
    #[command(subcommand)]
    command: Option<AccountCommands>,
}

#[derive(Debug, Subcommand)]
enum AccountCommands {
    Balance {
        address: H160,
        token: Option<String>,
        #[arg(short = 'd', long = "decimals")]
        decimals: Option<usize>,
    },
    ErcBalance {
        address: H160,
        contract_address: H160,
    },
}

fn main() {
    let args = Cli::parse();

    let api_key = match args.api_key {
        Some(api_key) => api_key,
        None => {
            dotenv()
                .expect("API Key not provided. Either use the --api-key option or provide an .env");
            var("ETHERSCAN_API_KEY")
                .expect("API Key not provided. Either use the --api-key option or provide an .env")
        }
    };

    let etherscan = etherscan::Etherscan::new(api_key);

    match args.command {
        Commands::Gas => match etherscan.get_gas() {
            Ok(gas_info) => {
                println!("Gas info:");
                println!("Safe Gas Price: {}", gas_info.safe_gas_price);
                println!("Propose Gas Price: {}", gas_info.propose_gas_price);
                println!("Fast Gas Price: {}", gas_info.fast_gas_price);
            }
            Err(err) => {
                println!("{}", err);
            }
        },
        Commands::Price => match etherscan.get_eth_price() {
            Ok(price_info) => {
                println!("price info:");
                println!("ETH/USD Price: {}", price_info.eth_usd_price);
                println!("ETH/BTC Price: {}", price_info.eth_btc_price);
            }
            Err(err) => {
                println!("{}", err);
            }
        },
        Commands::Account(account) => {
            match account.command {
                Some(AccountCommands::Balance {
                    address,
                    token,
                    decimals,
                }) => {
                    let (balance, token_name) = match token {
                        None => (
                            etherscan.get_eth_balance(address).unwrap(),
                            "ETH".to_string(),
                        ),
                        Some(ticker) => (
                            etherscan
                                .get_token_balance(
                                    address,
                                    ethers::addressbook::contract(&ticker.to_lowercase())
                                        .expect("Invalid token name")
                                        .address(ethers::types::Chain::Mainnet)
                                        .unwrap(),
                                )
                                .unwrap(),
                            ticker,
                        ),
                    };

                    match decimals {
                        None => {
                            let parsed_balance =
                                ethers::core::utils::parse_units(balance, "wei").unwrap();
                            println!(
                                "{} Balance: {}",
                                token_name,
                                ethers::core::utils::format_units(parsed_balance, "ether").unwrap(),
                            )
                        }
                        Some(decimals) => {
                            println!(
                            "{} balance: {}",
                            token_name,
                            ethers::core::utils::format_units(
                                U256::from_dec_str(&balance)
                                    .expect("Malformed balance input from Etherscan"),
                                decimals
                            )
                            .expect("Could not format units properly. Did you input valid decimals?")
                        );
                        }
                    }
                }
                _ => todo!(),
            }
        }
    }
}
