use std::str::FromStr;
use aptos_sdk::crypto::ed25519::Ed25519PrivateKey;
use aptos_sdk::crypto::{PrivateKey, ValidCryptoMaterialStringExt};
use aptos_sdk::transaction_builder::TransactionBuilder;
use aptos_sdk::types::account_address::AccountAddress;
use aptos_sdk::types::chain_id::ChainId;
use aptos_sdk::types::transaction::{Script, TransactionPayload};
use clap::{CommandFactory, Parser, Subcommand};
use reqwest::Url;
use serde::{Serialize, Deserialize};

#[derive(Parser, Debug)]
#[command(about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// run move script
    Run {
        #[arg(short, long)]
        private_key: String,

        #[arg(short, long)]
        address: String,

        #[arg(short, long)]
        byte_code: String,
    },
}

#[derive(Serialize, Deserialize)]
struct Account {
    pub sequence_number: String,
    pub authentication_key: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run {
            private_key,
            address,
            byte_code,
        }) => {
            let private_key = Ed25519PrivateKey::from_encoded_string(&private_key).unwrap();
            let public_key = private_key.public_key();
            let account = AccountAddress::from_str(&address).unwrap();
            let account_info: Account = reqwest::get(format!("https://fullnode.devnet.aptoslabs.com/v1/accounts/{}", account.to_string())).await.unwrap().json().await.unwrap();

            let decoded_byte_code = hex::decode(byte_code).expect("wrong bytecode");

            let tx = TransactionBuilder::new(
                TransactionPayload::Script(Script::new(
                    decoded_byte_code,
                    vec![],
                    vec![],
                )),
                32425224034,
                ChainId::new(43),
            )
                .max_gas_amount(10000)
                .gas_unit_price(100)
                .sequence_number(account_info.sequence_number.parse().unwrap())
                .sender(account)
                .build();

            let signed_tx = tx.sign(&private_key, public_key).unwrap();

            let client = aptos_sdk::rest_client::Client::new(Url::from_str("https://fullnode.devnet.aptoslabs.com/v1").unwrap());
            let res = client.submit(&signed_tx).await.unwrap();
            println!("{:?}", res);
        },
        None => {
            Cli::command().print_help().unwrap();
            std::process::exit(0)
        }
    }
}
