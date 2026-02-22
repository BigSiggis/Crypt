// GRAVEYARD 32-BIT PALETTE
export const C = {
  bg: "#08080c", surface: "#0e0e14", card: "#0a0a10", inner: "#06060a",
  white: "#e0ddd4",        // parchment white
  gray: "#7a7770",
  dim: "#4a4840",
  faint: "#1e1e24",
  stroke: "#1c1c24",
  // Graveyard greens + purples
  green: "#00cc66",        // terminal green
  greenDim: "#009944",
  greenGlow: "rgba(0,204,102,0.15)",
  purple: "#8844cc",
  purpleDim: "#6633aa",
  purpleGlow: "rgba(136,68,204,0.15)",
  // Keep red for legendary / danger
  red: "#cc0000", redBright: "#ff1a1a",
  redFaint: "rgba(204,0,0,0.06)", redGlow: "rgba(204,0,0,0.12)",
  // Amber for rare
  amber: "#cc8800",
  amberGlow: "rgba(204,136,0,0.15)",
  // Bone/skull tones
  bone: "#d4c9a8",
  boneDim: "#9a9080",
};

export const RARITY = {
  legendary: { border:"#8844cc", glow:"rgba(136,68,204,0.4)", label:"#bb88ff", bg:"rgba(136,68,204,0.06)" },
  rare: { border:"#00cc66", glow:"rgba(0,204,102,0.25)", label:"#33ff88", bg:"rgba(0,204,102,0.04)" },
  common: { border:"#2a2a34", glow:"rgba(0,204,102,0.1)", label:"#00cc66", bg:"transparent" },
};

export const TT = {
  swap:          { label:"SWAP",          mark:"⚡", color:"#00cc66" },
  rug:           { label:"RUG PULL",      mark:"☠",  color:"#cc0000" },
  mint:          { label:"MINT",          mark:"†",  color:"#8844cc" },
  diamond_hands: { label:"DIAMOND HANDS", mark:"◆",  color:"#cc8800" },
  big_move:      { label:"WHALE MOVE",    mark:"▲",  color:"#00cc66" },
};

export const MOOD_QUERIES = {
  swap: ["edm", "electronic dance", "house music", "party"],
  rug: ["dark bass", "dubstep", "trap beat", "heavy electronic"],
  mint: ["hip hop beat", "rap instrumental", "boom bap", "beats"],
  diamond_hands: ["lo-fi", "chill beats", "ambient", "chillhop"],
  big_move: ["cinematic", "epic music", "orchestral", "soundtrack"],
};

export const tw = "'Press Start 2P','Courier New',monospace";
export const mono = "'VT323','Courier New',monospace";
export const body = "'VT323','Courier New',monospace";

export const fmt = (n) => n >= 1e6 ? (n/1e6).toFixed(1)+"M" : n >= 1e3 ? (n/1e3).toFixed(1)+"K" : ""+n;
