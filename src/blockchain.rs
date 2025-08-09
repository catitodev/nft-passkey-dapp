use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    pub name: String,
    pub rpc_url: String,
    pub chain_id: u64,
    pub symbol: String,
    pub explorer_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainManager {
    pub blockchains: HashMap<String, BlockchainConfig>,
}

impl BlockchainManager {
    pub fn new() -> Self {
        let mut blockchains = HashMap::new();
        
        // Configuração para Ethereum
        blockchains.insert("ethereum".to_string(), BlockchainConfig {
            name: "Ethereum".to_string(),
            rpc_url: "https://mainnet.infura.io/v3/YOUR_INFURA_PROJECT_ID".to_string(),
            chain_id: 1,
            symbol: "ETH".to_string(),
            explorer_url: Some("https://etherscan.io".to_string()),
        });
        
        // Configuração para Polygon
        blockchains.insert("polygon".to_string(), BlockchainConfig {
            name: "Polygon".to_string(),
            rpc_url: "https://polygon-rpc.com".to_string(),
            chain_id: 137,
            symbol: "MATIC".to_string(),
            explorer_url: Some("https://polygonscan.com".to_string()),
        });
        
        // Configuração para Solana
        blockchains.insert("solana".to_string(), BlockchainConfig {
            name: "Solana".to_string(),
            rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            chain_id: 0, // Solana não usa chain_id da mesma forma
            symbol: "SOL".to_string(),
            explorer_url: Some("https://solscan.io".to_string()),
        });
        
        // Configuração para Celo
        blockchains.insert("celo".to_string(), BlockchainConfig {
            name: "Celo".to_string(),
            rpc_url: "https://forno.celo.org".to_string(),
            chain_id: 42220,
            symbol: "CELO".to_string(),
            explorer_url: Some("https://celoscan.io".to_string()),
        });
        
        BlockchainManager { blockchains }
    }
    
    pub fn get_blockchain(&self, name: &str) -> Option<&BlockchainConfig> {
        self.blockchains.get(name)
    }
    
    pub fn list_blockchains(&self) -> Vec<&BlockchainConfig> {
        self.blockchains.values().collect()
    }
}