use clap::{Parser, Subcommand};
use colored::Colorize;

mod scanner;
mod scoring;
mod soul;
mod verify;
mod display;

#[derive(Parser)]
#[command(name = "crypt")]
#[command(about = "CRYPT — Resurrect your Solana wallet history as collectible trading cards")]
#[command(version = "0.1.0")]
#[command(author = "BigSiggis")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a Solana wallet and generate Crypt Cards
    Scan {
        /// Solana wallet address to scan
        #[arg(short, long)]
        address: String,

        /// Helius API key for enhanced transaction parsing
        #[arg(short = 'k', long, env = "HELIUS_API_KEY")]
        api_key: Option<String>,

        /// Maximum number of transactions to fetch
        #[arg(short, long, default_value = "100")]
        limit: usize,

        /// Output format: table, json, or cards
        #[arg(short, long, default_value = "cards")]
        format: String,

        /// Only show cards above this rarity: common, rare, legendary
        #[arg(short, long)]
        min_rarity: Option<String>,
    },

    /// Generate a Soul Signature seed from a transaction hash
    Soul {
        /// Transaction hash (signature)
        #[arg(short, long)]
        tx_hash: String,

        /// Show full 32-byte seed in hex
        #[arg(short, long)]
        verbose: bool,
    },

    /// Verify a Crypt Card's authenticity
    Verify {
        /// Card account address on Solana
        #[arg(short, long)]
        card: String,

        /// Solana RPC URL
        #[arg(short, long, default_value = "https://api.devnet.solana.com")]
        rpc: String,
    },

    /// Show rarity score breakdown for a transaction
    Score {
        /// Transaction type (SWAP, NFT_MINT, TRANSFER, etc.)
        #[arg(short, long)]
        tx_type: String,

        /// SOL amount involved
        #[arg(short, long, default_value = "0")]
        sol: f64,

        /// Is this a memecoin trade?
        #[arg(short, long)]
        memecoin: bool,

        /// Is this from a known DEX?
        #[arg(short, long)]
        defi: bool,
    },

    /// Display collection statistics
    Stats {
        /// Solana RPC URL
        #[arg(short, long, default_value = "https://api.devnet.solana.com")]
        rpc: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    println!("{}", "\n ╔═══════════════════════════════╗".bright_cyan());
    println!("{}", " ║     C R Y P T                 ║".bright_cyan());
    println!("{}", " ║     Solana Wallet Graveyard    ║".bright_cyan());
    println!("{}", " ╚═══════════════════════════════╝\n".bright_cyan());

    match cli.command {
        Commands::Scan { address, api_key, limit, format, min_rarity } => {
            scanner::scan_wallet(&address, api_key.as_deref(), limit, &format, min_rarity.as_deref()).await;
        }
        Commands::Soul { tx_hash, verbose } => {
            soul::generate_soul_seed(&tx_hash, verbose);
        }
        Commands::Verify { card, rpc } => {
            verify::verify_card(&card, &rpc).await;
        }
        Commands::Score { tx_type, sol, memecoin, defi } => {
            scoring::show_score(&tx_type, sol, memecoin, defi);
        }
        Commands::Stats { rpc } => {
            display::show_stats(&rpc).await;
        }
    }
}
