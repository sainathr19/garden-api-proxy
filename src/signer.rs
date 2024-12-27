use std::env;

use alloy::signers::{local::PrivateKeySigner, Signer};
use alloy_primitives::FixedBytes;
use ethers::signers::{coins_bip39::English, MnemonicBuilder};
use hex::encode as hex_encode;
use dotenv::dotenv;

pub struct LocalSigner {
    signer: PrivateKeySigner,
}

impl LocalSigner {
    pub fn init() -> Self {
        dotenv().ok();

        let secret_phrase = env::var("SECRET_PHRASE").expect("SECRET_PHRASE is required");

        // Create wallet from mnemonic
        let wallet = MnemonicBuilder::<English>::default()
        .phrase(secret_phrase.as_str())
        .build()
        .expect("Failed to create wallet from mnemonic");
    
        let private_key_bytes = wallet.signer().to_bytes();
        let key_array: [u8; 32] = private_key_bytes.try_into().expect("Private key must be 32 bytes");
        let fixed_key = FixedBytes::<32>::from(key_array);
        
        let signer = PrivateKeySigner::from_bytes(&fixed_key).expect("Failed to initialize signer");

        Self { signer }
    }

    pub fn address(&self) -> String {
        self.signer.address().to_string()
    }

    pub async fn sign_siwe(&self, message: &String) -> Result<String, Box<dyn std::error::Error>> {
        let sig = self.signer.sign_message(message.as_bytes()).await?;
        Ok(hex_encode(sig.as_bytes()))
    }
}