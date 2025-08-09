use serde::{Deserialize, Serialize};
use reqwest;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTMetadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub attributes: Vec<NFTAttribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTAttribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GotasNFT {
    pub id: String,
    pub url: String,
    pub metadata: NFTMetadata,
    pub blockchain: String,
    pub contract_address: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GotasClient {
    base_url: String,
    api_key: Option<String>,
}

impl GotasClient {
    pub fn new(base_url: String, api_key: Option<String>) -> Self {
        GotasClient { base_url, api_key }
    }
    
    pub async fn create_nft(&self, metadata: NFTMetadata, blockchain: &str) -> Result<GotasNFT> {
        let client = reqwest::Client::new();
        
        let mut request = client
            .post(&format!("{}/api/nfts", self.base_url))
            .json(&serde_json::json!({
                "metadata": metadata,
                "blockchain": blockchain
            }));
            
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request.send().await?;
        let nft: GotasNFT = response.json().await?;
        
        Ok(nft)
    }
    
    pub async fn get_nft(&self, id: &str) -> Result<GotasNFT> {
        let client = reqwest::Client::new();
        
        let mut request = client
            .get(&format!("{}/api/nfts/{}", self.base_url, id));
            
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request.send().await?;
        let nft: GotasNFT = response.json().await?;
        
        Ok(nft)
    }
    
    pub async fn mint_nft(&self, id: &str, wallet_address: &str, passphrase: &str) -> Result<String> {
        let client = reqwest::Client::new();
        
        let mut request = client
            .post(&format!("{}/api/nfts/{}/mint", self.base_url, id))
            .json(&serde_json::json!({
                "wallet_address": wallet_address,
                "passphrase": passphrase
            }));
            
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request.send().await?;
        let result: serde_json::Value = response.json().await?;
        
        // Retornar o hash da transação se disponível
        if let Some(tx_hash) = result.get("transaction_hash") {
            Ok(tx_hash.as_str().unwrap_or("").to_string())
        } else {
            Ok("".to_string())
        }
    }
}