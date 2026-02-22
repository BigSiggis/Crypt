import React, { useState, useEffect, useRef } from "react";
import { C, RARITY, TT, MOOD_QUERIES, tw, mono, fmt } from "./theme";
import { getTrending, formatDuration } from "./audius";
import { useAudioPlayer } from "./useAudioPlayer";
import { shortenAddress, detectWallets, connectWallet } from "./wallet";
import { scanWallet } from "./helius";
import { resolveIdentity, findOrCreateProfile } from "./tapestry";
import { ZineBg, Logo, Noise, Dash, PixelDivider, GlitchText, AudioPlayerUI } from "./components";
import SoulSignature from "./SoulSignature";
import Graveyard from "./Graveyard";
import MintCeremony from "./MintCeremony";

// MintButton with full ceremony
function MintButton({ onMint }) {
  const [minted, setMinted] = useState(false);
  if (minted) return (
    <div style={{ marginLeft:"auto", padding:"5px 12px", border:`1px solid ${C.green}`, background:"rgba(0,204,102,0.1)" }}>
      <span style={{ color:C.green, fontSize:11, fontFamily:mono, letterSpacing:"0.15em", textShadow:`0 0 8px ${C.greenGlow}` }}>MINTED</span>
    </div>
  );
  return (
    <button onClick={() => { onMint(() => setMinted(true)); }} style={{ marginLeft:"auto", background:"none", border:`1px solid ${C.green}40`, cursor:"pointer", color:C.green, fontSize:11, fontFamily:mono, fontWeight:400, padding:"5px 12px", letterSpacing:"0.15em", textShadow:`0 0 6px ${C.greenGlow}` }}>MINT</button>
  );
}

// Unique skull avatars per user — pixel art style
const SKULL_COLORS = {
  "degen_whale.sol": { fill:"#00d4b0", eye:"#66ffcc", bg:"#0a1a15", crown:"#c8a030" },
  "sol_og_2021": { fill:"#9660ff", eye:"#bb88ff", bg:"#0d0520", crown:null },
  "0xAlpha": { fill:"#c89030", eye:"#ffc850", bg:"#1a1200", crown:null },
  "paperhands.sol": { fill:"#cc3030", eye:"#ff6060", bg:"#1a0505", crown:null },
};

function SkullAvatar({ name, size = 30 }) {
  const c = SKULL_COLORS[name] || { fill:"#889898", eye:"#aacccc", bg:"#0e1214", crown:null };
  // Higher detail pixel skull — clean, no scan lines
  return (
    <svg width={size} height={size} viewBox="0 0 16 16" style={{ borderRadius:"50%", flexShrink:0, imageRendering:"pixelated" }}>
      <rect width="16" height="16" rx="8" fill={c.bg} />
      {/* Skull shape - pixel grid */}
      <rect x="5" y="3" width="6" height="1" fill={c.fill} />
      <rect x="4" y="4" width="8" height="1" fill={c.fill} />
      <rect x="3" y="5" width="10" height="1" fill={c.fill} />
      <rect x="3" y="6" width="10" height="1" fill={c.fill} />
      <rect x="3" y="7" width="10" height="1" fill={c.fill} />
      <rect x="3" y="8" width="10" height="1" fill={c.fill} />
      <rect x="4" y="9" width="8" height="1" fill={c.fill} />
      <rect x="5" y="10" width="6" height="1" fill={c.fill} />
      <rect x="6" y="11" width="4" height="1" fill={c.fill} />
      <rect x="6" y="12" width="4" height="1" fill={c.fill} />
      {/* Eyes */}
      <rect x="4" y="6" width="2" height="2" fill={c.bg} />
      <rect x="10" y="6" width="2" height="2" fill={c.bg} />
      <rect x="4" y="6" width="1" height="1" fill={c.eye} opacity="0.9" />
      <rect x="10" y="6" width="1" height="1" fill={c.eye} opacity="0.9" />
      {/* Nose */}
      <rect x="7" y="9" width="2" height="1" fill={c.bg} />
      {/* Jaw detail */}
      <rect x="7" y="11" width="1" height="1" fill={c.bg} opacity="0.4" />
      <rect x="8" y="12" width="1" height="1" fill={c.bg} opacity="0.4" />
      {/* Crown for legendary */}
      {c.crown && <>
        <rect x="5" y="2" width="1" height="1" fill={c.crown} />
        <rect x="7" y="1" width="2" height="1" fill={c.crown} />
        <rect x="10" y="2" width="1" height="1" fill={c.crown} />
        <rect x="5" y="2" width="6" height="1" fill={c.crown} opacity="0.7" />
      </>}
    </svg>
  );
}


