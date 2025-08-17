use serde::{Deserialize, Serialize};

/// Tondi network types
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NetworkType {
    Mainnet,
    Testnet,
    Devnet,
    Simnet,
}

impl From<NetworkType> for bitcoin::Network {
    fn from(network_type: NetworkType) -> Self {
        match network_type {
            NetworkType::Mainnet => bitcoin::Network::Bitcoin,
            NetworkType::Testnet => bitcoin::Network::Testnet,
            NetworkType::Devnet => bitcoin::Network::Regtest, // Map to Regtest for compatibility
            NetworkType::Simnet => bitcoin::Network::Signet,  // Map to Signet for compatibility
        }
    }
}

impl From<bitcoin::Network> for NetworkType {
    fn from(network: bitcoin::Network) -> Self {
        match network {
            bitcoin::Network::Bitcoin => NetworkType::Mainnet,
            bitcoin::Network::Testnet => NetworkType::Testnet,
            bitcoin::Network::Regtest => NetworkType::Devnet,
            bitcoin::Network::Signet => NetworkType::Simnet,
            _ => NetworkType::Mainnet, // Default fallback
        }
    }
}

impl NetworkType {
    pub fn default_rpc_port(&self) -> u16 {
        match self {
            NetworkType::Mainnet => 16110,
            NetworkType::Testnet => 16210,
            NetworkType::Simnet => 16510,
            NetworkType::Devnet => 16610,
        }
    }

    pub fn default_borsh_rpc_port(&self) -> u16 {
        match self {
            NetworkType::Mainnet => 17110,
            NetworkType::Testnet => 17210,
            NetworkType::Simnet => 17510,
            NetworkType::Devnet => 17610,
        }
    }

    pub fn default_json_rpc_port(&self) -> u16 {
        match self {
            NetworkType::Mainnet => 18110,
            NetworkType::Testnet => 18210,
            NetworkType::Simnet => 18510,
            NetworkType::Devnet => 18610,
        }
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        static NETWORK_TYPES: [NetworkType; 4] = [
            NetworkType::Mainnet,
            NetworkType::Testnet,
            NetworkType::Devnet,
            NetworkType::Simnet,
        ];
        NETWORK_TYPES.iter().copied()
    }
}

impl std::str::FromStr for NetworkType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mainnet" => Ok(NetworkType::Mainnet),
            "testnet" => Ok(NetworkType::Testnet),
            "simnet" => Ok(NetworkType::Simnet),
            "devnet" => Ok(NetworkType::Devnet),
            _ => Err(format!("Invalid network type: {}", s)),
        }
    }
}

impl std::fmt::Display for NetworkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            NetworkType::Mainnet => "mainnet",
            NetworkType::Testnet => "testnet",
            NetworkType::Simnet => "simnet",
            NetworkType::Devnet => "devnet",
        };
        f.write_str(s)
    }
}
