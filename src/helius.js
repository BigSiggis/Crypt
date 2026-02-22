const API_KEY = import.meta.env.VITE_HELIUS_API_KEY;
const BASE = `https://api.helius.xyz/v0`;

export async function getWalletHistory(address, limit = 100) {
  if (!API_KEY || API_KEY === "your_helius_api_key_here") {
    console.warn("No Helius API key — get one free at https://helius.dev");
    return [];
  }
  try {
    const res = await fetch(`${BASE}/addresses/${address}/transactions?api-key=${API_KEY}&limit=${limit}`);
    if (!res.ok) {
      const text = await res.text().catch(() => "");
      console.warn(`Helius ${res.status}: ${text}`);
      throw new Error(`Helius ${res.status}`);
    }
    return await res.json();
  } catch (e) {
    console.warn("Helius history failed:", e);
    return [];
  }
}

// Known tokens
const TOKENS = {
  "So11111111111111111111111111111111111111112": { s:"SOL", i:"◎" },
  "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v": { s:"USDC", i:"$" },
  "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB": { s:"USDT", i:"$" },
  "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263": { s:"BONK", i:"$" },
  "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN": { s:"JUP", i:"♃" },
  "EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm": { s:"WIF", i:"$" },
  "7GCihgDB8fe6KNjn2MYtkzZcRjQy3t9GHdC8uHYmW2hr": { s:"POPCAT", i:"$" },
  "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So": { s:"mSOL", i:"◎" },
  "7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj": { s:"stSOL", i:"◎" },
  "HZ1JovNiVvGrGNiiYvEozEVgZ58xaU3RKwX8eACQBCt3": { s:"PYTH", i:"$" },
  "hntyVP6YFm1Hg25TN9WGLqM12b8TQmcknKrdu1oxWux": { s:"HNT", i:"$" },
  "rndrizKT3MK1iimdxRdWabcF7Zg7AR5T4nud4EkHBof": { s:"RNDR", i:"$" },
  "DUSTawucrTsGU8hcqRdHDCbuYhCPADMLM2VcCb8VnFnQ": { s:"DUST", i:"$" },
  "TNSRxcUxoT9xBG3de7PiJyTDYu7kskLqcpddxnEJAS6": { s:"TNSR", i:"$" },
  "jtojtomepa8beP8AuQc6eXt5FriJwfFMwQx2v2f9mCL": { s:"JTO", i:"⚡" },
  "WENWENvqqNya429ubCdR81ZmD69brwQaaBYY6p3LCpk": { s:"WEN", i:"$" },
  "MEW1gQWJ3nEXg2qgERiKu7FAFj79PHvQVREQUzScPP5": { s:"MEW", i:"$" },
  "A3eME5CetyZPBoWbRUwY3tSe25S6tb18ba9ZPbWk9eFJ": { s:"PENG", i:"$" },
  "7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs": { s:"RAY", i:"☀" },
  "orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE": { s:"ORCA", i:"$" },
  "bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1": { s:"bSOL", i:"◎" },
  "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R": { s:"RAY", i:"☀" },
};

const MEMECOINS = new Set(["BONK","WIF","POPCAT","MEW","PENG","WEN","DUST"]);
const DEFI_SOURCES = new Set(["JUPITER","RAYDIUM","ORCA","MARINADE","DRIFT","MANGO","TENSOR","MAGIC_EDEN"]);

function tk(mint) {
  if (!mint) return { s:"???", i:"?" };
  return TOKENS[mint] || { s: mint.slice(0,4)+".."+mint.slice(-3), i:"$" };
}

function fmtAmt(n) {
  if (!n || n === 0) return "0";
  if (n >= 1e9) return (n/1e9).toFixed(1) + "B";
  if (n >= 1e6) return (n/1e6).toFixed(1) + "M";
  if (n >= 1e3) return (n/1e3).toFixed(1) + "K";
  if (n >= 1) return n.toFixed(2);
  return n.toFixed(4);
}

function timeAgo(ts) {
  const d = (Date.now()/1000) - ts;
  if (d < 3600) return Math.floor(d/60)+"m";
  if (d < 86400) return Math.floor(d/3600)+"h";
  if (d < 604800) return Math.floor(d/86400)+"d";
  if (d < 2592000) return Math.floor(d/604800)+"w";
  return Math.floor(d/2592000)+"mo";
}

