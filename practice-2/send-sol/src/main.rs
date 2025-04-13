use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, instruction::{AccountMeta, Instruction}, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::{Keypair, Signer}, signer::SeedDerivable, system_instruction, transaction::Transaction
};
use std::{env, str::FromStr};
use tokio;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let private_key_string = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let keypair_bytes: Vec<u8> = serde_json::from_str(&private_key_string).unwrap();
    let sender = Keypair::from_seed(&keypair_bytes).expect("Invalid keypair bytes");

    println!("\u{1F511} Our public key is: {}", sender.pubkey());

    let recipient = Pubkey::from_str("8tV9fiyNoDxmBU696XKBsKYiY4ouu1GzckDTHdNxPyZ7").unwrap();
    println!("\u{1F4B8} Attempting to send 0.01 SOL to {}...", recipient);

    let client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    let lamports = (0.01 * LAMPORTS_PER_SOL as f64) as u64;

    let transfer_instruction = system_instruction::transfer(&sender.pubkey(), &recipient, lamports);

    let memo_text = "memo text!";
    println!("\u{1F4DD} memo is: {}", memo_text);

    let memo_program_id = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr").unwrap();
    let memo_instruction = Instruction {
        program_id: memo_program_id,
        accounts: vec![AccountMeta::new(sender.pubkey(), true)],
        data: memo_text.as_bytes().to_vec(),
    };

    let recent_blockhash = client.get_latest_blockhash().await.unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[transfer_instruction, memo_instruction],
        Some(&sender.pubkey()),
        &[&sender],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&tx).await.unwrap();
    println!("\u{2705} Transaction confirmed, signature: {}", signature);
}
