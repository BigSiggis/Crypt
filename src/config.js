// API Keys - set these in a .env file or replace directly
// NEVER commit real keys to git

export const CONFIG = {
  // Get free key at https://dev.helius.xyz
  HELIUS_API_KEY: import.meta.env.VITE_HELIUS_API_KEY || "",
  
  // Get key at https://console.anthropic.com
  ANTHROPIC_API_KEY: import.meta.env.VITE_ANTHROPIC_API_KEY || "",
  
  // Solana RPC (Helius recommended)
  SOLANA_RPC: import.meta.env.VITE_SOLANA_RPC || "https://api.mainnet-beta.solana.com",
};