function fmtDate(ts) {
  const d = new Date(ts * 1000);
  const m = ["I","II","III","IV","V","VI","VII","VIII","IX","X","XI","XII"];
  return `${m[d.getMonth()]}.${d.getDate()}.${d.getFullYear()}`;
}

function getSol(tx) {
  if (!tx.nativeTransfers?.length) return 0;
  let biggest = 0;
  for (const t of tx.nativeTransfers) {
    const a = Math.abs(t.amount) / 1e9;
    if (a > biggest) biggest = a;
  }
  return biggest;
}

function getNetSol(tx, wallet) {
  let net = 0;
  for (const t of (tx.nativeTransfers || [])) {
    if (t.toUserAccount === wallet) net += t.amount / 1e9;
    if (t.fromUserAccount === wallet) net -= t.amount / 1e9;
  }
  return net;
}

// ============ SCORING ============
function scoreTx(tx, wallet) {
  const type = tx.type || "";
  const src = tx.source || "";
  const sol = getSol(tx);
  const tokens = tx.tokenTransfers || [];
  const net = getNetSol(tx, wallet);
  let score = 0;
  let tags = [];

  // SWAPS
  if (type === "SWAP") {
    score += 25;
    if (sol > 100) { score += 80; tags.push("whale"); }
    else if (sol > 50) { score += 60; tags.push("whale"); }
    else if (sol > 10) { score += 35; tags.push("big"); }
    else if (sol > 2) { score += 15; tags.push("solid"); }
    else if (sol > 0.5) { score += 5; }
    else { score -= 5; }

    if (DEFI_SOURCES.has(src)) score += 5;
    
    for (const t of tokens) {
      const name = tk(t.mint).s;
      if (MEMECOINS.has(name)) { score += 25; tags.push("memecoin"); }
    }
  }
  // NFT MINTS
  else if (type === "NFT_MINT" || type === "COMPRESSED_NFT_MINT") {
    score += 35;
    if (sol > 10) { score += 40; tags.push("premium_mint"); }
    else if (sol > 2) { score += 20; tags.push("mint"); }
    else { tags.push("free_mint"); }
  }
  // NFT SALES
  else if (type === "NFT_SALE") {
    score += 30;
    if (sol > 50) { score += 70; tags.push("whale_sale"); }
    else if (sol > 10) { score += 40; tags.push("big_sale"); }
    else if (sol > 2) { score += 15; tags.push("sale"); }
    else { score -= 5; }
    
    // Check if we're the seller (profit!)
    if (net > 0) { score += 20; tags.push("profit"); }
  }
  // NFT LISTINGS
  else if (type === "NFT_LISTING") {
    if (sol > 20) { score += 30; tags.push("high_listing"); }
    else { score -= 10; }
  }
  // TRANSFERS
  else if (type === "TRANSFER" || type === "SOL_TRANSFER") {
    if (sol > 500) { score += 80; tags.push("massive"); }
    else if (sol > 100) { score += 55; tags.push("whale_move"); }
    else if (sol > 20) { score += 25; tags.push("big_move"); }
    else if (sol > 5) { score += 10; }
    else { score -= 15; }
  }
  // STAKING
  else if (type === "STAKE_SOL" || type === "UNSTAKE_SOL") {
    if (sol > 100) { score += 50; tags.push("whale_stake"); }
    else if (sol > 20) { score += 25; tags.push("stake"); }
    else { score += 5; }
  }
  // TOKEN CREATION
  else if (type === "TOKEN_MINT") {
    score += 50;
    tags.push("creator");
  }
  // BURN
  else if (type === "BURN" || type === "BURN_NFT") {
    score += 20;
    tags.push("burn");
  }

  // Deductions
  if (sol < 0.01 && !["NFT_MINT","COMPRESSED_NFT_MINT","TOKEN_MINT","BURN","BURN_NFT"].includes(type)) score -= 25;
  if (type === "UNKNOWN" || type === "") score -= 20;
  if (!type) score -= 30;

  return { score, tags, sol, net };
}

