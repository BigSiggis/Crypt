import React, { useState, useEffect, useRef } from "react";
import { C, mono } from "./theme";

// ============ CRYPT — HAUNTED ARCADE BOOT SCREEN ============
// Not a landing page. A blockchain ritual.
// Calm. Dark. Charged. Controlled.

export default function Graveyard({ onConnect, onScan, wallets, onWalletConnect }) {
  const canvasRef = useRef(null);
  const animRef = useRef(null);
  const imgRef = useRef(null);
  const [uiPhase, setUiPhase] = useState(0);
  const [inputAddr, setInputAddr] = useState("");
  const [showWalletPicker, setShowWalletPicker] = useState(false);

  useEffect(() => {
    const t1 = setTimeout(() => setUiPhase(1), 600);
    const t2 = setTimeout(() => setUiPhase(2), 1200);
    const t3 = setTimeout(() => setUiPhase(3), 1800);
    return () => { clearTimeout(t1); clearTimeout(t2); clearTimeout(t3); };
  }, []);

  useEffect(() => {
    const img = new Image();
    img.src = "/graveyard-bg.png";
    img.onload = () => { imgRef.current = img; };
  }, []);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d", { alpha: false });
    const DPR = Math.min(2, window.devicePixelRatio || 1);

    function resize() {
      canvas.width = Math.floor(innerWidth * DPR);
      canvas.height = Math.floor(innerHeight * DPR);
      ctx.imageSmoothingEnabled = false;
    }
    resize();
    addEventListener("resize", resize);

    // Image coordinate system
    let iDx = 0, iDy = 0, iDw = 0, iDh = 0;
    function i2c(nx, ny) { return [iDx + nx * iDw, iDy + ny * iDh]; }
    function iSc(n) { return n * iDw; }

    function drawBg(img) {
      const cw = canvas.width, ch = canvas.height;
      const ir = img.width / img.height, cr = cw / ch;
      if (cr < ir) { iDh = ch; iDw = iDh * ir; }
      else { iDw = cw; iDh = iDw / ir; }
      iDx = (cw - iDw) * 0.5;
      iDy = (ch - iDh) * 0.5;
      ctx.drawImage(img, iDx, iDy, iDw, iDh);
    }

    // === CANDLE POSITIONS (exact image coords) ===
    const CANDLES = [
      { x: 0.22, y: 0.525, p: 0, freq: 7 + Math.random() * 4 },
      { x: 0.27, y: 0.54, p: 1.2, freq: 8 + Math.random() * 5 },
      { x: 0.32, y: 0.535, p: 2.8, freq: 6 + Math.random() * 4 },
      { x: 0.38, y: 0.55, p: 0.7, freq: 9 + Math.random() * 3 },
      { x: 0.42, y: 0.545, p: 3.5, freq: 7 + Math.random() * 5 },
      { x: 0.58, y: 0.545, p: 1.9, freq: 8 + Math.random() * 4 },
      { x: 0.63, y: 0.55, p: 2.3, freq: 6 + Math.random() * 5 },
      { x: 0.69, y: 0.535, p: 0.4, freq: 9 + Math.random() * 4 },
      { x: 0.73, y: 0.54, p: 3.1, freq: 7 + Math.random() * 3 },
      { x: 0.78, y: 0.525, p: 1.6, freq: 8 + Math.random() * 5 },
    ];

    // === GREEN EYES ===
    const EYES = [
      { x: 0.09, y: 0.415, s: 0.006, p: 0, bt: 0, bi: 5 + Math.random() * 5 },
      { x: 0.105, y: 0.415, s: 0.005, p: 0, bt: 0.3, bi: 5 + Math.random() * 5 },
      { x: 0.89, y: 0.42, s: 0.006, p: 2, bt: 0, bi: 6 + Math.random() * 5 },
      { x: 0.905, y: 0.42, s: 0.005, p: 2, bt: 0.2, bi: 6 + Math.random() * 5 },
      { x: 0.15, y: 0.40, s: 0.004, p: 1.5, bt: 0, bi: 7 + Math.random() * 6 },
      { x: 0.162, y: 0.40, s: 0.004, p: 1.5, bt: 0.15, bi: 7 + Math.random() * 6 },
      { x: 0.84, y: 0.395, s: 0.004, p: 3, bt: 0, bi: 8 + Math.random() * 5 },
      { x: 0.852, y: 0.395, s: 0.004, p: 3, bt: 0.1, bi: 8 + Math.random() * 5 },
    ];

    // === LOW FOG (bottom 28% only) ===
    const FOG = Array.from({ length: 10 }, () => ({
      x: Math.random() * 1.4 - 0.2, y: 0.72 + Math.random() * 0.22,
      r: 130 + Math.random() * 250,
      vx: (0.02 + Math.random() * 0.03) * (Math.random() > 0.5 ? 1 : -1),
      a: 0.04 + Math.random() * 0.06,
    }));



    // === GLITCH STATE ===
    let nextGlitch = 14 + Math.random() * 6;
    let glitchType = 0; // 0=none, 1=hShift, 2=vNudge, 3=rgb, 4=letterFlicker
    let glitchFrames = 0;

    // === MOON SHIMMER STATE ===
    let nextShimmer = 3 + Math.random() * 3;
    let shimmerActive = false;
    let shimmerTime = 0;

    function rgba(r, g, b, a) { return `rgba(${r},${g},${b},${a})`; }

    const t0 = performance.now();

    function frame(now) {
      const t = (now - t0) / 1000;
      const cw = canvas.width, ch = canvas.height;

      // === DRAW BASE IMAGE ===
      ctx.fillStyle = "#04040a";
      ctx.fillRect(0, 0, cw, ch);
      if (imgRef.current) drawBg(imgRef.current);
      if (!imgRef.current) { animRef.current = requestAnimationFrame(frame); return; }

      // ============ MOON — SLOW RITUAL PULSE (9s cycle) ============
      const [mx, my] = i2c(0.50, 0.155);
      const moonR = iSc(0.085);
      // Brightness: 100% → 108% → 100%
      const mp = 1.0 + 0.08 * Math.sin(t / 9 * Math.PI * 2);
      const mg = ctx.createRadialGradient(mx, my, moonR * 0.2, mx, my, moonR * 2.8);
      mg.addColorStop(0, rgba(50, 255, 220, 0.06 * mp));
      mg.addColorStop(0.3, rgba(140, 80, 240, 0.03 * mp));
      mg.addColorStop(1, "rgba(0,0,0,0)");
      ctx.globalCompositeOperation = "screen";
      ctx.fillStyle = mg;
      ctx.fillRect(mx - moonR * 3, my - moonR * 3, moonR * 6, moonR * 6);
      ctx.globalCompositeOperation = "source-over";

      // Moon Solana logo shimmer (every 6s, 1px horizontal)
      nextShimmer -= 0.016;
      if (nextShimmer <= 0) { shimmerActive = true; shimmerTime = 0; nextShimmer = 5 + Math.random() * 2; }
      if (shimmerActive) {
        shimmerTime += 0.016;
        if (shimmerTime > 0.3) { shimmerActive = false; }
        else {
          const shimA = 0.06 * Math.sin(shimmerTime / 0.3 * Math.PI);
          ctx.globalCompositeOperation = "screen";
          ctx.fillStyle = rgba(200, 255, 240, shimA);
          ctx.fillRect(mx - moonR, my - 1 * DPR, moonR * 2, 2 * DPR);
          ctx.globalCompositeOperation = "source-over";
        }
      }

      // ============ CLOUD BOWL BREATH (behind hero, 7s, 2% brightness) ============
      const bowl = 0.5 + 0.5 * Math.sin(t / 7 * Math.PI * 2);
      const bx = cw * 0.5, by = ch * 0.44;
      const bg = ctx.createRadialGradient(bx, by, 0, bx, by, cw * 0.55);
      bg.addColorStop(0, rgba(50, 220, 190, 0.02 * (0.5 + 0.5 * bowl)));
      bg.addColorStop(0.5, rgba(140, 80, 220, 0.015 * (0.5 + 0.5 * bowl)));
      bg.addColorStop(1, "rgba(0,0,0,0)");
      ctx.globalCompositeOperation = "screen";
      ctx.fillStyle = bg;
      ctx.fillRect(0, 0, cw, ch);
      ctx.globalCompositeOperation = "source-over";

      // ============ CANDLES — INDEPENDENT FLICKER ============
      ctx.globalCompositeOperation = "screen";
      CANDLES.forEach(c => {
        const [cx, cy] = i2c(c.x, c.y);
        // Random brightness 85-115%, non-synced
        const flick = 0.85 + 0.30 * (0.5 + 0.5 * Math.sin(t * c.freq + c.p));
        const jitter = (Math.random() - 0.5) * 0.05;
        const brightness = Math.max(0.15, flick + jitter);
        // Tiny vertical flame shift (1-2px)
        const yShift = Math.sin(t * c.freq * 0.7 + c.p) * 2 * DPR;
        const sway = Math.sin(t * 3 + c.p * 2) * 1.5 * DPR;

        const r = 30 * DPR;
        // Warm core (small, bright)
        const core = ctx.createRadialGradient(cx + sway, cy + yShift - 3 * DPR, 0, cx + sway, cy + yShift - 3 * DPR, r * 0.25);
        core.addColorStop(0, rgba(255, 240, 200, 0.08 * brightness));
        core.addColorStop(1, "rgba(0,0,0,0)");
        ctx.fillStyle = core;
        ctx.fillRect(cx - r, cy - r, r * 2, r * 2);

        // Teal ambient (desaturated, controlled)
        const amb = ctx.createRadialGradient(cx + sway, cy + yShift, 0, cx + sway, cy + yShift, r);
        amb.addColorStop(0, rgba(50, 200, 180, 0.05 * brightness));
        amb.addColorStop(0.6, rgba(120, 70, 200, 0.025 * brightness));
        amb.addColorStop(1, "rgba(0,0,0,0)");
        ctx.fillStyle = amb;
        ctx.fillRect(cx - r, cy - r, r * 2, r * 2);
      });
      ctx.globalCompositeOperation = "source-over";

      // ============ GREEN EYES (blink) ============
      EYES.forEach(eye => {
        eye.bt += 0.016;
        if (eye.bt > eye.bi) { eye.bt = 0; eye.bi = 3 + Math.random() * 8; }
        if (eye.bt < 0.15) return;
        const [ex, ey] = i2c(eye.x, eye.y);
        const sz = iSc(eye.s);
        const pulse = 0.5 + 0.5 * Math.sin(t * 1.2 + eye.p);
        ctx.globalCompositeOperation = "screen";
        const eg = ctx.createRadialGradient(ex, ey, 0, ex, ey, sz * 5);
        eg.addColorStop(0, rgba(0, 230, 100, 0.10 * pulse));
        eg.addColorStop(0.5, rgba(0, 230, 100, 0.02 * pulse));
        eg.addColorStop(1, "rgba(0,0,0,0)");
        ctx.fillStyle = eg;
        ctx.fillRect(ex - sz * 5, ey - sz * 5, sz * 10, sz * 10);
        ctx.fillStyle = rgba(0, 230, 100, 0.5 * pulse);
        ctx.beginPath(); ctx.arc(ex, ey, sz, 0, Math.PI * 2); ctx.fill();
        ctx.globalCompositeOperation = "source-over";
      });

      // ============ LOW FOG (bottom 28% only, slow drift) ============
      ctx.globalCompositeOperation = "screen";
      ctx.save();
      ctx.beginPath();
      ctx.rect(0, ch * 0.72, cw, ch * 0.28);
      ctx.clip();
      FOG.forEach(f => {
        f.x += f.vx * 0.016;
        if (f.x < -0.3) f.x = 1.3;
        if (f.x > 1.3) f.x = -0.3;
        const fx = f.x * cw, fy = f.y * ch, r = f.r * DPR;
        const opVar = 0.8 + 0.2 * Math.sin(t * 0.5 + f.x * 3);
        const g = ctx.createRadialGradient(fx, fy, 0, fx, fy, r);
        g.addColorStop(0, rgba(120, 70, 200, f.a * 0.18 * opVar));
        g.addColorStop(0.6, rgba(40, 180, 160, f.a * 0.06 * opVar));
        g.addColorStop(1, "rgba(0,0,0,0)");
        ctx.fillStyle = g;
        ctx.fillRect(fx - r, fy - r, r * 2, r * 2);
      });
      ctx.restore();
      ctx.globalCompositeOperation = "source-over";

      // Bats removed

      // ============ HERO RADIAL DARKENING (replaces dark panel) ============
      // 12-15% darkening behind where text sits, invisible as a shape
      const hg = ctx.createRadialGradient(cw * 0.5, ch * 0.72, cw * 0.05, cw * 0.5, ch * 0.72, cw * 0.45);
      hg.addColorStop(0, "rgba(0,0,0,0.14)");
      hg.addColorStop(0.6, "rgba(0,0,0,0.08)");
      hg.addColorStop(1, "rgba(0,0,0,0)");
      ctx.fillStyle = hg;
      ctx.fillRect(0, ch * 0.4, cw, ch * 0.6);

      // ============ CRT SCANLINE PASS (every 4s, 400ms sweep) ============
      const scanCycle = t % 4;
      if (scanCycle < 0.4) {
        const scanProg = scanCycle / 0.4;
        const scanY = scanProg * ch;
        ctx.globalCompositeOperation = "screen";
        ctx.fillStyle = rgba(255, 255, 255, 0.03);
        ctx.fillRect(0, scanY, cw, 1 * DPR);
        ctx.globalCompositeOperation = "source-over";
      }

      // Static scanlines (very subtle)
      ctx.fillStyle = "rgba(0,0,0,0.018)";
      for (let sy = 0; sy < ch; sy += 3 * DPR) ctx.fillRect(0, sy, cw, DPR);

      // ============ VIGNETTE ============
      const vg = ctx.createRadialGradient(cw * 0.5, ch * 0.48, cw * 0.18, cw * 0.5, ch * 0.48, cw * 0.76);
      vg.addColorStop(0, "rgba(0,0,0,0)");
      vg.addColorStop(0.55, "rgba(0,0,0,0.14)");
      vg.addColorStop(1, "rgba(0,0,0,0.52)");
      ctx.fillStyle = vg;
      ctx.fillRect(0, 0, cw, ch);

      animRef.current = requestAnimationFrame(frame);
    }

    animRef.current = requestAnimationFrame(frame);
    return () => { cancelAnimationFrame(animRef.current); removeEventListener("resize", resize); };
  }, []);

  const handleScan = () => { const a = inputAddr.trim(); if (a.length >= 32 && a.length <= 44) onScan(a); };

  // === MICRO GLITCH for CRYPT text (CSS-driven) ===
  const [glitch, setGlitch] = useState(null);
  useEffect(() => {
    if (uiPhase < 1) return;
    let timeout;
    function scheduleGlitch() {
      const delay = (14 + Math.random() * 6) * 1000;
      timeout = setTimeout(() => {
        const types = ["hShift", "vNudge", "rgb", "flicker"];
        setGlitch(types[Math.floor(Math.random() * types.length)]);
        setTimeout(() => { setGlitch(null); scheduleGlitch(); }, 50 + Math.random() * 50);
      }, delay);
    }
    scheduleGlitch();
    return () => clearTimeout(timeout);
  }, [uiPhase]);

  const cryptStyle = {
    fontFamily: "'Press Start 2P',monospace",
    fontSize: Math.min(48, window.innerWidth * 0.14),
    letterSpacing: "0.2em",
    color: "#00d4b0", // slightly muted teal
    textShadow: "1px 1px 0 #2a1a50", // 1px deep purple shadow, hard offset
    lineHeight: 1,
    position: "relative",
    // Glitch transforms
    transform: glitch === "hShift" ? "translateX(1px)" :
               glitch === "vNudge" ? "translateY(1px)" :
               glitch === "rgb" ? "translateX(0)" : "none",
    filter: glitch === "rgb" ? "hue-rotate(15deg)" : "none",
    opacity: glitch === "flicker" ? 0.7 : 1,
    transition: "none",
  };

  return (
    <div style={{ position: "fixed", inset: 0, overflow: "hidden", background: "#04040a" }}>
      <canvas ref={canvasRef} style={{ position: "absolute", inset: 0, width: "100vw", height: "100vh", imageRendering: "pixelated" }} />

      <div style={{ position: "absolute", inset: 0, zIndex: 10, display: "flex", flexDirection: "column",
        alignItems: "center", justifyContent: "center",
        overflow: "auto", WebkitOverflowScrolling: "touch" }}>

                {/* CRYPT — Primary Sigil at 44% height */}
        <div style={{ opacity: uiPhase >= 1 ? 1 : 0, transform: `translateY(${uiPhase >= 1 ? 0 : 15}px)`,
          transition: "all 0.8s cubic-bezier(0.22,1,0.36,1) 0.15s", textAlign: "center", marginTop: 0 }}>
          <div style={cryptStyle}>CRYPT</div>
        </div>

        {/* SUBTITLE */}
        <div style={{ opacity: uiPhase >= 2 ? 1 : 0, transform: `translateY(${uiPhase >= 2 ? 0 : 8}px)`,
          transition: "all 0.6s cubic-bezier(0.22,1,0.36,1)", marginTop: 4 }}>
          <span style={{ fontFamily: mono, fontSize: Math.min(9, window.innerWidth * 0.023),
            color: "rgba(0,212,176,0.85)", letterSpacing: "0.3em" }}>RESURRECT YOUR WALLET</span>
        </div>

        {/* WALLET UI — Arcade style, no panel */}
        <div style={{ opacity: uiPhase >= 3 ? 1 : 0, transform: `translateY(${uiPhase >= 3 ? 0 : 15}px)`,
          transition: "all 0.8s cubic-bezier(0.22,1,0.36,1)", width: "100%", maxWidth: 310,
          padding: "0 16px", marginTop: 16 }}>

          {wallets.length > 0 ? (
            <button onClick={() => { if (wallets.length === 1) onWalletConnect(wallets[0]); else setShowWalletPicker(!showWalletPicker); }}
              style={{ width: "100%", padding: "13px 16px", background: "transparent",
                border: "1px solid rgba(0,212,176,0.4)", color: "#00d4b0", fontFamily: mono,
                fontSize: 13, letterSpacing: "0.18em", cursor: "pointer",
                transition: "border-color 120ms, text-shadow 120ms" }}
              onMouseEnter={e => { e.target.style.borderColor = "rgba(0,212,176,0.8)"; e.target.style.textShadow = "0 0 8px rgba(0,212,176,0.3)"; }}
              onMouseLeave={e => { e.target.style.borderColor = "rgba(0,212,176,0.4)"; e.target.style.textShadow = "none"; }}>
              ⚡ CONNECT WALLET
            </button>
          ) : (
            <div style={{ textAlign: "center" }}>
              <div style={{ color: "rgba(255,255,255,0.7)", fontSize: 11, fontFamily: mono, marginBottom: 8 }}>NO WALLETS DETECTED</div>
              <div style={{ display: "flex", gap: 5, justifyContent: "center", flexWrap: "wrap" }}>
                {["Phantom", "Solflare", "Backpack"].map(n => (
                  <a key={n} href={`https://${n.toLowerCase()}.app`} target="_blank" rel="noopener noreferrer"
                    style={{ padding: "4px 8px", fontSize: 9, fontFamily: mono,
                      border: "1px solid rgba(0,212,176,0.5)", color: "rgba(0,212,176,0.85)",
                      textDecoration: "none", background: "transparent" }}>GET {n.toUpperCase()}</a>
                ))}
              </div>
            </div>
          )}

          {showWalletPicker && wallets.length > 1 && (
            <div style={{ marginTop: 5, display: "flex", flexDirection: "column", gap: 4 }}>
              {wallets.map((ww, i) => (
                <button key={i} onClick={() => { setShowWalletPicker(false); onWalletConnect(ww); }}
                  style={{ width: "100%", padding: "10px", display: "flex", alignItems: "center",
                    justifyContent: "center", gap: 7, background: "transparent",
                    color: "#00d4b0", border: "1px solid rgba(0,212,176,0.3)",
                    fontFamily: mono, fontSize: 11, cursor: "pointer" }}>
                  {ww.icon && <img src={ww.icon} alt="" style={{ width: 14, height: 14 }} onError={e => e.target.style.display = "none"} />}
                  {ww.name.toUpperCase()}
                </button>
              ))}
            </div>
          )}

          <div style={{ display: "flex", alignItems: "center", gap: 8, margin: "12px 0" }}>
            <div style={{ flex: 1, height: 1, background: "linear-gradient(90deg,transparent,rgba(0,212,176,0.3),transparent)" }} />
            <span style={{ color: "rgba(255,255,255,0.5)", fontSize: 9, fontFamily: mono }}>OR</span>
            <div style={{ flex: 1, height: 1, background: "linear-gradient(90deg,transparent,rgba(0,212,176,0.3),transparent)" }} />
          </div>

          <div style={{ display: "flex", gap: 4 }}>
            <input value={inputAddr} onChange={e => setInputAddr(e.target.value)} placeholder="PASTE ADDRESS..."
              style={{ flex: 1, padding: "10px 8px 10px 14px", background: "rgba(20,10,45,0.82)",
                border: "1px solid rgba(0,212,176,0.40)", color: "#00d4b0",
                fontFamily: mono, fontSize: 11, outline: "none" }}
              onFocus={e => e.target.style.borderColor = "rgba(0,212,176,0.5)"}
              onBlur={e => e.target.style.borderColor = "rgba(0,212,176,0.25)"}
              onKeyDown={e => e.key === "Enter" && handleScan()} />
            <button onClick={handleScan} style={{ padding: "10px 12px", background: "rgba(20,10,45,0.82)",
              color: inputAddr.trim().length >= 32 ? "#00d4b0" : "rgba(255,255,255,0.5)",
              border: `1px solid ${inputAddr.trim().length >= 32 ? "rgba(0,212,176,0.6)" : "rgba(0,212,176,0.35)"}`,
              fontFamily: mono, fontSize: 11, cursor: "pointer" }}>SCAN</button>
          </div>

          <div style={{ textAlign: "center", marginTop: 10 }}>
            <button onClick={() => onScan(null)} style={{ background: "none", border: "none",
              cursor: "pointer", color: "rgba(255,255,255,0.8)", fontSize: 14,
              fontFamily: mono, letterSpacing: "0.15em" }}>DEMO MODE</button>
          </div>

          <div style={{ marginTop: 12 }} />
        </div>
      </div>

        {/* HACKATHON BADGE — fixed bottom */}
        <div style={{ position: "absolute", bottom: "2vh", left: 0, right: 0, textAlign: "center", zIndex: 20 }}>
          <span style={{ color: "rgba(200,160,48,0.8)", fontSize: Math.min(10, window.innerWidth * 0.025),
            fontFamily: "'Press Start 2P',monospace", letterSpacing: "0.08em",
            textShadow: "0 0 8px rgba(200,160,48,0.3)" }}>SOLANA GRAVEYARD HACKATHON</span>
        </div>
    </div>
  );
}
