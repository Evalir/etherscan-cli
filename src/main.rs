use clap::{Args, Parser, Subcommand};
use ethers_core::types::H160;

mod gas;

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
    let args = Cli::parse();

    match args.command {
        Commands::Gas => {
            gas::get_gas();
            println!("get gas")
        }
        _ => {
            unimplemented!()
        }
    }

    println!("{:?}", args);
    println!("Hello, world!");
}
