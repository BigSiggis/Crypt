const TAPESTRY_API = "https://api.usetapestry.dev/v1";
const API_KEY = import.meta.env.VITE_TAPESTRY_API_KEY;

// Resolve wallet address to social identity
export async function resolveIdentity(walletAddress) {
  if (!API_KEY) return null;
  try {
    const res = await fetch(
      `${TAPESTRY_API}/profiles/search?apiKey=${API_KEY}`,
      {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          walletAddress,
          shouldIncludeExternalProfiles: true,
          limit: 5,
        }),
      }
    );
    if (!res.ok) return null;
    const data = await res.json();
    const profiles = data.profiles || data || [];
    if (profiles.length === 0) return null;

    // Pick the best profile (prefer one with username + image)
    const best = profiles.find(p => p.username && p.customProperties?.profileImage) 
      || profiles.find(p => p.username)
      || profiles[0];

    return {
      username: best.username || null,
      bio: best.customProperties?.bio || null,
      image: best.customProperties?.profileImage || null,
      namespace: best.namespace || null,
      profileId: best.id || null,
    };
  } catch (e) {
    console.warn("Tapestry resolve failed:", e);
    return null;
  }
}

// Create or find profile for connected wallet
export async function findOrCreateProfile(walletAddress, username) {
  if (!API_KEY) return null;
  try {
    const res = await fetch(
      `${TAPESTRY_API}/profiles/findOrCreate?apiKey=${API_KEY}`,
      {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          walletAddress,
          username: username || walletAddress.slice(0, 8),
          blockchain: "SOLANA",
          execution: "FAST_UNCONFIRMED",
        }),
      }
    );
    if (!res.ok) return null;
    return await res.json();
  } catch (e) {
    console.warn("Tapestry profile create failed:", e);
    return null;
  }
}

// Like a card (content node on Tapestry)
export async function likeContent(profileId, contentId) {
  if (!API_KEY || !profileId) return null;
  try {
    const res = await fetch(
      `${TAPESTRY_API}/likes/create?apiKey=${API_KEY}`,
      {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          profileId,
          contentId,
          blockchain: "SOLANA",
          execution: "FAST_UNCONFIRMED",
        }),
      }
    );
    if (!res.ok) return null;
    return await res.json();
  } catch (e) {
    console.warn("Tapestry like failed:", e);
    return null;
  }
}

// Post a card as content to Tapestry
export async function postCard(profileId, card) {
  if (!API_KEY || !profileId) return null;
  try {
    const res = await fetch(
      `${TAPESTRY_API}/contents/create?apiKey=${API_KEY}`,
      {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          profileId,
          content: card.narration || card.title,
          contentType: "text",
          customProperties: [
            { key: "txHash", value: card.fullTx || card.tx },
            { key: "title", value: card.title },
            { key: "rarity", value: card.rarity },
            { key: "type", value: card.type },
            { key: "platform", value: card.platform },
            { key: "app", value: "crypt" },
          ],
          blockchain: "SOLANA",
          execution: "FAST_UNCONFIRMED",
        }),
      }
    );
    if (!res.ok) return null;
    return await res.json();
  } catch (e) {
    console.warn("Tapestry post failed:", e);
    return null;
  }
}
