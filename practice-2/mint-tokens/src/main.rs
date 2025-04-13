
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair, signer::{SeedDerivable, Signer}};
use spl_token::{instruction::mint_to, ID as TOKEN_PROGRAM_ID};
use std::{env, str::FromStr};
use anyhow::Result;

fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let secret_key_str = env::var("SECRET_KEY").expect("SECRET_KEY not found in .env");
    let bytes: Vec<u8> = serde_json::from_str(&secret_key_str)?;
    let sender = Keypair::from_seed(&bytes).unwrap();

    let rpc_url = solana_client::rpc_client::RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    let token_mint_pubkey = Pubkey::from_str("D2sKqXgv5c2MnicwSkXfKC6HushoFuWJ97kbwqpn6oLU")?;
    let recipient_token_account = Pubkey::from_str("DGHhQ5s4VqZzbABtpRRCWee9aes5GNMKxrgKQMg65BHU")?;

    let decimals = 2;
    let amount = 10 * 10u64.pow(decimals);

    let ix = mint_to(
        &TOKEN_PROGRAM_ID,
        &token_mint_pubkey,
        &recipient_token_account,
        &sender.pubkey(),
        &[],
        amount,
    )?;

    let recent_blockhash = rpc_url.get_latest_blockhash()?;

    let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[ix],
        Some(&sender.pubkey()),
        &[&sender],
        recent_blockhash,
    );

    let signature = rpc_url.send_and_confirm_transaction(&tx)?;
    println!("✅ Success!");
    println!("Explorer: https://explorer.solana.com/tx/{}?cluster=devnet", signature);

    Ok(())
}
