//! Crypt RPC client for interacting with the on-chain program.

use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use crate::error::CryptSdkError;
use crate::types::*;

/// Program ID for the Crypt on-chain program.
pub const PROGRAM_ID: &str = "CRYPTxGraveyardSo1ana1111111111111111111111";

/// High-level client for the Crypt Solana program.
pub struct CryptClient {
    rpc: RpcClient,
    program_id: Pubkey,
}

impl CryptClient {
    /// Create a new client connected to a Solana RPC endpoint.
    pub fn new(rpc_url: &str) -> Self {
        let program_id = Pubkey::from_str(PROGRAM_ID)
            .expect("Invalid program ID");
        Self {
            rpc: RpcClient::new(rpc_url.to_string()),
            program_id,
        }
    }

    /// Create a client with a custom program ID (for testing).
    pub fn with_program_id(rpc_url: &str, program_id: &str) -> Result<Self, CryptSdkError> {
        let pid = Pubkey::from_str(program_id)
            .map_err(|e| CryptSdkError::InvalidAddress(e.to_string()))?;
        Ok(Self {
            rpc: RpcClient::new(rpc_url.to_string()),
            program_id: pid,
        })
    }

    /// Get the collection PDA address.
    pub fn collection_address(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[b"collection"], &self.program_id)
    }

    /// Get a card PDA address from tx_hash and minter.
    pub fn card_address(&self, tx_hash: &str, minter: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[b"card", tx_hash.as_bytes(), minter.as_ref()],
            &self.program_id,
        )
    }

    /// Get an interaction PDA address.
    pub fn interaction_address(&self, card: &Pubkey, user: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[b"interaction", card.as_ref(), user.as_ref()],
            &self.program_id,
        )
    }

    /// Fetch collection statistics from on-chain data.
    pub fn get_collection_stats(&self) -> Result<CollectionStats, CryptSdkError> {
        let (pda, _) = self.collection_address();
        let account = self.rpc.get_account(&pda)?;

        if account.data.len() < 8 + 32 + 8 + 8 {
            return Err(CryptSdkError::CollectionNotInitialized);
        }

        // Parse account data (skip 8-byte Anchor discriminator)
        let data = &account.data[8..];
        let authority = Pubkey::try_from(&data[0..32])
            .map_err(|_| CryptSdkError::Serialization("Invalid authority".into()))?;
        let total_minted = u64::from_le_bytes(
            data[32..40].try_into()
                .map_err(|_| CryptSdkError::Serialization("Invalid total_minted".into()))?
        );
        let max_supply = u64::from_le_bytes(
            data[40..48].try_into()
                .map_err(|_| CryptSdkError::Serialization("Invalid max_supply".into()))?
        );

        Ok(CollectionStats {
            authority,
            total_minted,
            max_supply,
            mint_fee: 0,
            paused: false,
            created_at: 0,
        })
    }

    /// Check if an account exists on-chain.
    pub fn account_exists(&self, address: &Pubkey) -> bool {
        self.rpc.get_account(address).is_ok()
    }

    /// Get SOL balance for an address.
    pub fn get_balance(&self, address: &Pubkey) -> Result<u64, CryptSdkError> {
        Ok(self.rpc.get_balance(address)?)
    }

    /// Get the program ID.
    pub fn program_id(&self) -> &Pubkey {
        &self.program_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_pda_derivation() {
        let client = CryptClient::new("https://api.devnet.solana.com");
        let (pda, bump) = client.collection_address();
        assert_ne!(pda, Pubkey::default());
        assert!(bump <= 255);
    }

    #[test]
    fn test_card_pda_derivation() {
        let client = CryptClient::new("https://api.devnet.solana.com");
        let minter = Pubkey::new_unique();
        let (pda1, _) = client.card_address("tx_hash_1", &minter);
        let (pda2, _) = client.card_address("tx_hash_2", &minter);
        assert_ne!(pda1, pda2);
    }

    #[test]
    fn test_custom_program_id() {
        let client = CryptClient::with_program_id(
            "https://api.devnet.solana.com",
            "11111111111111111111111111111111"
        );
        assert!(client.is_ok());
    }
}
