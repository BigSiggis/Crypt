//! In-memory event store.
//! In production, this would be backed by Postgres or DynamoDB.

use std::collections::HashMap;
use crate::events::*;
use colored::Colorize;

/// Indexed card data.
#[derive(Debug, Clone)]
pub struct IndexedCard {
    pub mint_id: u64,
    pub owner: String,
    pub tx_hash: String,
    pub rarity: u8,
    pub card_type: u8,
    pub title: String,
    pub interaction_count: u64,
    pub minted_at: i64,
    pub burned: bool,
}

/// In-memory store for indexed Crypt data.
pub struct InMemoryStore {
    cards: HashMap<u64, IndexedCard>,
    owner_cards: HashMap<String, Vec<u64>>,
    total_minted: u64,
    total_burned: u64,
    total_transfers: u64,
    total_interactions: u64,
    rarity_counts: [u64; 3],
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            cards: HashMap::new(),
            owner_cards: HashMap::new(),
            total_minted: 0,
            total_burned: 0,
            total_transfers: 0,
            total_interactions: 0,
            rarity_counts: [0; 3],
        }
    }

    /// Process a Crypt event and update the index.
    pub fn process_event(&mut self, event: &CryptEvent) {
        match event {
            CryptEvent::CardMinted(e) => {
                self.cards.insert(e.mint_id, IndexedCard {
                    mint_id: e.mint_id,
                    owner: e.owner.clone(),
                    tx_hash: e.tx_hash.clone(),
                    rarity: e.rarity,
                    card_type: e.card_type,
                    title: e.title.clone(),
                    interaction_count: 0,
                    minted_at: e.timestamp,
                    burned: false,
                });
                self.owner_cards
                    .entry(e.owner.clone())
                    .or_default()
                    .push(e.mint_id);
                self.total_minted += 1;
                if (e.rarity as usize) < 3 {
                    self.rarity_counts[e.rarity as usize] += 1;
                }

                println!(
                    "  {} Card #{} minted — {} [{}]",
                    "MINT".bright_green(),
                    e.mint_id,
                    e.title,
                    match e.rarity {
                        0 => "COMMON".white(),
                        1 => "RARE".bright_cyan(),
                        2 => "LEGENDARY".bright_magenta(),
                        _ => "???".white(),
                    }
                );
            }

            CryptEvent::CardTransferred(e) => {
                if let Some(card) = self.cards.get_mut(&e.mint_id) {
                    // Remove from old owner
                    if let Some(cards) = self.owner_cards.get_mut(&e.from) {
                        cards.retain(|&id| id != e.mint_id);
                    }
                    // Update owner
                    card.owner = e.to.clone();
                    self.owner_cards
                        .entry(e.to.clone())
                        .or_default()
                        .push(e.mint_id);
                }
                self.total_transfers += 1;

                println!(
                    "  {} Card #{} transferred: {} → {}",
                    "XFER".bright_yellow(),
                    e.mint_id,
                    &e.from[..8],
                    &e.to[..8],
                );
            }

            CryptEvent::CardBurned(e) => {
                if let Some(card) = self.cards.get_mut(&e.mint_id) {
                    card.burned = true;
                    if let Some(cards) = self.owner_cards.get_mut(&card.owner) {
                        cards.retain(|&id| id != e.mint_id);
                    }
                }
                self.total_burned += 1;

                println!(
                    "  {} Card #{} burned by {}",
                    "BURN".bright_red(),
                    e.mint_id,
                    &e.owner[..8],
                );
            }

            CryptEvent::RarityUpgraded(e) => {
                if let Some(card) = self.cards.get_mut(&e.mint_id) {
                    if (card.rarity as usize) < 3 {
                        self.rarity_counts[card.rarity as usize] -= 1;
                    }
                    card.rarity = e.new_rarity;
                    if (e.new_rarity as usize) < 3 {
                        self.rarity_counts[e.new_rarity as usize] += 1;
                    }
                }

                println!(
                    "  {} Card #{} upgraded: {} → {}",
                    "UP".bright_magenta(),
                    e.mint_id,
                    crate::processor::rarity_name(e.old_rarity),
                    crate::processor::rarity_name(e.new_rarity),
                );
            }

            CryptEvent::CardInteraction(e) => {
                if let Some(card) = self.cards.get_mut(&e.card_mint_id) {
                    card.interaction_count += 1;
                }
                self.total_interactions += 1;
            }
        }
    }

    /// Get cards owned by a specific wallet.
    pub fn get_cards_by_owner(&self, owner: &str) -> Vec<&IndexedCard> {
        self.owner_cards
            .get(owner)
            .map(|ids| ids.iter().filter_map(|id| self.cards.get(id)).collect())
            .unwrap_or_default()
    }

    /// Get a card by mint ID.
    pub fn get_card(&self, mint_id: u64) -> Option<&IndexedCard> {
        self.cards.get(&mint_id)
    }

    /// Print current statistics.
    pub fn print_stats(&self) {
        println!("\n  {} Collection Statistics:", ">>".bright_cyan());
        println!("    Total minted:  {}", self.total_minted.to_string().bright_green());
        println!("    Total burned:  {}", self.total_burned.to_string().bright_red());
        println!("    Transfers:     {}", self.total_transfers);
        println!("    Interactions:  {}", self.total_interactions);
        println!("    Common:        {}", self.rarity_counts[0]);
        println!("    Rare:          {}", self.rarity_counts[1].to_string().bright_cyan());
        println!("    Legendary:     {}", self.rarity_counts[2].to_string().bright_magenta());
        println!("    Active cards:  {}", self.cards.values().filter(|c| !c.burned).count());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::*;

    #[test]
    fn test_mint_event() {
        let mut store = InMemoryStore::new();
        store.process_event(&CryptEvent::CardMinted(CardMintedEvent {
            mint_id: 0, owner: "owner123456789".into(), tx_hash: "tx123".into(),
            rarity: 2, card_type: 0, title: "Test".into(),
            soul_seed: [0; 32], timestamp: 1000,
        }));
        assert_eq!(store.total_minted, 1);
        assert_eq!(store.rarity_counts[2], 1);
    }

    #[test]
    fn test_transfer_event() {
        let mut store = InMemoryStore::new();
        store.process_event(&CryptEvent::CardMinted(CardMintedEvent {
            mint_id: 0, owner: "alice12345678".into(), tx_hash: "tx".into(),
            rarity: 0, card_type: 0, title: "".into(),
            soul_seed: [0; 32], timestamp: 0,
        }));
        store.process_event(&CryptEvent::CardTransferred(CardTransferredEvent {
            mint_id: 0, from: "alice12345678".into(), to: "bob1234567890".into(),
            tx_hash: "tx".into(), timestamp: 1,
        }));
        assert_eq!(store.get_card(0).unwrap().owner, "bob1234567890");
        assert_eq!(store.total_transfers, 1);
    }

    #[test]
    fn test_burn_event() {
        let mut store = InMemoryStore::new();
        store.process_event(&CryptEvent::CardMinted(CardMintedEvent {
            mint_id: 0, owner: "owner123456789".into(), tx_hash: "tx".into(),
            rarity: 1, card_type: 0, title: "".into(),
            soul_seed: [0; 32], timestamp: 0,
        }));
        store.process_event(&CryptEvent::CardBurned(CardBurnedEvent {
            mint_id: 0, owner: "owner123456789".into(), tx_hash: "tx".into(),
            rarity: 1, timestamp: 1,
        }));
        assert!(store.get_card(0).unwrap().burned);
        assert_eq!(store.total_burned, 1);
    }
}
