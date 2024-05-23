use std::env;

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::Keypair, signer::Signer, system_instruction, system_program,
    transaction::Transaction,
};

// https://explorer.solana.com/tx/3VS6kpTHtGnZ91HYbDofkFpPW2ZJ6S8eQa4UBVTE4hBCNN2TYiS2VHuSDJn3Ugk52ry9zpu4afTRPuR6he1SpPkG?cluster=devnet
fn main() -> Result<()> {
    let client = RpcClient::new("https://api.devnet.solana.com");
    let private_key = env::var("SOLANA_PRIVATE_KEY")?;
    let current_key_pair = Keypair::from_base58_string(&private_key);

    let new_account_key_pair = Keypair::new();
    const ACCOUNT_SPACE: usize = 0;
    let account_min_balance = client.get_minimum_balance_for_rent_exemption(ACCOUNT_SPACE)?;

    let create_account_instruction = system_instruction::create_account(
        &current_key_pair.pubkey(),
        &new_account_key_pair.pubkey(),
        account_min_balance,
        ACCOUNT_SPACE as u64,
        &system_program::id(),
    );
    let transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction],
        Some(&current_key_pair.pubkey()),
        &[&current_key_pair, &new_account_key_pair],
        client.get_latest_blockhash()?,
    );
    let signature = client.send_and_confirm_transaction(&transaction)?;

    let explorer_url = util::get_signature_explorer_url(&signature);
    println!("explorer url: {}", explorer_url);

    Ok(())
}
