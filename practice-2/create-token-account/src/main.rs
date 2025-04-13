use std::env;
use std::str::FromStr;

use anyhow::Result;
use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer}, signer::SeedDerivable,
};
use spl_associated_token_account::{create_associated_token_account, get_associated_token_address};


fn main() -> Result<()> {
    dotenv().ok();
    let payer = load_keypair_from_env()?;

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    println!("🔑 Our public key is: {}", payer.pubkey());

    let mint_pubkey = Pubkey::from_str("D2sKqXgv5c2MnicwSkXfKC6HushoFuWJ97kbwqpn6oLU")?;
    let recipient = Pubkey::from_str("8tV9fiyNoDxmBU696XKBsKYiY4ouu1GzckDTHdNxPyZ7")?;

    let ata_address = get_associated_token_address(&recipient, &mint_pubkey);

    if client.get_account(&ata_address).is_err() {
        let ix = create_associated_token_account(&payer.pubkey(), &recipient, &mint_pubkey);
        let recent_blockhash = client.get_latest_blockhash()?;
        let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );
        client.send_and_confirm_transaction(&tx)?;
        println!("✅ Created new ATA.");
    } else {
        println!("✅ ATA already exists.");
    }

    println!("🎯 Token Account: {}", ata_address);
    println!(
        "🔗 Explorer link: https://explorer.solana.com/address/{}?cluster=devnet",
        ata_address
    );

    Ok(())
}

fn load_keypair_from_env() -> Result<Keypair> {
    let key_str = env::var("SECRET_KEY")?;
    let bytes: Vec<u8> = serde_json::from_str(&key_str)?;
    Ok(Keypair::from_seed(&bytes).unwrap())
}