const DEMO_CARDS = [
  { id:1, type:"swap", rarity:"legendary", title:"420 SOL \u2192 BONK",
    narration:"420 SOL into BONK at 3am. Woke up a different person. The degen gods were smiling that night.",
    user:{name:"degen_whale.sol",addr:"3vR7...xN9w"}, platform:"Jupiter", date:"XII.14.2024",
    tIn:{s:"SOL",a:"420",i:"\u25CE"}, tOut:{s:"BONK",a:"41.8B",i:"B"},
    pnl:"+4,200%", up:true, usd:"$58,240", tx:"4xK7m...9pR2", nft:false,
    likes:1842, comments:247, liked:false, ago:"2h" },
  { id:2, type:"rug", rarity:"rare", title:"RUGGED \u2014 $SQUIDGAME",
    narration:"Bought the top. Dev said 'gm.' Dev meant 'goodbye.' Four minutes and -99.8% later, a lesson was learned.",
    user:{name:"sol_og_2021",addr:"9pL2...hK4r"}, platform:"Raydium", date:"III.22.2024",
    tIn:{s:"SOL",a:"12",i:"\u25CE"}, tOut:{s:"SQUID",a:"890K",i:"\u2620"},
    pnl:"-99.8%", up:false, usd:"$2.40 LEFT", tx:"7rN3q...2wL5", nft:false,
    likes:3201, comments:589, liked:true, ago:"5h" },
  { id:3, type:"mint", rarity:"legendary", title:"MINTED DeGods #4271",
    narration:"Minted when nobody cared. First 100. Floor hit 500 SOL. History remembers the ones who showed up early.",
    user:{name:"0xAlpha",addr:"7wE5...kR1n"}, platform:"Magic Eden", date:"X.8.2022",
    tIn:{s:"SOL",a:"3",i:"\u25CE"}, tOut:{s:"DeGods",a:"#4271",i:"\u2020"},
    pnl:"+16,566%", up:true, usd:"PEAK: 540 SOL", tx:"2bH8k...5mX9", nft:true,
    likes:7421, comments:1203, liked:false, ago:"8h" },
  { id:4, type:"diamond_hands", rarity:"rare", title:"HELD THROUGH THE CRASH",
    narration:"Bought at $34. Watched it bleed to $8. Deleted the app for three months. Came back. Still here.",
    user:{name:"paperhands.sol",addr:"2mQ8...vB3j"}, platform:"Phantom", date:"XI.9.2022",
    tIn:{s:"SOL",a:"500",i:"\u25CE"}, tOut:{s:"SOL",a:"500",i:"\u25CE"},
    pnl:"HELD", up:true, usd:"$17K \u2192 $4K \u2192 $52K", tx:"9vF1r...3kP7", nft:false,
    likes:5102, comments:892, liked:true, ago:"12h" },
  { id:5, type:"big_move", rarity:"legendary", title:"10,000 SOL \u2192 COLD STORAGE",
    narration:"10,000 SOL off the exchange. Not your keys, not your coins. Learned from three letters: F-T-X.",
    user:{name:"degen_whale.sol",addr:"3vR7...xN9w"}, platform:"Transfer", date:"I.15.2024",
    tIn:{s:"SOL",a:"10,000",i:"\u25CE"}, tOut:{s:"VAULT",a:"LOCKED",i:"\u25BC"},
    pnl:"SECURED", up:true, usd:"$1,040,000", tx:"5tG2m...8nQ4", nft:false,
    likes:9847, comments:1456, liked:false, ago:"1d" },
];

