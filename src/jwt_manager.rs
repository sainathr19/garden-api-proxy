use chrono::{DateTime, Utc};
use crate::{relay::Relay, signer::{self}};


use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
}

#[derive(Clone)]
pub struct JwtManager {
    token: Option<String>,
}

impl JwtManager {

    pub fn new() -> Self {
        Self { token: None }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }


    pub async fn get_token(&mut self, signer: Arc<tokio::sync::Mutex<signer::LocalSigner>>) -> Result<String, String> {
        // First check if we have a valid token
        if let Some(token) = &self.token {
            let is_valid = match self.validate_token() {
                Ok(valid) =>valid,
                Err(err) => {
                    println!("Error validating token {:?}", err);
                    false
                }
            };
            
            if is_valid {
                return Ok(token.clone());
            } else{
                println!()
            }
        }
    
        // If we don't have a valid token, generate a new one
        self.generate_new_jwt(signer).await
    }
    pub fn validate_token(&self) -> Result<bool, Box<dyn Error>> {
        if let Some(token) = &self.token {
            let decoding_key = DecodingKey::from_secret(b"dummy_secret_key");
            let mut validation = Validation::new(Algorithm::HS256);
            validation.validate_exp = false;
            validation.insecure_disable_signature_validation();
    
            match decode::<Claims>(token, &decoding_key, &validation) {
                Ok(decoded_token) => {
                    let exp_timestamp = decoded_token.claims.exp;
                    if let Some(exp_time) = DateTime::from_timestamp(exp_timestamp as i64, 0) {
                        if Utc::now() < exp_time {
                            return Ok(true);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Token decode error: {:?}", err);
                }
            }
        }
        Ok(false)
    }

    pub async fn generate_new_jwt(&mut self, signer : Arc<tokio::sync::Mutex<signer::LocalSigner>>) -> Result<String, String> {
        let nonce = match Relay::get_nonce().await{
            Ok(nonce) => nonce,
            Err(err) => {
                return Err(format!("Error fetching nonce: {}",err).into())
            }
        };

        let signer = signer.lock().await;

        let msg = r#"testnet.garden.finance wants you to sign in with your Ethereum account:
0x6Da01670d8fc844e736095918bbE11fE8D564163

Garden.fi

URI: https://testnet.garden.finance
Version: 1
Chain ID: 11155111
Nonce: kEWepMt9knR6lWJ6A
Issued At: 2021-12-07T18:28:18.807Z"#;
        let msg = msg.replace("kEWepMt9knR6lWJ6A", &nonce);
        let msg = msg.replace(
            "0x6Da01670d8fc844e736095918bbE11fE8D564163",
            &signer.address(),
        );

        let signature = signer.sign_siwe(&msg).await.map_err(|_| "Signing failed")?;
        let jwt_token = match Relay::verify_signature(msg, nonce, signature).await{
            Ok(token) => token,
            Err(err) => {
                    return Err(format!("Error verifying signature: {}",err).into())
            }
        };

        self.set_token(jwt_token.clone());    
        Ok(jwt_token)
    }
    
}

