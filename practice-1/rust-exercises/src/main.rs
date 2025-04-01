use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Instant};

use solana_client::rpc_client::RpcClient;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::{SeedDerivable, Signer}};

const RPC_URL: &str = "https://api.devnet.solana.com";

fn main() {
    generate_parallel_keypair();
    let keypair = load_keypair();
    println!("Loaded pubkey: {}", keypair.pubkey());

    check_balance(&keypair.pubkey());
    airdrop(&keypair.pubkey());
    check_balance(&keypair.pubkey());
}

fn generate_parallel_keypair() {
    let prefix = "anza";

    let num_threads = num_cpus::get();
    println!("Using {} threads to generate keypairs", num_threads);

    let found = Arc::new(AtomicBool::new(false));

    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let found = Arc::clone(&found);
            let prefix = prefix.to_string();

            std::thread::spawn(move || {
                generate_keypair(&prefix, found);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

fn generate_keypair(prefix: &str, found: Arc<AtomicBool>) {
    let start_time = Instant::now(); 

    loop {
        if found.load(Ordering::Relaxed) {
            return;
        }

        let keypair = Keypair::new();
        let pubkey = keypair.pubkey().to_string();

        if pubkey.starts_with(&prefix) {
            println!("Public Key: {}", pubkey);
            let duration = start_time.elapsed();

            println!("Keypair generation took: {:?}", duration);
            println!("Secret Key: {:?}", keypair.to_bytes());
        
            std::fs::write(".env",  format!("SECRET_KEY=\"{:?}\"", keypair.secret().as_bytes()))
                .expect("Failed to write .env file");

            found.store(true, Ordering::Relaxed);

            return;
        }
    }
}

fn load_keypair() -> Keypair {
    dotenv::dotenv().ok();
    let secret_key_str = std::env::var("SECRET_KEY").expect("SECRET_KEY not found in .env");
    println!("Loaded Secret Key: {}", secret_key_str);

    let secret_key_bytes: Vec<u8> = secret_key_str
        .trim_matches(&['[', ']'][..])  
        .split(", ")                    
        .map(|s| s.parse().unwrap())    
        .collect();

    Keypair::from_seed(&secret_key_bytes).expect("Failed to load keypair from .env secret_key_bytes")
}

fn check_balance(pubkey: &Pubkey) {
    let rpc_client = RpcClient::new(RPC_URL.to_string());
    let balance = rpc_client.get_balance(pubkey).unwrap();
    println!("Balance: {}", balance);
}

fn airdrop(pubkey: &Pubkey) {
    let rpc_client = RpcClient::new(RPC_URL.to_string());
    let signature = rpc_client.request_airdrop(pubkey, LAMPORTS_PER_SOL).unwrap();
    println!("Airdrop Signature: {:?}", signature);
}