function TxVis({ card }) {
  const c = card;
  if (c.type === "rug") return (
    <div style={{ padding:"20px 0", textAlign:"center" }}>
      <div style={{ display:"flex", alignItems:"center", justifyContent:"center", gap:24 }}>
        <div><div style={{ fontSize:16, color:C.white, fontFamily:mono, fontWeight:700 }}>{c.tIn.i} {c.tIn.a}</div><div style={{ color:C.dim, fontSize:8, fontFamily:mono }}>{c.tIn.s}</div></div>
        <div><div style={{ color:C.red, fontSize:18, fontFamily:mono }}>{"\u2192"}</div><div style={{ color:C.red, fontFamily:mono, fontSize:7, fontWeight:700 }}>DEAD</div></div>
        <div style={{ opacity:0.2 }}><div style={{ fontSize:16, color:C.gray, fontFamily:mono, fontWeight:700, textDecoration:"line-through" }}>{c.tOut.i} {c.tOut.a}</div><div style={{ color:C.dim, fontSize:8, fontFamily:mono }}>{c.tOut.s}</div></div>
      </div>
    </div>
  );
  if (c.type === "diamond_hands") return (
    <div style={{ padding:"20px 0", textAlign:"center" }}>
      <div style={{ fontSize:22, color:C.white, fontFamily:mono, fontWeight:700 }}>{"\u25CE"} {c.tIn.a} SOL</div>
      <div style={{ display:"flex", alignItems:"center", justifyContent:"center", gap:8, marginTop:8, fontSize:13, fontFamily:mono }}>
        <span style={{ color:C.red }}>$17K</span><span style={{ color:C.dim }}>{"\u2192"}</span>
        <span style={{ color:C.redBright, fontWeight:700 }}>$4K</span><span style={{ color:C.dim }}>{"\u2192"}</span>
        <span style={{ color:C.white, fontWeight:700 }}>$52K</span>
      </div>
      <div style={{ color:C.gray, fontWeight:700, marginTop:8, letterSpacing:"0.35em", fontSize:7, fontFamily:mono }}>NEVER. SOLD.</div>
    </div>
  );
  if (c.type === "big_move") return (
    <div style={{ padding:"20px 0", textAlign:"center" }}>
      <div style={{ fontSize:26, color:C.white, fontFamily:mono, fontWeight:700 }}>{"\u25CE"} {c.tIn.a}</div>
      <div style={{ color:C.dim, fontSize:10, fontFamily:mono }}>SOL</div>
      <div style={{ color:C.white, fontSize:11, fontWeight:700, marginTop:4, fontFamily:mono }}>{c.usd}</div>
      <div style={{ color:C.dim, fontSize:8, marginTop:4, fontFamily:mono }}>EXCHANGE {"\u2192"} COLD STORAGE</div>
    </div>
  );
  return (
    <div style={{ display:"flex", alignItems:"center", justifyContent:"center", padding:"20px 0", gap:24 }}>
      <div style={{ textAlign:"center" }}><div style={{ fontSize:16, color:C.white, fontFamily:mono, fontWeight:700 }}>{c.tIn.i} {c.tIn.a}</div><div style={{ color:C.dim, fontSize:8, fontFamily:mono }}>{c.tIn.s}</div></div>
      <div style={{ color:C.dim, fontSize:16, fontFamily:mono }}>{"\u2192"}</div>
      <div style={{ textAlign:"center" }}><div style={{ fontSize:16, color:C.white, fontFamily:mono, fontWeight:700 }}>{c.tOut.i} {c.tOut.a}</div><div style={{ color:C.dim, fontSize:8, fontFamily:mono }}>{c.tOut.s}</div></div>
    </div>
  );
}

