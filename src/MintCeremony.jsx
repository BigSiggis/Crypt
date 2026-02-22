import React, { useEffect, useRef } from "react";

// ============ MINT CEREMONY - PIXEL GLOW RESURRECTION ============
// On-brand: pixel blocks, teal/purple, no smooth gradients, no emojis, no runes.
// 0.0-0.8: dim + card lifts
// 0.8-2.5: pixel glow blocks pulse inward
// 2.5-3.5: energy builds, card shakes
// 3.5-4.0: flash + pixel burst
// 4.0-8.5: "MINTED / SOUL PRESERVED ON-CHAIN" (4.5s hold)
// 8.5-9.2: fade out

export default function MintCeremony({ onComplete }) {
  const canvasRef = useRef(null);
  const animRef = useRef(null);
  const t0 = useRef(Date.now());

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    const dpr = Math.min(2, window.devicePixelRatio || 1);
    const w = window.innerWidth;
    const h = window.innerHeight;
    canvas.width = w * dpr;
    canvas.height = h * dpr;
    canvas.style.width = w + "px";
    canvas.style.height = h + "px";
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);

    const cx = w / 2, cy = h / 2;
    const cardW = Math.min(300, w * 0.75);
    const cardH = cardW * 1.45;

    // Pixel glow blocks that orbit and collapse inward
    const GLOWS = [];
    for (let i = 0; i < 45; i++) {
      GLOWS.push({
        angle: (i / 45) * Math.PI * 2,
        baseR: 160 + Math.random() * 100,
        size: 3 + Math.floor(Math.random() * 5),
        speed: 0.6 + Math.random() * 1.0,
        phase: Math.random() * Math.PI * 2,
        teal: Math.random() > 0.35,
      });
    }

    // Pixel burst fragments
    const BURST = [];
    let burstDone = false;

    function render() {
      const t = (Date.now() - t0.current) / 1000;
      if (t > 9.2) { cancelAnimationFrame(animRef.current); if (onComplete) onComplete(); return; }

      ctx.clearRect(0, 0, w, h);

      // Dim
      let dim = t < 0.8 ? (t / 0.8) * 0.93 : t < 8.5 ? 0.93 : 0.93 * Math.max(0, 1 - (t - 8.5) / 0.7);
      ctx.fillStyle = `rgba(4,4,10,${dim})`;
      ctx.fillRect(0, 0, w, h);

      // Card position
      let cY = cy - cardH / 2, cAlpha = 1, shX = 0, shY = 0, scl = 1;
      if (t < 0.8) { const p = t / 0.8; cY = h * 0.65 * (1 - p) + (cy - cardH / 2) * p; cAlpha = 0.3 + p * 0.7; }
      else if (t < 2.5) { cY += Math.sin(t * 3) * 2; scl = 1.02; }
      else if (t < 3.5) { const i = (t - 2.5) * 6; shX = (Math.random() - 0.5) * i; shY = (Math.random() - 0.5) * i; scl = 1.02; }
      else if (t < 4.0) { scl = 1.06; }
      else if (t < 8.5) { scl = 1.03; }
      else { cAlpha = Math.max(0, 1 - (t - 8.5) / 0.7); }

      // Pixel glow blocks orbiting inward
      if (t > 0.8 && t < 4.0) {
        const rA = t < 1.2 ? (t - 0.8) / 0.4 : t > 3.5 ? (4.0 - t) / 0.5 : 1;
        GLOWS.forEach(g => {
          const collapse = t < 2.5 ? 1.0 : Math.max(0.15, 1 - (t - 2.5) * 0.7);
          const r = g.baseR * collapse;
          const pulse = 0.4 + 0.6 * Math.sin(t * g.speed * 3 + g.phase);
          const px = cx + Math.cos(g.angle + t * 0.35) * r;
          const py = cy + Math.sin(g.angle + t * 0.35) * r * 0.55;
          const a = rA * 0.55 * pulse;
          ctx.fillStyle = g.teal ? `rgba(0,212,176,${a})` : `rgba(150,90,255,${a})`;
          ctx.fillRect(Math.floor(px), Math.floor(py), g.size, g.size);
        });
      }

      // Draw card (hidden during flash peak)
      if (!(t > 3.5 && t < 3.8) && cAlpha > 0) {
        ctx.save();
        ctx.globalAlpha = cAlpha;
        ctx.translate(cx + shX, cy + shY);

        // 360 spin 3.5-4.0
        if (t > 3.5 && t < 4.0) {
          const sp = (t - 3.5) / 0.5;
          ctx.scale(Math.cos(sp * Math.PI * 2) * scl, scl);
        } else {
          ctx.scale(scl, scl);
        }

        // Blocky glow behind card
        const gs = t < 2.5 ? 0.03 : t < 3.5 ? 0.03 + (t - 2.5) * 0.12 : 0.05;
        for (let i = 3; i >= 1; i--) {
          const ga = gs * (1 - i * 0.25);
          ctx.fillStyle = i % 2 === 0 ? `rgba(0,212,176,${ga})` : `rgba(150,90,255,${ga})`;
          ctx.fillRect(-cardW/2 - i*4, -cardH/2 - i*4, cardW + i*8, cardH + i*8);
        }

        // Card body
        ctx.fillStyle = "rgba(8,5,18,0.95)";
        ctx.fillRect(-cardW/2, -cardH/2, cardW, cardH);

        // Border
        const bA = t > 2.5 ? 0.5 + Math.sin(t * 8) * 0.3 : 0.25;
        ctx.strokeStyle = `rgba(0,212,176,${bA})`;
        ctx.lineWidth = 2;
        ctx.strokeRect(-cardW/2, -cardH/2, cardW, cardH);

        // Art area
        const artH = cardH * 0.55;
        ctx.fillStyle = "rgba(6,4,14,0.9)";
        ctx.fillRect(-cardW/2 + 10, -cardH/2 + 10, cardW - 20, artH);

        // Ghost pixel silhouette
        const gp = Math.max(2, Math.floor(cardW * 0.014));
        const gy0 = -cardH/2 + artH/2 - gp * 5;
        const gx0 = -gp * 5;
        const G = [
          "  ######  ",
          " ######## ",
          "##########",
          "##.##.####",
          "##.##.####",
          "##########",
          " ######## ",
          "# ## ## ##",
          "  #  #  # ",
        ];
        const ga = 0.12 + Math.sin(t * 2) * 0.05;
        G.forEach((row, ry) => {
          for (let rx = 0; rx < row.length; rx++) {
            if (row[rx] === '#') {
              ctx.fillStyle = `rgba(0,212,176,${ga})`;
              ctx.fillRect(gx0 + rx * gp, gy0 + ry * gp, gp, gp);
            } else if (row[rx] === '.') {
              ctx.fillStyle = `rgba(0,212,176,${ga * 2.5})`;
              ctx.fillRect(gx0 + rx * gp, gy0 + ry * gp, gp, gp);
            }
          }
        });

        // Text placeholders
        ctx.fillStyle = "rgba(0,212,176,0.15)";
        ctx.fillRect(-cardW/2 + 14, -cardH/2 + artH + 20, cardW * 0.35, 5);
        ctx.fillStyle = "rgba(255,255,255,0.06)";
        ctx.fillRect(-cardW/2 + 14, -cardH/2 + artH + 33, cardW * 0.7, 4);
        ctx.fillRect(-cardW/2 + 14, -cardH/2 + artH + 43, cardW * 0.55, 4);

        ctx.restore();
      }

      // Flash
      if (t > 3.5 && t < 4.0) {
        const fi = t < 3.65 ? (t - 3.5) / 0.15 : (4.0 - t) / 0.35;
        ctx.fillStyle = `rgba(180,255,230,${fi * 0.75})`;
        ctx.fillRect(0, 0, w, h);
      }

      // Pixel burst
      if (t > 3.55 && !burstDone) {
        burstDone = true;
        for (let i = 0; i < 50; i++) {
          const a = Math.random() * Math.PI * 2;
          const spd = 2 + Math.random() * 8;
          BURST.push({ x: cx, y: cy, vx: Math.cos(a)*spd, vy: Math.sin(a)*spd,
            size: 2 + Math.floor(Math.random() * 5), life: 1, teal: Math.random() > 0.4 });
        }
      }
      BURST.forEach(b => {
        b.x += b.vx; b.y += b.vy; b.vy += 0.03; b.vx *= 0.99; b.life -= 0.012;
        if (b.life <= 0) return;
        ctx.fillStyle = b.teal ? `rgba(0,212,176,${b.life * 0.7})` : `rgba(150,90,255,${b.life * 0.7})`;
        ctx.fillRect(Math.floor(b.x), Math.floor(b.y), b.size, b.size);
      });

      // MINTED text (4.0 - 8.5 = 4.5 second hold)
      if (t > 4.0 && t < 8.5) {
        const fIn = t < 4.4 ? (t - 4.0) / 0.4 : 1;
        const fOut = t > 8.0 ? (8.5 - t) / 0.5 : 1;
        const ta = fIn * fOut;

        ctx.fillStyle = `rgba(0,212,176,${ta * 0.05})`;
        ctx.fillRect(cx - 130, cy - 30, 260, 60);

        ctx.fillStyle = `rgba(0,212,176,${ta})`;
        ctx.font = `${Math.min(32, w * 0.08)}px "Press Start 2P", monospace`;
        ctx.textAlign = "center";
        ctx.textBaseline = "middle";
        ctx.fillText("MINTED", cx, cy - 8);

        ctx.fillStyle = `rgba(150,90,255,${ta * 0.85})`;
        ctx.font = `${Math.min(9, w * 0.023)}px "Press Start 2P", monospace`;
        ctx.fillText("SOUL PRESERVED ON-CHAIN", cx, cy + 25);

        // Scanline
        const sY = cy - 20 + ((t * 35) % 50);
        ctx.fillStyle = `rgba(0,212,176,${ta * 0.04})`;
        ctx.fillRect(cx - 160, sY, 320, 1);
      }

      // CRT
      ctx.fillStyle = "rgba(0,0,0,0.025)";
      for (let sy = 0; sy < h; sy += 3) ctx.fillRect(0, sy, w, 1);

      animRef.current = requestAnimationFrame(render);
    }

    animRef.current = requestAnimationFrame(render);
    return () => cancelAnimationFrame(animRef.current);
  }, [onComplete]);

  return (
    <div style={{ position:"fixed", inset:0, zIndex:9999 }}>
      <canvas ref={canvasRef} style={{ position:"absolute", inset:0 }} />
    </div>
  );
}
