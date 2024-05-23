use solana_sdk::signature::Signature;

/// Get explorer url for signature in devnet cluster
pub fn get_signature_explorer_url(signature: &Signature) -> String {
    format!("https://explorer.solana.com/tx/{}?cluster=devnet", signature)
}