// ============ NARRATIONS ============
// These need to be DRAMATIC. Not generic. The user should feel something reading these.

function pickRandom(arr) { return arr[Math.floor(Math.random() * arr.length)]; }

const N = {
  swap_whale: [
    (s) => `${s.inAmt} ${s.inTk} into ${s.outTk}. In one click. No slippage prayer, no second guess. When you're this deep, hesitation is the only real risk.`,
    (s) => `Most people don't move ${s.inAmt} ${s.inTk} in a year. This wallet did it in one transaction on ${s.src}. The chain doesn't flinch. But the orderbook did.`,
    (s) => `A ${s.inAmt} ${s.inTk} swap that briefly moved the price. Somewhere a chart watcher spilled their coffee. Somewhere else, a bot recalibrated. The whale doesn't care about either.`,
  ],
  swap_memecoin: [
    (s) => `Aped ${s.inAmt} ${s.inTk} into ${s.outTk}. No whitepaper. No roadmap. Just a ticker, a vibe, and the unshakeable conviction that this time it's different. It's always different.`,
    (s) => `The degen alarm went off at 3am. ${s.outTk} was trending. ${s.inAmt} ${s.inTk} later, the position was open. Sleep is for people who don't check charts in the dark.`,
    (s) => `${s.outTk}. That's it. That's the thesis. ${s.inAmt} ${s.inTk} deployed on pure instinct. The memecoins don't need fundamentals. They need believers.`,
    (s) => `Swapped into ${s.outTk} like it was inevitable. ${s.inAmt} ${s.inTk} gone. No research. No due diligence. Just a Telegram screenshot and faith.`,
  ],
  swap_big: [
    (s) => `${s.inAmt} ${s.inTk} → ${s.outAmt} ${s.outTk}. Not a casual trade. This was a decision. The kind you make after staring at the same chart for three hours straight.`,
    (s) => `Rotated ${s.inAmt} ${s.inTk} into ${s.outTk} on ${s.src}. A calculated rebalance or a conviction bet? The chain records the what. Never the why.`,
    (s) => `${s.inAmt} ${s.inTk} converted to ${s.outAmt} ${s.outTk}. Big enough to mean something. Small enough to not be reckless. The sweet spot.`,
  ],
  swap_solid: [
    (s) => `${s.inAmt} ${s.inTk} → ${s.outAmt} ${s.outTk} via ${s.src}. A clean swap. No drama. Just someone who knows what they want and takes it.`,
    (s) => `Swapped ${s.inTk} for ${s.outTk}. Every portfolio is a story told in trades. This chapter was quiet but deliberate.`,
    (s) => `Another day, another swap. ${s.inAmt} ${s.inTk} became ${s.outAmt} ${s.outTk}. The grind doesn't sleep.`,
  ],
  swap_small: [
    (s) => `${s.inAmt} ${s.inTk} → ${s.outTk}. Small trade, maybe. But every empire starts with a single transaction. This one's on the permanent record.`,
    (s) => `A modest swap on ${s.src}. Not every move needs to be a whale play. Sometimes you're just building a position, one brick at a time.`,
  ],
  nft_mint_premium: [
    (s) => `Dropped ${fmtAmt(s.sol)} SOL on a mint. While others waited for the free claim, this wallet paid full price to be early. That's not spending — that's a statement of intent.`,
    (s) => `${fmtAmt(s.sol)} SOL minted. First 100 energy. The kind of bet that either ages like wine or becomes an expensive JPEG lesson. Either way, it's permanent.`,
  ],
  nft_mint: [
    (s) => `Minted. Added another piece to the permanent collection. Some people collect art. Some collect status. Some just can't resist a mint button at 2am.`,
    (s) => `Another NFT enters the wallet. Every mint is a tiny act of faith — believing that this image, this community, this moment, means something.`,
    (s) => `Hit the mint button. That split second between "confirm transaction" and the NFT appearing in your wallet? That's the purest form of hope in crypto.`,
  ],
  nft_sale_whale: [
    (s) => `${fmtAmt(s.sol)} SOL from a single NFT sale. That's not flipping — that's cashing a lottery ticket. Diamond hands finally let go, and the market paid up.`,
    (s) => `Sold for ${fmtAmt(s.sol)} SOL. The kind of exit that makes you rethink everything. Someone held through the doubt, the FUD, the "NFTs are dead" tweets. This was the payoff.`,
  ],
  nft_sale: [
    (s) => `NFT sold for ${fmtAmt(s.sol)} SOL. Someone's exit. Someone else's entry. The art stays the same. The stories around it keep changing.`,
    (s) => `Closed a position at ${fmtAmt(s.sol)} SOL. In a market full of diamond hand LARPers, actually taking profit is the rarest move of all.`,
  ],
  transfer_massive: [
    (s) => `${fmtAmt(s.sol)} SOL just moved. That's not a transaction — that's a migration. Cold storage? OTC deal? Estate planning? The chain keeps the secret forever.`,
    (s) => `${fmtAmt(s.sol)} SOL in a single transfer. Somewhere between reckless and legendary. The kind of move that makes block explorers feel like thriller novels.`,
  ],
  transfer_big: [
    (s) => `${fmtAmt(s.sol)} SOL on the move. Large enough to raise eyebrows. The destination tells one story. The timing tells another. Both are written in stone.`,
    (s) => `Moved ${fmtAmt(s.sol)} SOL. Not a swap. Not a trade. A deliberate relocation of capital. Every big wallet has a system. This was part of the plan.`,
  ],
  transfer_in: [
    (s) => `${fmtAmt(s.sol)} SOL landed. Funds arrived like a message in a bottle — you know it came from somewhere, but the chain only tells you from whom, not why.`,
    (s) => `Incoming: ${fmtAmt(s.sol)} SOL. Payday? Profit withdrawal? A friend paying back a loan from 2023? The best stories on-chain are the ones you'll never fully know.`,
  ],
  transfer_out: [
    (s) => `${fmtAmt(s.sol)} SOL sent out into the void. Every outbound transfer is a little death — SOL leaving your wallet, headed somewhere you can only watch.`,
    (s) => `Sent ${fmtAmt(s.sol)} SOL. The wallet got lighter. But lighter isn't always worse. Sometimes you're paying for something that doesn't show up on-chain yet.`,
  ],
  stake: [
    (s) => `${fmtAmt(s.sol)} SOL staked. Locked up and earning. While traders chase the next 10x, the stakers play the infinite game. Patience as a position.`,
    (s) => `Staked ${fmtAmt(s.sol)} SOL. The most boring trade in crypto is also the most disciplined. Validators eat well tonight.`,
  ],
  whale_stake: [
    (s) => `${fmtAmt(s.sol)} SOL locked in stake. That's not just yield farming — that's a vote of confidence in Solana's future. The biggest bets are the quietest ones.`,
  ],
  unstake: [
    (s) => `${fmtAmt(s.sol)} SOL unstaked and freed. The lockup period ended. Now the real question: redeploy, rotate, or ride it? The chain is watching.`,
  ],
  token_create: [
    (s) => `Launched a token. From zero to contract address. Most won't survive the week. But every blue chip started exactly like this — one deploy, zero holders, infinite possibility.`,
    (s) => `Token created. A new asset born on Solana. The ticker is set. The supply is minted. Everything else — the community, the narrative, the chart — that's still unwritten.`,
  ],
  burn: [
    (s) => `Burned. Sent to the void address, never to return. Some things need to be destroyed to make room for what comes next. Digital cremation.`,
    (s) => `Burned on-chain. Permanent. Irreversible. The blockchain equivalent of lighting a match. Whatever this was, it's ash now.`,
  ],
  default: [
    (s) => `A transaction recorded on Solana. The chain doesn't judge. It doesn't editorialize. It just remembers. And now, so do you.`,
    (s) => `On-chain activity. Not every move needs a headline. Some moments are just proof that this wallet was alive, active, doing something. That's enough.`,
  ],
};

