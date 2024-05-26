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
use spl_token::instruction::{self as token_instruction, AuthorityType};
use spl_token::state::Mint;
use spl_token::ID as token_program_id;

// 10%
const NFT_ROYALTY: u16 = 1000;

// https://explorer.solana.com/tx/4d3NSe4hHSJhsTMs6qfzVbCKVgXuhYu5kHpQ4wVG2fEcVhrF7Pmwrpd51EqbEJGSXwRNrDzYevGVnUvZY6p7o4yz?cluster=devnet
fn main() -> Result<()> {
    let client = RpcClient::new("https://api.devnet.solana.com");

    let private_key = env::var("SOLANA_PRIVATE_KEY")?;
    let owner_keys = Keypair::from_base58_string(&private_key);

    let nft_mint_account_keys = Keypair::new();
    let (metadata_account_address, _) = Pubkey::find_program_address(
        &[
            b"metadata",
            &metadata_program_id.to_bytes(),
            &nft_mint_account_keys.pubkey().to_bytes(),
        ],
        &metadata_program_id,
    );

    let nft_address =
        get_associated_token_address(&owner_keys.pubkey(), &nft_mint_account_keys.pubkey());

    let create_nft_mint_account = system_instruction::create_account(
        &owner_keys.pubkey(),
        &nft_mint_account_keys.pubkey(),
        client.get_minimum_balance_for_rent_exemption(Mint::LEN)?,
        Mint::LEN as u64,
        &token_program_id,
    );
    let init_nft_mint_account = token_instruction::initialize_mint(
        &token_program_id,
        &nft_mint_account_keys.pubkey(),
        &owner_keys.pubkey(),
        Some(&nft_mint_account_keys.pubkey()),
        0,
    )?;
    let create_metadata_account = metadata_instruction::CreateMetadataAccountV3 {
        metadata: metadata_account_address,
        mint: nft_mint_account_keys.pubkey(),
        mint_authority: owner_keys.pubkey(),
        payer: owner_keys.pubkey(),
        update_authority: (owner_keys.pubkey(), true),
        system_program: system_program::ID,
        rent: None
    }
    .instruction(CreateMetadataAccountV3InstructionArgs {
        data: DataV2 {
            name: "Solana bootcamp logo".to_string(),
            symbol: "SBL".to_string(),
            uri: "https://raw.githubusercontent.com/akagiyuu/fptu-solana-bootcamp/main/assets/solana-bootcamp-logo.json".to_string(),
            seller_fee_basis_points: NFT_ROYALTY,
            creators: None,
            collection: None,
            uses: None,
        },
        is_mutable: true,
        collection_details: None,
    });

    let create_nft = spl_associated_token_account::instruction::create_associated_token_account(
        &owner_keys.pubkey(),
        &owner_keys.pubkey(),
        &nft_mint_account_keys.pubkey(),
        &token_program_id,
    );
    let init_nft = token_instruction::mint_to(
        &token_program_id,
        &nft_mint_account_keys.pubkey(),
        &nft_address,
        &owner_keys.pubkey(),
        &[&owner_keys.pubkey()],
        1,
    )?;
    let remove_mint_authority = token_instruction::set_authority(
        &token_program_id,
        &nft_mint_account_keys.pubkey(),
        None,
        AuthorityType::MintTokens,
        &owner_keys.pubkey(),
        &[&owner_keys.pubkey(), &nft_mint_account_keys.pubkey()],
    )?;

    let transaction = Transaction::new_signed_with_payer(
        &[
            create_nft_mint_account,
            init_nft_mint_account,
            create_metadata_account,
            create_nft,
            init_nft,
            remove_mint_authority,
        ],
        Some(&owner_keys.pubkey()),
        &[&owner_keys, &nft_mint_account_keys],
        client.get_latest_blockhash()?,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;

    let explorer_url = util::get_signature_explorer_url(&signature);
    println!("explorer url: {}", explorer_url);

    Ok(())
}
