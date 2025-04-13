use std::env;

use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, program_pack::Pack, signature::{Keypair, Signer}, signer::SeedDerivable
};
use spl_token::instruction::initialize_mint;
use solana_sdk::transaction::Transaction;
use solana_sdk::system_instruction;


fn main() {
    dotenv().ok();
    let payer = load_keypair_from_env();

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    println!("🔑 Our public key is: {}", payer.pubkey());

    let mint_account = Keypair::new();
    let mint_rent = client.get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN).unwrap();

    let blockhash = client.get_latest_blockhash().unwrap();

    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint_account.pubkey(),
        mint_rent,
        spl_token::state::Mint::LEN as u64,
        &spl_token::id(),
    );

    let init_mint_ix = initialize_mint(
        &spl_token::id(),
        &mint_account.pubkey(),
        &payer.pubkey(),
        None,
        2, 
    ).unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[create_account_ix, init_mint_ix],
        Some(&payer.pubkey()),
        &[&payer, &mint_account],
        blockhash,
    );

    client.send_and_confirm_transaction(&tx).unwrap();

    println!("✅ Token Mint: https://explorer.solana.com/address/{}?cluster=devnet", mint_account.pubkey());
}

fn load_keypair_from_env() -> Keypair {
    let key_str = env::var("SECRET_KEY").unwrap();
    let bytes: Vec<u8> = serde_json::from_str(&key_str).unwrap();
    let secret: &[u8] = &bytes;
    Keypair::from_seed(secret).unwrap()
}