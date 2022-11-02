use clap::{Args, Parser, Subcommand};

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
        address: String,
    },
    ErcBalance {
        address: String,
        contract_address: String,
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
