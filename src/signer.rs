use std::env;

use alloy::signers::{local::PrivateKeySigner, Signer};
use alloy_primitives::FixedBytes;
use hex::{decode, encode as hex_encode};
use dotenv::dotenv;

pub struct LocalSigner {
    signer: PrivateKeySigner,
}

impl LocalSigner {
    pub fn init() -> Self {
        dotenv().ok();

        let private_key = env::var("PRIVATE_KEY").expect("PRIVATE KEY IS REQUIRED");

        let key_bytes = decode(&private_key).expect("Invalid private key format");
        let key_array: [u8; 32] = key_bytes
            .as_slice()
            .try_into()
            .expect("Private key must be 32 bytes long");
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