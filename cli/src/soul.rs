use colored::Colorize;

/// Compute a deterministic 32-byte soul seed from a transaction hash.
/// Mirrors the on-chain logic in programs/crypt/src/utils/hashing.rs
pub fn compute_soul_seed_bytes(tx_hash: &str) -> [u8; 32] {
    let bytes = tx_hash.as_bytes();
    let mut seed = [0u8; 32];

    // Phase 1: XOR-fold
    for (i, &b) in bytes.iter().enumerate() {
        seed[i % 32] ^= b;
    }

    // Phase 2: Bit mixing
    for i in 0..32 {
        let mut h: u64 = seed[i] as u64;
        h = h.wrapping_mul(0x517cc1b727220a95);
        h ^= h >> 17;
        h = h.wrapping_mul(0x6c62272e07bb0142);
        h ^= h >> 11;
        seed[i] = (h & 0xFF) as u8;
    }

    // Phase 3: Diffusion
    for i in 1..32 {
        seed[i] ^= seed[i - 1].wrapping_add(37);
    }
    for i in (0..31).rev() {
        seed[i] ^= seed[i + 1].wrapping_add(53);
    }

    seed
}

/// Generate and display a Soul Signature seed for a transaction.
pub fn generate_soul_seed(tx_hash: &str, verbose: bool) {
    let seed = compute_soul_seed_bytes(tx_hash);
    let hex_str = hex::encode(&seed);

    println!("{}", "  SOUL SIGNATURE".bright_magenta());
    println!("  ────────────────────────────────────");
    println!("  TX:   {}", tx_hash.bright_yellow());
    println!("  Seed: {}", hex_str[..16].bright_cyan());

    if verbose {
        println!("  Full: {}", hex_str.bright_cyan());
        println!("\n  Byte breakdown:");
        for (i, chunk) in seed.chunks(8).enumerate() {
            let hex_chunk: Vec<String> = chunk.iter().map(|b| format!("{:02x}", b)).collect();
            println!("    [{:2}-{:2}] {}", i*8, i*8+7, hex_chunk.join(" ").bright_black());
        }
    }

    // Show art parameters derived from seed
    println!("\n  {} Art Parameters:", ">>".bright_green());
    println!("    Eye style:    {}", seed[0] % 8);
    println!("    Glow eyes:    {}", if seed[1] > 64 { "YES".bright_cyan() } else { "no".white() });
    println!("    Hat type:     {}", seed[2] % 30);
    println!("    Glasses:      {}", seed[3] % 12);
    println!("    Mouth item:   {}", seed[4] % 8);
    println!("    Neck item:    {}", seed[5] % 6);
    println!("    Teeth style:  {}", seed[6] % 6);
    println!("    Has scar:     {}", if seed[7] > 153 { "YES".bright_red() } else { "no".white() });
}