function CryptCard({ card, onLike, onMint, soundtrack, audio, delay = 0 }) {
  const th = TT[card.type];
  const isLeg = card.rarity === "legendary";
  const isRug = card.type === "rug";
  const rar = RARITY[card.rarity] || RARITY.common;
  const [showCom, setShowCom] = useState(false);
  const [fol, setFol] = useState(card.id % 2 === 0);
  const isCurrent = audio.currentTrack?.id === soundtrack?.id;

  // Trading card border colors
  const borderCol = isLeg ? "rgba(150,90,255,0.5)" : card.rarity === "rare" ? "rgba(0,212,176,0.35)" : "rgba(0,212,176,0.15)";
  const glowCol = isLeg ? "0 0 25px rgba(150,90,255,0.3), 0 0 50px rgba(150,90,255,0.1)" : card.rarity === "rare" ? "0 0 20px rgba(0,212,176,0.15)" : "none";

  return (
    <div style={{ marginBottom:28, animation:`cardEntrance 0.5s ease-out ${delay}ms both` }}>
      {/* TRADING CARD WRAPPER - Pokemon/collectible proportions */}
      <div style={{
        maxWidth: 360, margin: "0 auto",
        background: "rgba(8,5,18,0.85)",
        border: `2px solid ${borderCol}`,
        boxShadow: glowCol,
        overflow: "hidden",
        backdropFilter: "blur(6px)",
      }}>
        {/* CARD HEADER - user + follow */}
        <div style={{ padding:"12px 14px", display:"flex", alignItems:"center", justifyContent:"space-between", borderBottom:`1px solid rgba(0,212,176,0.08)` }}>
          <div style={{ display:"flex", alignItems:"center", gap:8 }}>
            <SkullAvatar name={card.user.name} size={28} />
            <div>
              <div style={{ color:"#00d4b0", fontWeight:400, fontSize:13, fontFamily:mono }}>{card.user.name}</div>
              <div style={{ color:"rgba(255,255,255,0.55)", fontFamily:mono, fontSize:9 }}>{card.user.addr}</div>
            </div>
          </div>
          <button onClick={() => setFol(!fol)} style={{
            fontSize:9, fontWeight:400, fontFamily:mono, letterSpacing:"0.15em", padding:"4px 10px", cursor:"pointer",
            border:`1px solid ${fol ? "rgba(0,212,176,0.2)" : "rgba(0,212,176,0.5)"}`,
            background: "transparent", color: fol ? "rgba(255,255,255,0.55)" : "#00d4b0",
          }}>
            {fol ? "FOLLOWING" : "FOLLOW"}
          </button>
        </div>

        {/* RARITY + TYPE BAR */}
        <div style={{ padding:"8px 14px", display:"flex", alignItems:"center", gap:6, borderBottom:`1px solid rgba(0,212,176,0.05)` }}>
          <span style={{ padding:"2px 8px", fontSize:9, letterSpacing:"0.2em", fontFamily:mono,
            border:`1px solid ${rar.border}`, color:rar.label, background:"transparent" }}>{card.rarity.toUpperCase()}</span>
          <span style={{ padding:"2px 8px", fontSize:9, letterSpacing:"0.12em", fontFamily:mono,
            border:`1px solid ${th.color || "rgba(255,255,255,0.1)"}30`, color: th.color || "rgba(255,255,255,0.4)" }}>{th.mark} {th.label}</span>
          <span style={{ marginLeft:"auto", fontSize:9, color:"rgba(255,255,255,0.5)", fontFamily:mono }}>{card.ago}</span>
        </div>

        {/* SOUL SIGNATURE ART - tall trading card proportions */}
        <div style={{ position:"relative", width:"100%", aspectRatio:"3/4", overflow:"hidden", borderBottom:`1px solid rgba(0,212,176,0.08)` }}>
          <SoulSignature txHash={card.tx} rarity={card.rarity} type={card.type} width={420} height={560} isPlaying={isCurrent && audio.playing} />
          {/* Title overlay at top */}
          <div style={{ position:"absolute", top:0, left:0, right:0, padding:"10px 12px", zIndex:2 }}>
            <span style={{ color:"#fff", fontSize:14, fontFamily:mono, textShadow:"0 0 12px rgba(0,0,0,0.9), 0 2px 4px rgba(0,0,0,0.8)", letterSpacing:"0.04em" }}>{card.title}</span>
          </div>
        </div>

        {/* SWAP DATA - clean, big numbers like concept */}
        <div style={{ padding:"14px 16px", borderBottom:`1px solid rgba(0,212,176,0.08)` }}>
          <div style={{ display:"flex", alignItems:"center", justifyContent:"center", gap:16 }}>
            <div style={{ textAlign:"center" }}>
              <div style={{ fontSize:22, color:"#fff", fontFamily:mono, fontWeight:400 }}>{card.tIn.i} {card.tIn.a}</div>
              <div style={{ color:"rgba(255,255,255,0.55)", fontSize:8, fontFamily:mono, marginTop:2 }}>{card.tIn.s}</div>
            </div>
            <div style={{ color:"rgba(255,255,255,0.5)", fontSize:18, fontFamily:mono }}>{isRug ? "X" : "→"}</div>
            <div style={{ textAlign:"center", opacity: isRug ? 0.3 : 1 }}>
              <div style={{ fontSize:22, color:"#fff", fontFamily:mono, fontWeight:400, textDecoration: isRug ? "line-through" : "none" }}>{card.tOut.i} {card.tOut.a}</div>
              <div style={{ color:"rgba(255,255,255,0.55)", fontSize:8, fontFamily:mono, marginTop:2 }}>{card.tOut.s}</div>
            </div>
          </div>
          {/* PnL + TX hash */}
          <div style={{ display:"flex", alignItems:"center", justifyContent:"space-between", marginTop:10 }}>
            <div style={{ display:"flex", alignItems:"center", gap:6 }}>
              <span style={{ fontSize:14, fontFamily:mono, color: card.up ? "#00d4b0" : C.red }}>{card.pnl}</span>
              <span style={{ color:"rgba(255,255,255,0.5)", fontSize:9, fontFamily:mono }}>{card.usd}</span>
            </div>
            <span style={{ color:"rgba(0,212,176,0.6)", fontSize:8, fontFamily:mono }}>HASH: {card.tx}</span>
          </div>
        </div>

        {/* AI NARRATION */}
        <div style={{ padding:"12px 14px", borderBottom:`1px solid rgba(0,212,176,0.05)` }}>
          <div style={{ display:"flex", alignItems:"center", gap:4, marginBottom:4 }}>
            <span style={{ color:"rgba(150,90,255,0.6)", fontSize:7 }}>✦</span>
            <span style={{ color:"rgba(255,255,255,0.5)", fontSize:7, letterSpacing:"0.25em", fontFamily:mono }}>AI NARRATION</span>
          </div>
          <p style={{ color:"rgba(255,255,255,0.85)", fontSize:12, lineHeight:1.7, fontFamily:tw, margin:0 }}>{card.narration}</p>
        </div>

        {/* SOUNDTRACK */}
        <div style={{ padding:"10px 14px", borderBottom:`1px solid rgba(0,212,176,0.05)` }}>
          <div style={{ display:"flex", alignItems:"center", justifyContent:"space-between", marginBottom:4 }}>
            <span style={{ color:"rgba(255,255,255,0.5)", fontSize:7, letterSpacing:"0.25em", fontFamily:mono }}>♫ SOUNDTRACK</span>
            <span style={{ color:"rgba(255,255,255,0.4)", fontSize:7, fontFamily:mono }}>AUDIUS</span>
          </div>
          {soundtrack ? (
            <AudioPlayerUI track={soundtrack} isPlaying={audio.playing} isCurrent={isCurrent} onPlay={audio.play} progress={isCurrent ? audio.progress : 0} currentTime={isCurrent ? audio.currentTime : 0} />
          ) : (
            <div style={{ padding:"8px 10px", border:"1px solid rgba(0,212,176,0.08)", background:"rgba(0,0,0,0.3)" }}>
              <span style={{ color:"rgba(255,255,255,0.5)", fontSize:9, fontFamily:mono }}>loading soundtrack...</span>
            </div>
          )}
        </div>

        {/* ACTIONS - like, comment, share, mint */}
        <div style={{ padding:"10px 14px", display:"flex", alignItems:"center", gap:16 }}>
          <button onClick={() => onLike(card.id)} style={{ display:"flex", alignItems:"center", gap:4, background:"none", border:"none", cursor:"pointer", color: card.liked ? C.red : "rgba(255,255,255,0.55)" }}>
            <span style={{ fontSize:14, fontFamily:mono }}>{card.liked ? "♥" : "♡"}</span>
            <span style={{ fontSize:11, fontFamily:mono }}>{fmt(card.likes)}</span>
          </button>
          <button onClick={() => setShowCom(!showCom)} style={{ display:"flex", alignItems:"center", gap:4, background:"none", border:"none", cursor:"pointer", color:"rgba(255,255,255,0.55)" }}>
            <span style={{ fontSize:11, fontFamily:mono }}>{fmt(card.comments)} comments</span>
          </button>
          <button style={{ background:"none", border:"none", cursor:"pointer", color:"rgba(255,255,255,0.55)", fontSize:11, fontFamily:mono }}>SHARE</button>
          <div style={{ marginLeft:"auto" }}><MintButton onMint={onMint} /></div>
        </div>

        {showCom && (
          <div style={{ padding:"0 14px 12px", borderTop:"1px solid rgba(0,212,176,0.05)" }}>
            <div style={{ paddingTop:10, marginBottom:6 }}><span style={{ color:"#00d4b0", fontSize:12, fontFamily:mono }}>@sol_maxi</span><span style={{ color:"rgba(255,255,255,0.4)", fontSize:12, fontFamily:mono, marginLeft:8 }}>the AI narration hit different on this one</span></div>
            <div style={{ marginBottom:4 }}><span style={{ color:"#00d4b0", fontSize:12, fontFamily:mono }}>@ngmi</span><span style={{ color:"rgba(255,255,255,0.4)", fontSize:12, fontFamily:mono, marginLeft:8 }}>that soundtrack choice is perfect lmao</span></div>
          </div>
        )}
      </div>
    </div>
  );
}

