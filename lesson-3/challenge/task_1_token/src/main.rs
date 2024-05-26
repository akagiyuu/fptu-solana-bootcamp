use std::env;

use anyhow::Result;
use mpl_token_metadata::instructions as metadata_instruction;
use mpl_token_metadata::instructions::CreateMetadataAccountV3InstructionArgs;
use mpl_token_metadata::types::DataV2;
use mpl_token_metadata::ID as metadata_program_id;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::system_program;
use solana_sdk::transaction::Transaction;
use solana_sdk::{program_pack::Pack, signature::Keypair, signer::Signer, system_instruction};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction as token_instruction;
use spl_token::state::Mint;
use spl_token::ID as token_program_id;

const MINT_DECIMAL: u8 = 6;
const TOKEN_TO_MINT_AMOUNT: u64 = 100;

// https://explorer.solana.com/tx/5WFed3Tv7aa72qVZGD4sjiMjX6TL8H15styqEP3TRk4vEK3ejYGzmoyTzs3qeBv2esoRZZ27QzzhWaWq4Zdp6ucB?cluster=devnet
fn main() -> Result<()> {
    let client = RpcClient::new("https://api.devnet.solana.com");

    let private_key = env::var("SOLANA_PRIVATE_KEY")?;
    let owner_keys = Keypair::from_base58_string(&private_key);

    let mint_account_keys = Keypair::new();
    let (metadata_account_address, _) = Pubkey::find_program_address(
        &[
            b"metadata",
            &metadata_program_id.to_bytes(),
            &mint_account_keys.pubkey().to_bytes(),
        ],
        &metadata_program_id,
    );

    let associated_token_account_address =
        get_associated_token_address(&owner_keys.pubkey(), &mint_account_keys.pubkey());

    let create_mint_account = system_instruction::create_account(
        &owner_keys.pubkey(),
        &mint_account_keys.pubkey(),
        client.get_minimum_balance_for_rent_exemption(Mint::LEN)?,
        Mint::LEN as u64,
        &token_program_id,
    );
    let init_mint_account = token_instruction::initialize_mint(
        &token_program_id,
        &mint_account_keys.pubkey(),
        &owner_keys.pubkey(),
        Some(&mint_account_keys.pubkey()),
        MINT_DECIMAL,
    )?;
    let create_metadata_account = metadata_instruction::CreateMetadataAccountV3 {
        metadata: metadata_account_address,
        mint: mint_account_keys.pubkey(),
        mint_authority: owner_keys.pubkey(),
        payer: owner_keys.pubkey(),
        update_authority: (owner_keys.pubkey(), true),
        system_program: system_program::ID,
        rent: None
    }
    .instruction(CreateMetadataAccountV3InstructionArgs {
        data: DataV2 {
            name: "Test Token".to_string(),
            symbol: "TT".to_string(),
            uri: "https://raw.githubusercontent.com/akagiyuu/fptu-solana-bootcamp/main/assets/tt-token.json".to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        is_mutable: true,
        collection_details: None,
    });

    let create_associated_token_account =
        spl_associated_token_account::instruction::create_associated_token_account(
            &owner_keys.pubkey(),
            &owner_keys.pubkey(),
            &mint_account_keys.pubkey(),
            &token_program_id,
        );
    let mint_to_account = token_instruction::mint_to(
        &token_program_id,
        &mint_account_keys.pubkey(),
        &associated_token_account_address,
        &owner_keys.pubkey(),
        &[&owner_keys.pubkey()],
        TOKEN_TO_MINT_AMOUNT,
    )?;

    let mut instructions = Vec::with_capacity(5);
    instructions.push(create_mint_account);
    instructions.push(init_mint_account);
    instructions.push(create_metadata_account);
    // if associated token account does not exist
    if client
        .get_account(&associated_token_account_address)
        .is_err()
    {
        instructions.push(create_associated_token_account);
    }
    instructions.push(mint_to_account);

    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&owner_keys.pubkey()),
        &[&owner_keys, &mint_account_keys],
        client.get_latest_blockhash()?,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;

    let explorer_url = util::get_signature_explorer_url(&signature);
    println!("explorer url: {}", explorer_url);

    Ok(())
}
