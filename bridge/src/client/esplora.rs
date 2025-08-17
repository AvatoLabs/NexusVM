use crate::network::NetworkType;

const DEVNET_ESPLORA_URL: &str = "http://localhost:8094/devnet/api/";
// This endpoint accepts non-standard transactions.
// const ALPEN_SIGNET_ESPLORA_URL: &str = "https://esplora-large.devnet-annapurna.stratabtc.org";
const NEXUSVM_SIMNET_ESPLORA_URL: &str = "https://esplora.nexusvmnet.org";

// TODO: Needs to be updated for production environment.
pub fn get_esplora_url(network: NetworkType) -> &'static str {
    match network {
        NetworkType::Devnet => DEVNET_ESPLORA_URL,
        _ => NEXUSVM_SIMNET_ESPLORA_URL,
    }
}
