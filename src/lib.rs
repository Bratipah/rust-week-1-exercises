// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // todo!()

    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    // Parse the first 4 bytes (8 hex characters) as little-endian u32
    let version_hex = &raw_tx_hex[0..8];
    hex::decode(version_hex)
        .map_err(|_| "Hex decode error".to_string())
        .and_then(|bytes| {
            if bytes.len() != 4 {
                Err("Invalid version length".to_string())
            } else {
                Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            }
        })
}
