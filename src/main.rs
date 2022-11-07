use clap::{Args, Parser, Subcommand};
use ethers::core::types::H160;

mod etherscan;

#[derive(Debug, Parser)]
#[command(name = "etherscan")]
#[command(about = "A CLI to interact with etherscan", long_about = None)]
struct Cli {
    #[arg(short, long)]
    api_key: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Gas,
    Price,
    Account(Account),
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
    },
    ErcBalance {
        address: H160,
        contract_address: H160,
    },
}

fn main() {
    dotenv::dotenv().unwrap();
    let args = Cli::parse();

    match args.command {
        Commands::Gas => match etherscan::get_gas() {
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
        Commands::Price => match etherscan::get_eth_price() {
            Ok(price_info) => {
                println!("price info:");
                println!("ETH/USD Price: {}", price_info.eth_usd_price);
                println!("ETH/BTC Price: {}", price_info.eth_btc_price);
            }
            Err(err) => {
                println!("{}", err);
            }
        },
        _ => {
            unimplemented!()
        }
    }
}