function ScanView({ onDone, walletAddr, onCardsFound }) {
  const [phase, setPhase] = useState(0);
  const [txC, setTxC] = useState(0);
  const [moments, setMoments] = useState([]);
  const [prog, setProg] = useState(0);

  const TYPE_MARKS = { swap:"\u26A1", mint:"\u2020", big_move:"\u25B2", diamond_hands:"\u25C6" };

  useEffect(() => {
    setTimeout(() => setPhase(1), 600);
    
    // Progress bar animation
    const pi = setInterval(() => setProg(p => { if(p>=95){clearInterval(pi);return 95;} return p+0.8; }), 80);

    // If we have a wallet, do real scan
    if (walletAddr) {
      // Animate tx counter while loading
      let c = 0;
      const ti = setInterval(() => { c += Math.floor(Math.random()*12)+3; setTxC(c); }, 200);

      scanWallet(walletAddr).then(cards => {
        clearInterval(ti);
        setTxC(cards.length > 0 ? 50 : 0);
        
        if (cards.length > 0) {
          // Reveal cards one by one
          cards.slice(0, 8).forEach((card, i) => {
            setTimeout(() => {
              setMoments(p => [...p, {
                mark: TYPE_MARKS[card.type] || "\u26A1",
                text: card.title + " (" + card.date + ")",
              }]);
            }, 800 + i * 500);
          });
          
          // Finish
          setTimeout(() => { setProg(100); setPhase(2); }, 800 + cards.length * 500);
          setTimeout(() => { setPhase(3); onCardsFound(cards); }, 1500 + cards.length * 500);
        } else {
          // No interesting txs found - use demo
          setTxC(0);
          setTimeout(() => { setProg(100); setPhase(2); }, 2000);
          setTimeout(() => { setPhase(3); }, 3000);
        }
      });
    } else {
      // Demo mode - fake scan
      let c = 0;
      const ti = setInterval(() => { c += Math.floor(Math.random()*15)+5; if(c>247)c=247; setTxC(c); if(c>=247)clearInterval(ti); }, 180);
      
      const demoFound = [
        { mark:"\u26A1", text:"420 SOL \u2192 BONK (XII.2024)", d:1200 },
        { mark:"\u2620", text:"RUGGED \u2014 $SQUIDGAME (-99.8%)", d:1800 },
        { mark:"\u2020", text:"MINTED DeGods #4271 \u2014 FIRST 100", d:2400 },
        { mark:"\u25C6", text:"HELD SOL THROUGH FTX CRASH", d:3000 },
        { mark:"\u25B2", text:"10,000 SOL \u2192 COLD STORAGE", d:3600 },
      ];
      demoFound.forEach(m => setTimeout(() => setMoments(p => [...p, m]), m.d));
      setTimeout(() => { setProg(100); setPhase(2); }, 4200);
      setTimeout(() => setPhase(3), 5800);
      
      return () => { clearInterval(ti); clearInterval(pi); };
    }
    
    return () => { clearInterval(pi); };
  }, []);

  return (
    <div style={{ minHeight:"100vh", display:"flex", alignItems:"center", justifyContent:"center", padding:16, position:"relative", zIndex:10 }}>
      <div style={{ maxWidth:440, width:"100%" }}>
        <div style={{ textAlign:"center", marginBottom:28 }}>
          <Logo size="lg" />
          <div style={{ marginTop:10 }}><span style={{ color:"rgba(0,212,176,0.75)", fontSize:10, fontFamily:mono, letterSpacing:"0.3em" }}>RESURRECTING THE DEAD</span></div>
        </div>
        <div style={{ border:"1px solid rgba(0,212,176,0.15)", padding:20, position:"relative", overflow:"hidden", background:"rgba(20,10,45,0.65)", backdropFilter:"blur(8px)" }}>
          <Noise opacity={0.08} animated={true} />
          <div className="crt-lines" style={{ position:"absolute", inset:0, opacity:0.04, pointerEvents:"none" }} />
          <div style={{ position:"relative", zIndex:10 }}>
            <div style={{ display:"flex", alignItems:"center", gap:8, marginBottom:14 }}>
              <span style={{ color:"#00d4b0", fontSize:12, fontFamily:mono }}>SCANNING WALLET</span>
              <span style={{ color:"#9660ff", fontSize:12, fontFamily:mono }}>{walletAddr ? shortenAddress(walletAddr) : "DEMO"}</span>
            </div>
            <div style={{ height:3, background:"rgba(0,212,176,0.1)", marginBottom:16, overflow:"hidden" }}><div style={{ height:"100%", width:`${prog}%`, background:"#00d4b0", boxShadow:"0 0 8px rgba(0,212,176,0.5)", transition:"width 0.1s" }} /></div>
            <div style={{ display:"flex", justifyContent:"space-between", marginBottom:3 }}><span style={{ color:"rgba(255,255,255,0.4)", fontSize:12, fontFamily:mono }}>TRANSACTIONS</span><span style={{ color:"#00d4b0", fontSize:16, fontFamily:mono }}>{txC}</span></div>
            <div style={{ display:"flex", justifyContent:"space-between", marginBottom:14 }}><span style={{ color:"rgba(255,255,255,0.4)", fontSize:12, fontFamily:mono }}>STORIES FOUND</span><span style={{ color:"#c8a030", fontSize:16, fontFamily:mono }}>{moments.length}</span></div>
            <Dash style={{ marginBottom:12 }} />
            <div style={{ display:"flex", flexDirection:"column", gap:5, marginBottom:16 }}>
              {moments.map((m,i) => (
                <div key={i} style={{ display:"flex", alignItems:"center", gap:8, padding:"7px 10px", border:"1px solid rgba(0,212,176,0.12)", background:"rgba(10,5,25,0.6)", animation:"fadeSlideIn 0.4s ease-out", position:"relative", overflow:"hidden" }}>
                  <Noise opacity={0.04} animated={false} />
                  <span style={{ fontSize:14, color:"#00d4b0", fontFamily:mono, position:"relative", zIndex:1 }}>{m.mark}</span>
                  <span style={{ color:"rgba(255,255,255,0.7)", fontSize:13, fontFamily:mono, position:"relative", zIndex:1 }}>{m.text}</span>
                </div>
              ))}
            </div>
            {phase >= 2 && (
              <div style={{ padding:"8px 10px", marginBottom:14, border:`1px solid ${phase===3 ? "rgba(0,212,176,0.25)" : "rgba(0,212,176,0.1)"}`, background:phase===3 ? "rgba(0,212,176,0.03)" : "transparent" }}>
                <span style={{ color:phase===3 ? "#00d4b0" : "rgba(255,255,255,0.4)", fontSize:12, fontFamily:mono }}>
                  {phase===2 ? "GENERATING NARRATIONS & SOUNDTRACKS..." : "CRYPT READY. 5 CARDS RESURRECTED."}
                </span>
              </div>
            )}
            {phase === 3 && (
              <button onClick={onDone} style={{ width:"100%", padding:"14px 0", fontWeight:400, fontSize:14, color:"#00d4b0", fontFamily:mono, cursor:"pointer", letterSpacing:"0.2em", border:"1px solid rgba(0,212,176,0.4)", background:"rgba(20,10,45,0.65)", transition:"border-color 120ms" }}
                onMouseEnter={e => e.target.style.borderColor = "rgba(0,212,176,0.8)"}
                onMouseLeave={e => e.target.style.borderColor = "rgba(0,212,176,0.6)"}>
                REVIEW YOUR CARDS {"\u2192"}
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

export default function App() {
  const [view, setView] = useState("connect");
  const [walletAddr, setWalletAddr] = useState(null);
  const [wallets, setWallets] = useState([]);
  const [inputAddr, setInputAddr] = useState("");
  const [showWalletPicker, setShowWalletPicker] = useState(false);
  const sessionSeed = useRef(Math.random().toString(36).slice(2, 8));
  const [cards, setCards] = useState(() => DEMO_CARDS.map((c, i) => ({...c, tx: c.tx + sessionSeed.current + i})));
  const [tapestryProfile, setTapestryProfile] = useState(null);
  const [socialIdentity, setSocialIdentity] = useState(null);
  const [soundtracks, setSoundtracks] = useState({});
  const [mintActive, setMintActive] = useState(false);
  const mintCallbackRef = useRef(null);
  const [connectedWallet, setConnectedWallet] = useState(null);
  const audio = useAudioPlayer();

  useEffect(() => {
    const check = () => {
      const found = detectWallets();
      if (found.length > 0) setWallets(found);
    };
    check();
    const interval = setInterval(check, 500);
    const timeout = setTimeout(() => clearInterval(interval), 5000);
    return () => { clearInterval(interval); clearTimeout(timeout); };
  }, []);

  useEffect(() => {
    if (view !== "feed") return;
    
    const usedTrackIds = new Set();
    
    // Load trending as fallback pool
    let trendingPool = [];
    getTrending(30).then(t => { trendingPool = t; });

    cards.forEach((card, idx) => {
      if (soundtracks[card.id]) return;
      const queries = MOOD_QUERIES[card.type] || ["electronic"];
      
      setTimeout(async () => {
        // Search with more results so we can pick an unused one
        let picked = null;
        for (const q of queries) {
          const { searchTracks } = await import("./audius");
          const results = await searchTracks(q, 8);
          const unused = results.filter(t => !usedTrackIds.has(t.id));
          if (unused.length > 0) {
            picked = unused[Math.floor(Math.random() * Math.min(3, unused.length))];
            break;
          }
        }
        
        // Fallback to trending (also deduplicated)
        if (!picked && trendingPool.length > 0) {
          const unused = trendingPool.filter(t => !usedTrackIds.has(t.id));
          if (unused.length > 0) {
            picked = unused[idx % unused.length];
          }
        }
        
        if (picked) {
          usedTrackIds.add(picked.id);
          setSoundtracks(prev => ({ ...prev, [card.id]: picked }));
        }
      }, idx * 600);
    });
  }, [view]);

  const handleLike = (id) => setCards(cs => cs.map(c => c.id===id ? {...c, liked:!c.liked, likes:c.liked?c.likes-1:c.likes+1} : c));

  const handleConnect = async (wallet) => {
    const addr = await connectWallet(wallet.provider);
    if (addr) {
      setWalletAddr(addr);
      setConnectedWallet(wallet.provider);
      setView("scan");
      // Resolve social identity via Tapestry
      resolveIdentity(addr).then(id => { if (id) setSocialIdentity(id); });
      findOrCreateProfile(addr).then(p => { if (p) setTapestryProfile(p); });
    }
  };

  if (view === "connect") {
    return (
      <Graveyard
        wallets={wallets}
        onWalletConnect={handleConnect}
        onScan={(addr) => {
          if (addr) {
            setWalletAddr(addr);
            resolveIdentity(addr).then(id => { if (id) setSocialIdentity(id); });
          }
          setView("scan");
        }}
      />
    );
  }

  if (view === "scan") {
    return (
      <>
        <ZineBg />
        <ScanView walletAddr={walletAddr} onDone={() => setView("feed")} onCardsFound={(c) => { if (c.length > 0) setCards(c); }} />
      </>
    );
  }

  return (
    <>
      <ZineBg />
      <div style={{ position:"relative", zIndex:10 }}>
        <nav style={{ position:"sticky", top:0, zIndex:50, padding:"10px 16px", background:"rgba(4,4,10,0.92)", backdropFilter:"blur(12px)", borderBottom:"1px solid rgba(0,212,176,0.08)" }}>
          <div style={{ maxWidth:600, margin:"0 auto", display:"flex", alignItems:"center", justifyContent:"space-between" }}>
            <div style={{ cursor:"pointer", display:"flex", alignItems:"center", gap:8 }}>
              <img src="/ghost.png" alt="" style={{ width:24, height:24, objectFit:"contain" }} />
              <span style={{ fontFamily:"'Press Start 2P',monospace", fontWeight:400, fontSize:10, letterSpacing:"0.2em", color:"#00d4b0", textShadow:"1px 1px 0 #2a1a50" }}>CRYPT</span>
            </div>
            <div style={{ display:"flex", alignItems:"center", gap:12 }}>
              {walletAddr && <span style={{ color: socialIdentity?.username ? C.white : C.dim, fontSize:8, fontFamily:mono }}>{socialIdentity?.username ? ("@" + socialIdentity.username) : shortenAddress(walletAddr)}</span>}
            </div>
          </div>
        </nav>
        <div style={{ padding:"20px 16px", maxWidth:600, margin:"0 auto" }}>
          <div style={{ textAlign:"center", padding:"10px 0 24px" }}>
            <span style={{ color:"rgba(255,255,255,0.5)", fontSize:8, fontFamily:mono, letterSpacing:"0.35em" }}>{"\u25CE"} LATEST FROM THE GRAVEYARD</span>
          </div>
          {cards.map((c, i) => (
            <CryptCard key={c.id} card={c} onLike={handleLike} onMint={(cb) => { mintCallbackRef.current = { cb, card: c }; setMintActive(true); }} soundtrack={soundtracks[c.id]} audio={audio} delay={i * 120} />
          ))}
          <div style={{ textAlign:"center", padding:"30px 0 60px" }}>
            <Dash style={{ marginBottom:12 }} />
            <span style={{ color:"rgba(255,255,255,0.45)", fontSize:7, fontFamily:mono, letterSpacing:"0.5em" }}>END OF FEED {"\u00B7"} THE DEAD REST</span>
          </div>
        </div>
      </div>
      {audio.currentTrack && (
        <div style={{ position:"fixed", bottom:0, left:0, right:0, zIndex:100, background:"rgba(8,8,12,0.96)", borderTop:`1px solid ${C.green}20`, padding:"8px 16px", backdropFilter:"blur(12px)" }}>
          <div style={{ maxWidth:600, margin:"0 auto" }}>
            <AudioPlayerUI track={audio.currentTrack} isPlaying={audio.playing} isCurrent={true} onPlay={audio.play} progress={audio.progress} currentTime={audio.currentTime} />
          </div>
        </div>
      )}
      {mintActive && <MintCeremony onComplete={async () => {
        setMintActive(false);
        const ref = mintCallbackRef.current;
        if (!ref) return;
        const { cb, card } = ref;
        if (connectedWallet && connectedWallet.publicKey) {
          try {
            const { mintCard, buildCardMetadata } = await import("./mint.js");
            const metadata = buildCardMetadata(card, soundtracks[card.id], null);
            const result = await mintCard(card, connectedWallet, metadata);
            if (result.success) {
              console.log("ON-CHAIN MINT:", result.explorer);
            }
          } catch (e) {
            console.warn("Mint error:", e);
          }
        }
        if (cb) cb();
      }} />}
    </>
  );
}
