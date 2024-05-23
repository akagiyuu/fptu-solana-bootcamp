use std::{env, str::FromStr};

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey, signature::Keypair, signer::Signer, system_instruction,
    transaction::Transaction,
};

const RECEIVER_PUBLIC_KEY: &str = "63EEC9FfGyksm7PkVC6z8uAmqozbQcTzbkWJNsgqjkFs";
const TRANSFER_AMOUNT: u64 = 5000;

//https://explorer.solana.com/tx/3nfm7M5f5p7zF3DCMh2UNwY35nP3mNV7TCok94sbpkgQZNNBDWzrVAfLpE6N4jCkQ66LHaPqhfLCjfG4WNoxqDr?cluster=devnet
fn main() -> Result<()> {
    let client = RpcClient::new("https://api.devnet.solana.com");

    let private_key = env::var("SOLANA_PRIVATE_KEY")?;
    let sender_key_pair = Keypair::from_base58_string(&private_key);

    let receiver_public_key = Pubkey::from_str(RECEIVER_PUBLIC_KEY)?;

    let transfer_instruction = system_instruction::transfer(
        &sender_key_pair.pubkey(),
        &receiver_public_key,
        TRANSFER_AMOUNT,
    );
    let transaction = Transaction::new_signed_with_payer(
        &[transfer_instruction],
        Some(&sender_key_pair.pubkey()),
        &[&sender_key_pair],
        client.get_latest_blockhash()?,
    );
    let signature = client.send_and_confirm_transaction(&transaction)?;

    let explorer_url = util::get_signature_explorer_url(&signature);
    println!("explorer url: {}", explorer_url);

    Ok(())
}