function buildNarration(type, tags, ctx) {
  if (type === "SWAP") {
    if (tags.includes("whale")) return pickRandom(N.swap_whale)(ctx);
    if (tags.includes("memecoin")) return pickRandom(N.swap_memecoin)(ctx);
    if (tags.includes("big")) return pickRandom(N.swap_big)(ctx);
    if (tags.includes("solid")) return pickRandom(N.swap_solid)(ctx);
    return pickRandom(N.swap_small)(ctx);
  }
  if (type === "NFT_MINT" || type === "COMPRESSED_NFT_MINT") {
    if (tags.includes("premium_mint")) return pickRandom(N.nft_mint_premium)(ctx);
    return pickRandom(N.nft_mint)(ctx);
  }
  if (type === "NFT_SALE") {
    if (tags.includes("whale_sale") || tags.includes("big_sale")) return pickRandom(N.nft_sale_whale)(ctx);
    return pickRandom(N.nft_sale)(ctx);
  }
  if (type === "TRANSFER" || type === "SOL_TRANSFER") {
    if (tags.includes("massive")) return pickRandom(N.transfer_massive)(ctx);
    if (tags.includes("whale_move") || tags.includes("big_move")) return pickRandom(N.transfer_big)(ctx);
    if (ctx.net > 0) return pickRandom(N.transfer_in)(ctx);
    return pickRandom(N.transfer_out)(ctx);
  }
  if (type === "STAKE_SOL") {
    if (tags.includes("whale_stake")) return pickRandom(N.whale_stake)(ctx);
    return pickRandom(N.stake)(ctx);
  }
  if (type === "UNSTAKE_SOL") return pickRandom(N.unstake)(ctx);
  if (type === "TOKEN_MINT") return pickRandom(N.token_create)(ctx);
  if (type === "BURN" || type === "BURN_NFT") return pickRandom(N.burn)(ctx);
  return pickRandom(N.default)(ctx);
}

