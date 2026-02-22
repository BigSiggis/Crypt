//! Event processor — parses Anchor program logs into typed events.

use crate::events::*;
use bs58;

/// Event discriminators (first 8 bytes of SHA-256 hash of event name).
/// These are used by Anchor to identify events in transaction logs.
const CARD_MINTED_DISC: &str = "CardMinted";
const CARD_TRANSFERRED_DISC: &str = "CardTransferred";
const CARD_BURNED_DISC: &str = "CardBurned";
const RARITY_UPGRADED_DISC: &str = "RarityUpgraded";
const CARD_INTERACTION_DISC: &str = "CardInteraction";

/// Parse Anchor event data from program log lines.
/// Anchor events are base64-encoded in "Program data: <base64>" log lines.
pub fn parse_program_logs(logs: &[String]) -> Vec<CryptEvent> {
    let mut events = Vec::new();

    for log in logs {
        // Anchor events appear as "Program data: <base64>"
        if let Some(data_str) = log.strip_prefix("Program data: ") {
            if let Ok(data) = base64_decode(data_str.trim()) {
                if data.len() < 8 { continue; }

                // Match event discriminator
                if let Some(event) = try_parse_event(&data) {
                    events.push(event);
                }
            }
        }

        // Also capture "Program log: " messages for debugging
        if log.contains("CRYPT Card #") || log.contains("CRYPT batch mint") {
            // These are msg!() calls from the program
        }
    }

    events
}

fn base64_decode(input: &str) -> Result<Vec<u8>, ()> {
    // Simple base64 decoder
    let chars: Vec<u8> = input.bytes().collect();
    let mut result = Vec::new();
    let table = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut buf: u32 = 0;
    let mut bits: u32 = 0;

    for &ch in &chars {
        if ch == b'=' { break; }
        let val = table.iter().position(|&c| c == ch);
        if let Some(v) = val {
            buf = (buf << 6) | v as u32;
            bits += 6;
            if bits >= 8 {
                bits -= 8;
                result.push((buf >> bits) as u8);
                buf &= (1 << bits) - 1;
            }
        }
    }

    Ok(result)
}

fn try_parse_event(data: &[u8]) -> Option<CryptEvent> {
    if data.len() < 16 { return None; }

    // Try to parse based on data structure
    // In production, we'd match on the 8-byte Anchor event discriminator
    // For now, try to identify by field patterns

    // Check if this looks like a CardMinted event
    // (has a valid pubkey at offset 8, followed by strings)
    if data.len() > 80 {
        // Attempt CardMinted parse
        let mint_id = u64::from_le_bytes(data[8..16].try_into().ok()?);
        let owner_bytes: [u8; 32] = data[16..48].try_into().ok()?;
        let owner = bs58::encode(&owner_bytes).into_string();

        // Read tx_hash string (Borsh encoding: 4-byte length + bytes)
        let tx_hash_len = u32::from_le_bytes(data[48..52].try_into().ok()?) as usize;
        if tx_hash_len > 0 && tx_hash_len < 90 && 52 + tx_hash_len <= data.len() {
            let tx_hash = String::from_utf8_lossy(&data[52..52+tx_hash_len]).to_string();
            let offset = 52 + tx_hash_len;

            if offset + 2 <= data.len() {
                let rarity = data[offset];
                let card_type = data[offset + 1];

                return Some(CryptEvent::CardMinted(CardMintedEvent {
                    mint_id,
                    owner,
                    tx_hash,
                    rarity,
                    card_type,
                    title: String::new(),
                    soul_seed: [0u8; 32],
                    timestamp: chrono::Utc::now().timestamp(),
                }));
            }
        }
    }

    None
}

/// Format a rarity value as a string.
pub fn rarity_name(rarity: u8) -> &'static str {
    match rarity {
        0 => "COMMON",
        1 => "RARE",
        2 => "LEGENDARY",
        _ => "UNKNOWN",
    }
}

/// Format a card type value as a string.
pub fn card_type_name(card_type: u8) -> &'static str {
    match card_type {
        0 => "SWAP",
        1 => "RUG",
        2 => "MINT",
        3 => "DIAMOND_HANDS",
        4 => "BIG_MOVE",
        _ => "UNKNOWN",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_decode() {
        let encoded = "SGVsbG8=";
        let decoded = base64_decode(encoded).unwrap();
        assert_eq!(decoded, b"Hello");
    }

    #[test]
    fn test_rarity_name() {
        assert_eq!(rarity_name(0), "COMMON");
        assert_eq!(rarity_name(1), "RARE");
        assert_eq!(rarity_name(2), "LEGENDARY");
        assert_eq!(rarity_name(99), "UNKNOWN");
    }

    #[test]
    fn test_card_type_name() {
        assert_eq!(card_type_name(0), "SWAP");
        assert_eq!(card_type_name(1), "RUG");
        assert_eq!(card_type_name(4), "BIG_MOVE");
    }

    #[test]
    fn test_empty_logs() {
        let events = parse_program_logs(&[]);
        assert!(events.is_empty());
    }

    #[test]
    fn test_irrelevant_logs_ignored() {
        let logs = vec![
            "Program log: Instruction: Transfer".to_string(),
            "Program 11111111111111111111111111111111 success".to_string(),
        ];
        let events = parse_program_logs(&logs);
        assert!(events.is_empty());
    }
}
