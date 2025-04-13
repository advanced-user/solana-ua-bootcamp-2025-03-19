use std::{env, str::FromStr};
use anyhow::Result;

use mpl_token_metadata::{instructions::CreateMetadataAccountV3Builder, types::DataV2};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::Pubkey, signature::{Keypair, Signer}, signer::SeedDerivable, transaction::Transaction
};

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
    let payer = load_keypair_from_env()?;

    println!("🔑 Our public key is: {}", payer.pubkey());

    let mint_pubkey = Pubkey::from_str("D2sKqXgv5c2MnicwSkXfKC6HushoFuWJ97kbwqpn6oLU").unwrap();

    let metadata_program_id = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap();

    let (metadata_pda, _) = Pubkey::find_program_address(
        &[
            b"metadata",
            metadata_program_id.as_ref(),
            mint_pubkey.as_ref(),
        ],
        &metadata_program_id,
    );

    let data = DataV2 {
        name: "Solana UA Bootcamp 2025-03-19".to_string(),
        symbol: "UAB-3".to_string(),
        uri: "https://arweave.net/1234".to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    let ix =  CreateMetadataAccountV3Builder::new()
    .metadata(metadata_pda)
    .mint(mint_pubkey)
    .mint_authority(payer.pubkey())
    .payer(payer.pubkey())
    .update_authority(payer.pubkey(), true)
    .data(data)
    .is_mutable(true)
    .instruction();

    let recent_blockhash = client.get_latest_blockhash().unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let sig = client.send_and_confirm_transaction(&tx).unwrap();
    println!("✅ Metadata created: https://explorer.solana.com/tx/{}?cluster=devnet", sig);

    Ok(())
}

fn load_keypair_from_env() -> Result<Keypair> {
    let key_str = env::var("SECRET_KEY")?;
    let bytes: Vec<u8> = serde_json::from_str(&key_str)?;
    Ok(Keypair::from_seed(&bytes).unwrap())
}