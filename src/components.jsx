import React from "react";
import { C, mono } from "./theme";

export function Logo({ size = "lg" }) {
  const isLg = size === "lg";
  return (
    <div style={{ display:"flex", flexDirection:"column", alignItems:"center", gap: isLg ? 0 : 0 }}>
      <img src="/ghost.png" alt="CRYPT" style={{
        width: isLg ? 180 : 24,
        height: isLg ? 180 : 24,
        objectFit:"contain",
        imageRendering:"auto",
      }} />
      {isLg && (
        <div style={{
          fontFamily:"'Press Start 2P',monospace",
          fontWeight:400,
          fontSize: Math.min(48, window.innerWidth * 0.14),
          letterSpacing: "0.2em",
          color: "#00d4b0",
          textShadow: "1px 1px 0 #2a1a50",
          marginTop: 14,
        }}>CRYPT</div>
      )}
    </div>
  );
}

export function ZineBg() {
  return (
    <>
      {/* Graveyard background image (darkened) */}
      <div style={{
        position:"fixed", inset:0, pointerEvents:"none", zIndex:0,
        backgroundImage:"url(/graveyard-bg.png)",
        backgroundSize:"cover", backgroundPosition:"center",
        filter:"brightness(0.25)",
        imageRendering:"pixelated",
      }} />
      {/* Dark overlay for readability */}
      <div style={{
        position:"fixed", inset:0, pointerEvents:"none", zIndex:0,
        background:"rgba(4,4,10,0.55)",
      }} />
      {/* Animated scanline */}
      <div style={{
        position:"fixed", inset:0, pointerEvents:"none", zIndex:1,
        overflow:"hidden",
      }}>
        <div style={{
          position:"absolute", left:0, right:0, height:"4px",
          background:"linear-gradient(180deg, transparent, rgba(0,204,102,0.06), transparent)",
          animation:"scanline 8s linear infinite",
        }} />
      </div>
      {/* Static noise layer */}
      <div style={{
        position:"fixed", inset:0, pointerEvents:"none", zIndex:1,
        opacity:0.03,
        backgroundImage:`url("data:image/svg+xml,%3Csvg viewBox='0 0 256 256' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E")`,
        animation:"staticNoise 0.5s steps(8) infinite",
      }} />
    </>
  );
}

export function Noise({ opacity = 0.06, animated = false }) {
  return (
    <div style={{
      position:"absolute", inset:0, pointerEvents:"none",
      opacity,
      backgroundImage:`url("data:image/svg+xml,%3Csvg viewBox='0 0 256 256' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.85' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E")`,
      ...(animated ? { animation:"staticNoise 0.3s steps(5) infinite" } : {}),
    }} />
  );
}

export function Dash({ style = {} }) {
  return (
    <div style={{
      borderBottom:`1px dashed ${C.stroke}`,
      opacity:0.6,
      ...style,
    }} />
  );
}

export function PixelDivider({ text, color = C.green }) {
  return (
    <div style={{ display:"flex", alignItems:"center", gap:10, margin:"4px 0" }}>
      <div style={{ flex:1, height:"1px", background:`linear-gradient(90deg, transparent, ${color}40, transparent)` }} />
      {text && <span style={{ color: color, fontSize:10, fontFamily:mono, letterSpacing:"0.25em", textShadow:`0 0 8px ${color}40` }}>{text}</span>}
      <div style={{ flex:1, height:"1px", background:`linear-gradient(90deg, transparent, ${color}40, transparent)` }} />
    </div>
  );
}

export function GlitchText({ children, style = {} }) {
  return (
    <span style={{
      position:"relative",
      animation:"rgbSplit 6s ease-in-out infinite",
      ...style,
    }}>{children}</span>
  );
}

export function AudioPlayerUI({ track, isPlaying, isCurrent, onPlay, progress = 0, currentTime = 0 }) {
  return (
    <div
      onClick={() => onPlay(track)}
      style={{
        display:"flex", alignItems:"center", gap:10, padding:"8px 12px",
        border:`1px solid ${isCurrent ? C.green+"50" : C.stroke}`,
        background: isCurrent ? "rgba(0,204,102,0.04)" : C.inner,
        cursor:"pointer", position:"relative", overflow:"hidden",
        transition:"all 0.2s",
      }}
    >
      <Noise opacity={0.04} animated={false} />
      <div style={{ position:"relative", zIndex:1, display:"flex", alignItems:"center", gap:10, width:"100%" }}>
        {/* Play/Pause */}
        <div style={{
          width:28, height:28, display:"flex", alignItems:"center", justifyContent:"center",
          border:`1px solid ${isCurrent && isPlaying ? C.green : C.stroke}`,
          color: isCurrent && isPlaying ? C.green : C.gray,
          fontSize:11, fontFamily:"monospace", flexShrink:0,
          background: isCurrent && isPlaying ? "rgba(0,204,102,0.08)" : "transparent",
          textShadow: isCurrent && isPlaying ? `0 0 6px ${C.greenGlow}` : "none",
        }}>
          {isCurrent && isPlaying ? "▐▐" : "▶"}
        </div>
        {/* Track info */}
        <div style={{ flex:1, minWidth:0 }}>
          <div style={{
            color: isCurrent ? C.green : C.white, fontSize:13, fontFamily:mono,
            whiteSpace:"nowrap", overflow:"hidden", textOverflow:"ellipsis",
            textShadow: isCurrent ? `0 0 8px ${C.greenGlow}` : "none",
          }}>
            {track.title}
          </div>
          <div style={{ color:C.dim, fontSize:11, fontFamily:mono }}>{track.artist}</div>
        </div>
        {/* Progress */}
        {isCurrent && (
          <div style={{ width:50, textAlign:"right" }}>
            <span style={{ color:C.green, fontSize:11, fontFamily:mono }}>{Math.floor(currentTime/60)}:{String(Math.floor(currentTime%60)).padStart(2,"0")}</span>
          </div>
        )}
      </div>
      {/* Progress bar */}
      {isCurrent && (
        <div style={{ position:"absolute", bottom:0, left:0, right:0, height:2, background:C.faint }}>
          <div style={{ height:"100%", width:`${progress}%`, background:C.green, boxShadow:`0 0 6px ${C.green}`, transition:"width 0.3s" }} />
        </div>
      )}
    </div>
  );
}
