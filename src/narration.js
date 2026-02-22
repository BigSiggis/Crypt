// Claude AI narration generator
// Uses Anthropic API to generate card narrations from transaction data

const ANTHROPIC_API = "https://api.anthropic.com/v1/messages";

export async function generateNarration(card, apiKey) {
  if (!apiKey) return fallbackNarration(card);

  const prompt = buildPrompt(card);

  try {
    const res = await fetch(ANTHROPIC_API, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "x-api-key": apiKey,
        "anthropic-version": "2023-06-01",
        "anthropic-dangerous-direct-browser-access": "true",
      },
      body: JSON.stringify({
        model: "claude-sonnet-4-20250514",
        max_tokens: 200,
        messages: [{
          role: "user",
          content: prompt,
        }],
      }),
    });

    const data = await res.json();
    const text = data.content?.[0]?.text;
    if (text) return text.trim();
  } catch (e) {
    console.warn("Narration generation failed:", e);
  }

  return fallbackNarration(card);
}

// Batch generate narrations for multiple cards
export async function generateNarrations(cards, apiKey) {
  const results = {};
  // Process sequentially to avoid rate limits
  for (const card of cards) {
    if (card.narration) {
      results[card.id] = card.narration;
      continue;
    }
    results[card.id] = await generateNarration(card, apiKey);
    // Small delay between calls
    await new Promise(r => setTimeout(r, 500));
  }
  return results;
}

function buildPrompt(card) {
  const typeDescriptions = {
    swap: "a token swap on a Solana DEX",
    rug: "a rug pull where the token went to zero",
    mint: "an NFT mint that turned out to be valuable",
    diamond_hands: "holding through a massive price crash",
    big_move: "a large transfer of SOL (whale move)",
  };

  return `You are narrating blockchain transactions as dramatic micro-stories for a social trading card app called CRYPT. Write in a terse, punchy, first-person degen voice. Max 2 sentences. No hashtags, no emojis.

Transaction: ${card.title}
Type: ${typeDescriptions[card.type] || card.type}
Platform: ${card.platform}
Input: ${card.tIn.a} ${card.tIn.s}
Output: ${card.tOut.a} ${card.tOut.s}
PnL: ${card.pnl || "unknown"}
Date: ${card.date}

Write the narration:`;
}

export function fallbackNarration(card) {
  const narrations = {
    swap: [
      "Saw the chart. Didn't think twice. Sometimes the best trades happen at 3am.",
      "Ape first, ask questions never. The thesis was vibes only.",
      "Entry was clean. Exit was cleaner. The DEX remembers.",
    ],
    rug: [
      "Bought the top. Dev said 'gm.' Dev meant 'goodbye.'",
      "The chart looked bullish for exactly four minutes. Then the liquidity vanished.",
      "Lesson learned. Tuition paid. Moving on.",
    ],
    mint: [
      "Minted when nobody cared. History remembers the ones who showed up early.",
      "First 100. The mint price was nothing. The floor price became everything.",
      "Clicked mint. Waited. Refreshed. The rest is on-chain.",
    ],
    diamond_hands: [
      "Watched it bleed. Deleted the app. Came back. Still here.",
      "Everyone sold. The chart was red for months. But the thesis never changed.",
      "Diamond hands aren't about not feeling the pain. They're about holding anyway.",
    ],
    big_move: [
      "Off the exchange. Into cold storage. Not your keys, not your coins.",
      "The move that says everything without saying a word. Secured.",
      "When the number gets big enough, the only move is to lock it down.",
    ],
  };

  const options = narrations[card.type] || narrations.swap;
  return options[Math.floor(Math.random() * options.length)];
}
