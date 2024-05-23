use std::{env, str::FromStr};

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey, signature::Keypair, signer::Signer, system_instruction, system_program,
    transaction::Transaction,
};

const TRANSFER_AMOUNT: u64 = 5000;
const STATIC_RECEIVER: &str = "63EEC9FfGyksm7PkVC6z8uAmqozbQcTzbkWJNsgqjkFs";
const STATIC_RECEIVER_TRANSFER_AMOUNT: u64 = 7000;

// https://explorer.solana.com/tx/3wiK4y5E6jdGre2M7hCEe5NELqr8pJhVDGUbeiyeP5hfxXeFG2bxPqXLkdERLSxQVos24c6etsczkgXhyXD3eMnR?cluster=devnet
fn main() -> Result<()> {
    let client = RpcClient::new("https://api.devnet.solana.com");
    let private_key = env::var("SOLANA_PRIVATE_KEY")?;
    let sender_key_pair = Keypair::from_base58_string(&private_key);

    let receiver_key_pair = Keypair::new();
    const ACCOUNT_SPACE: usize = 0;
    let account_min_balance = client.get_minimum_balance_for_rent_exemption(ACCOUNT_SPACE)?;

    let static_receiver_public_key = Pubkey::from_str(STATIC_RECEIVER)?;

    let create_account_instruction = system_instruction::create_account(
        &sender_key_pair.pubkey(),
        &receiver_key_pair.pubkey(),
        account_min_balance,
        ACCOUNT_SPACE as u64,
        &system_program::id(),
    );
    let transfer_instruction = system_instruction::transfer(
        &sender_key_pair.pubkey(),
        &receiver_key_pair.pubkey(),
        TRANSFER_AMOUNT,
    );
    let transfer_to_static_receiver_instruction = system_instruction::transfer(
        &sender_key_pair.pubkey(),
        &static_receiver_public_key,
        STATIC_RECEIVER_TRANSFER_AMOUNT,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[
            create_account_instruction,
            transfer_instruction,
            transfer_to_static_receiver_instruction,
        ],
        Some(&sender_key_pair.pubkey()),
        &[&sender_key_pair, &receiver_key_pair],
        client.get_latest_blockhash()?,
    );
    let signature = client.send_and_confirm_transaction(&transaction)?;

    let explorer_url = util::get_signature_explorer_url(&signature);
    println!("explorer url: {}", explorer_url);

    Ok(())
}