// ============ CARD BUILDER ============
function buildCard(tx, wallet, rank) {
  const { tags, sol, net } = scoreTx(tx, wallet);
  const type = tx.type || "";
  const src = tx.source || "";
  const sig = tx.signature || "";
  const tokens = tx.tokenTransfers || [];
  const shortSig = sig.slice(0,5) + "..." + sig.slice(-4);

  let cardType = "swap";
  let title = "";
  let tIn = { s:"SOL", a:fmtAmt(sol), i:"◎" };
  let tOut = { s:"???", a:"?", i:"?" };
  let pnl = "";
  let usd = sol > 0 ? `~${fmtAmt(sol)} SOL` : "";

  // Parse by type
  if (type === "SWAP") {
    cardType = "swap";
    const sent = tokens.find(t => t.fromUserAccount === wallet);
    const received = tokens.find(t => t.toUserAccount === wallet);
    if (sent) { const t = tk(sent.mint); tIn = { s:t.s, a:fmtAmt(sent.tokenAmount), i:t.i }; }
    if (received) { const t = tk(received.mint); tOut = { s:t.s, a:fmtAmt(received.tokenAmount), i:t.i }; }
    if (!sent && sol > 0) tIn = { s:"SOL", a:fmtAmt(sol), i:"◎" };
    if (!received) {
      const solIn = tx.nativeTransfers?.find(t => t.toUserAccount === wallet);
      if (solIn) tOut = { s:"SOL", a:fmtAmt(Math.abs(solIn.amount)/1e9), i:"◎" };
    }
    title = `${tIn.a} ${tIn.s} → ${tOut.s}`;
    pnl = "SWAPPED";
  }
  else if (type === "NFT_MINT" || type === "COMPRESSED_NFT_MINT") {
    cardType = "mint";
    tIn = { s:"SOL", a:fmtAmt(sol), i:"◎" };
    tOut = { s:"NFT", a:"MINTED", i:"†" };
    title = sol > 0 ? `MINTED NFT FOR ${fmtAmt(sol)} SOL` : "FREE MINT";
    pnl = "MINTED";
  }
  else if (type === "NFT_SALE") {
    cardType = net > 0 ? "swap" : "big_move";
    tIn = { s:"NFT", a:"SOLD", i:"†" };
    tOut = { s:"SOL", a:fmtAmt(sol), i:"◎" };
    title = `NFT SOLD FOR ${fmtAmt(sol)} SOL`;
    pnl = net > 0 ? `+${fmtAmt(sol)}` : `${fmtAmt(sol)}`;
  }
  else if (type === "TRANSFER" || type === "SOL_TRANSFER") {
    cardType = "big_move";
    const isSend = net < 0;
    title = isSend ? `SENT ${fmtAmt(sol)} SOL` : `RECEIVED ${fmtAmt(sol)} SOL`;
    tOut = { s: isSend ? "SENT" : "IN", a:fmtAmt(sol), i: isSend ? "↗" : "↙" };
    pnl = isSend ? "SENT" : "RECEIVED";
  }
  else if (type === "STAKE_SOL" || type === "UNSTAKE_SOL") {
    cardType = "diamond_hands";
    title = type === "STAKE_SOL" ? `STAKED ${fmtAmt(sol)} SOL` : `UNSTAKED ${fmtAmt(sol)} SOL`;
    tOut = { s: type === "STAKE_SOL" ? "STAKED" : "UNSTAKED", a:fmtAmt(sol), i:"◎" };
    pnl = type === "STAKE_SOL" ? "LOCKED" : "FREED";
  }
  else if (type === "TOKEN_MINT") {
    cardType = "mint";
    tOut = { s:"TOKEN", a:"NEW", i:"⚡" };
    title = "LAUNCHED A TOKEN";
    pnl = "CREATED";
  }
  else if (type === "BURN" || type === "BURN_NFT") {
    cardType = "rug";
    tOut = { s:"BURNED", a:"X", i:"X" };
    title = "BURNED";
    pnl = "ASH";
  }
  else {
    title = `${type || "TX"} via ${src || "SOLANA"}`;
    tOut = { s: src || "TX", a:fmtAmt(sol), i:"⚡" };
    pnl = sol > 0 ? fmtAmt(sol) + " SOL" : "TX";
  }

  // Rarity
  let rarity = "common";
  if (sol > 100 || tags.includes("whale") || tags.includes("massive") || tags.includes("creator")) rarity = "legendary";
  else if (sol > 10 || tags.includes("big") || tags.includes("premium_mint") || tags.includes("whale_sale") || tags.includes("memecoin")) rarity = "rare";

  const narration = buildNarration(type, tags, {
    sol, net, src: src || "Solana",
    inAmt: tIn.a, inTk: tIn.s,
    outAmt: tOut.a, outTk: tOut.s,
  });

  return {
    id: rank + 1,
    type: cardType,
    rarity,
    title,
    narration,
    user: { name: wallet.slice(0,8) + ".sol", addr: wallet.slice(0,4) + "..." + wallet.slice(-4) },
    platform: src || type || "Solana",
    date: fmtDate(tx.timestamp),
    tIn, tOut, pnl,
    up: net >= 0 && pnl !== "SENT",
    usd,
    tx: shortSig,
    fullTx: sig,
    nft: ["NFT_MINT","NFT_SALE","COMPRESSED_NFT_MINT","BURN_NFT"].includes(type),
    likes: Math.floor(Math.random() * 5000) + 100,
    comments: Math.floor(Math.random() * 500) + 10,
    liked: false,
    ago: timeAgo(tx.timestamp),
    timestamp: tx.timestamp,
    tags,
  };
}

// ============ MAIN SCANNER ============
export async function scanWallet(address) {
  const txs = await getWalletHistory(address, 100);
  if (!txs || txs.length === 0) return [];

  // Score all
  const scored = txs.map(tx => ({ tx, ...scoreTx(tx, address) }));
  scored.sort((a, b) => b.score - a.score);

  // Diversity: max 2 of same type, ensure variety
  const typeCounts = {};
  const maxPerType = 2;
  const selected = [];

  for (const s of scored) {
    if (s.score <= 0) continue;
    const t = s.tx.type || "UNKNOWN";
    typeCounts[t] = (typeCounts[t] || 0) + 1;
    if (typeCounts[t] > maxPerType) continue;
    selected.push(s.tx);
    if (selected.length >= 8) break;
  }

  // If we have less than 5, relax the type limit
  if (selected.length < 5) {
    for (const s of scored) {
      if (s.score <= 0) continue;
      if (selected.includes(s.tx)) continue;
      selected.push(s.tx);
      if (selected.length >= 6) break;
    }
  }

  return selected.map((tx, i) => buildCard(tx, address, i));
}
