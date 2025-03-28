use std::fs;
use std::path::Path;

use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::EncodableKey;

use serde::{Deserialize, Serialize};

const ADMIN_KEYPAIR_PATH: &str = "admin-keypair.json";
const CONFIG_PATH: &str = "faucet-config.json";

#[derive(Serialize, Deserialize)]
struct FaucetConfig {
    admin_keypair: Vec<u8>,
    faucet_pubkey: String,
}

pub fn ensure_admin_keypair() -> Keypair {
    if Path::new(CONFIG_PATH).exists() {
        let data = fs::read_to_string(CONFIG_PATH).expect("Failed to read config file");
        let config: FaucetConfig = serde_json::from_str(&data).expect("Invalid config format");
        println!("✅ Admin keypair found. Loading...");

        Keypair::from_bytes(&config.admin_keypair).expect("Failed to load admin keypair")
    } else {
        println!("⚠️ Admin keypair not found. Generating a new one...");
        let keypair = Keypair::new();

        // Initially save with an empty faucet pubkey
        let config = FaucetConfig {
            admin_keypair: keypair.to_bytes().to_vec(),
            faucet_pubkey: String::new(),
        };

        fs::write(CONFIG_PATH, serde_json::to_string(&config).unwrap())
            .expect("Failed to save config");
        println!("✅ New admin keypair saved to {}", CONFIG_PATH);

        keypair
    }
}

pub fn save_faucet_pubkey(faucet_pubkey: &Pubkey) {
    let data = fs::read_to_string(CONFIG_PATH).expect("Failed to read config file");
    let mut config: FaucetConfig = serde_json::from_str(&data).expect("Invalid config format");

    config.faucet_pubkey = faucet_pubkey.to_string();
    fs::write(CONFIG_PATH, serde_json::to_string(&config).unwrap())
        .expect("Failed to save faucet pubkey");

    println!("✅ Faucet pubkey saved to {}", CONFIG_PATH);
}
