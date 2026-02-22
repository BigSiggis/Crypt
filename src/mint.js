// Solana on-chain minting for Crypt cards
// Devnet: creates real on-chain transaction with card metadata
// Production: would use the Anchor program at programs/crypt/

import { Connection, PublicKey, Transaction, SystemProgram, TransactionInstruction } from "@solana/web3.js";

const DEVNET_RPC = "https://api.devnet.solana.com";
const MEMO_PROGRAM_ID = new PublicKey("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");

// Build card metadata for on-chain storage
export function buildCardMetadata(card, soundtrack, artworkUrl) {
  return {
    name: `CRYPT #${card.id} — ${card.title}`,
    symbol: "CRYPT",
    description: card.narration || "A resurrected blockchain moment.",
    image: artworkUrl || "",
    external_url: `https://crypt-phi-two.vercel.app`,
    attributes: [
      { trait_type: "Type", value: card.type },
      { trait_type: "Rarity", value: card.rarity },
      { trait_type: "Platform", value: card.platform },
      { trait_type: "PnL", value: card.pnl || "N/A" },
      { trait_type: "Transaction", value: card.fullTx || card.tx },
    ],
    properties: {
      category: "image",
      files: artworkUrl ? [{ uri: artworkUrl, type: "image/png" }] : [],
      soundtrack: soundtrack ? {
        title: soundtrack.title,
        artist: soundtrack.artist,
        audius_id: soundtrack.id,
      } : null,
    },
  };
}

// Mint a card on Solana devnet
// Creates a memo transaction with the card data as on-chain proof
export async function mintCard(card, walletProvider, metadata) {
  if (!walletProvider) {
    throw new Error("Wallet not connected");
  }

  try {
    const connection = new Connection(DEVNET_RPC, "confirmed");
    const walletPubkey = walletProvider.publicKey;

    // Check balance
    const balance = await connection.getBalance(walletPubkey);
    if (balance < 10000) {
      // Try airdrop on devnet
      try {
        const sig = await connection.requestAirdrop(walletPubkey, 100000000); // 0.1 SOL
        await connection.confirmTransaction(sig);
      } catch (e) {
        console.log("Airdrop failed, proceeding anyway:", e.message);
      }
    }

    const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash();

    const transaction = new Transaction({
      recentBlockhash: blockhash,
      feePayer: walletPubkey,
    });

    // Memo instruction — stores card data on-chain
    const memoContent = JSON.stringify({
      protocol: "CRYPT",
      version: 1,
      action: "MINT_CARD",
      card_id: card.id,
      tx_hash: card.fullTx || card.tx,
      rarity: card.rarity,
      type: card.type,
      title: card.title,
      platform: card.platform,
      pnl: card.pnl,
      minted_by: walletPubkey.toBase58(),
      timestamp: Date.now(),
    });

    transaction.add(
      new TransactionInstruction({
        keys: [{ pubkey: walletPubkey, isSigner: true, isWritable: true }],
        programId: MEMO_PROGRAM_ID,
        data: new TextEncoder().encode(memoContent),
      })
    );

    // Sign and send — use signAndSendTransaction if available (better mobile support)
    if (walletProvider.signAndSendTransaction) {
      const sig = await walletProvider.signAndSendTransaction(transaction, {
        skipPreflight: false,
        preflightCommitment: "confirmed",
      });
      // Some wallets return { signature } object, others return string
      const sigStr = typeof sig === "string" ? sig : sig.signature;

      await connection.confirmTransaction({
        signature: sigStr,
        blockhash,
        lastValidBlockHeight,
      }, "confirmed");

      console.log("Minted on-chain:", sigStr);

      return {
        success: true,
        signature: sigStr,
        explorer: `https://explorer.solana.com/tx/${sigStr}?cluster=devnet`,
        solscan: `https://solscan.io/tx/${sigStr}?cluster=devnet`,
      };
    } else {
      const signedTx = await walletProvider.signTransaction(transaction);
      const sig = await connection.sendRawTransaction(signedTx.serialize(), {
        skipPreflight: false,
        preflightCommitment: "confirmed",
      });

      await connection.confirmTransaction({
        signature: sig,
        blockhash,
        lastValidBlockHeight,
      }, "confirmed");

      console.log("Minted on-chain:", sig);

      return {
        success: true,
        signature: sig,
        explorer: `https://explorer.solana.com/tx/${sig}?cluster=devnet`,
        solscan: `https://solscan.io/tx/${sig}?cluster=devnet`,
      };
    }
  } catch (e) {
    console.error("Mint failed:", e);
    return {
      success: false,
      error: e.message,
    };
  }
}

// Check if wallet is on devnet and has balance
export async function checkWalletReady(walletProvider) {
  if (!walletProvider?.publicKey) return { ready: false, reason: "No wallet" };

  try {
    const connection = new Connection(DEVNET_RPC, "confirmed");
    const balance = await connection.getBalance(walletProvider.publicKey);
    return {
      ready: balance > 5000,
      balance: balance / 1e9,
      address: walletProvider.publicKey.toBase58(),
      network: "devnet",
    };
  } catch (e) {
    return { ready: false, reason: e.message };
  }
}
