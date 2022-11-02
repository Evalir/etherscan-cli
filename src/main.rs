use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "etherscan")]
#[command(about = "A CLI to interact with etherscan", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Gas,
    // compose this as this needs further subcommands for different etherscan methods
    // see: https://docs.rs/clap/latest/clap/_derive/_cookbook/git_derive/index.html#example-git-like-cli-derive-api
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
    println!("Hello, world!");
}
