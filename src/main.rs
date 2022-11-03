use clap::{Args, Parser, Subcommand};
use dotenv;
use ethers_core::types::H160;

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
            Err(err) => {
                println!("There was an err fetching the gas price: {}", err);
            }
            _ => {}
        },
        Commands::Price => match etherscan::get_eth_price() {
            Err(err) => {
                println!("There was an err fetching the gas price: {}", err);
            }
            _ => {}
        },
        _ => {
            unimplemented!()
        }
    }
}
