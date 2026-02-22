//! On-chain analytics module for Crypt Cards.
//! Provides wallet profiling, transaction pattern detection,
//! and historical analysis for card generation.

pub mod wallet_profile;
pub mod patterns;
pub mod metrics;

pub use wallet_profile::*;
pub use patterns::*;
pub use metrics::*;
