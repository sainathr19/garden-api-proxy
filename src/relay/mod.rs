pub mod types;
use reqwest::Client;
use types::{FetchNonceResponse, ResponseStatus, SiweMessage, VerifySignatureResponse};

pub struct Relay;

impl Relay{
    pub async fn get_nonce() -> Result<String, Box<dyn std::error::Error>> {
        let client = Client::new();
    
        let response = client
            .get("https://evm-swapper-relay.onrender.com/nonce")
            .send()
            .await?;
    
        let parsed_response = response.json::<FetchNonceResponse>().await?;
    
        match parsed_response.status {
            ResponseStatus::Error => {
                Err(parsed_response.error.unwrap_or_else(|| "Unknown error occured".to_string()).into())
            },
            ResponseStatus::Ok => {
                parsed_response.result.ok_or_else(|| "No result found".into())
            }
        }
    }

    pub async fn verify_signature(message : String, nonce : String , sig : String) -> Result<String, Box<dyn std::error::Error>> {
        let client = Client::new();
        let payload = SiweMessage{
            signature : sig,
            nonce,
            message
        };
        let response  = client.post("https://evm-swapper-relay.onrender.com/verify").json(&payload).send().await?;

        let response_text = response.text().await?;

        let parsed_response: VerifySignatureResponse = serde_json::from_str(&response_text)?;
        match parsed_response.status{
            ResponseStatus::Ok => {
                parsed_response.result.ok_or_else(|| "No Result".into())
            }
            ResponseStatus::Error => {
                Err(parsed_response.error.unwrap_or_else(|| "Unknown Error occured".to_string()).into())
            },
        }
    }
}