import React, { useRef, useEffect } from "react";
import { getAnalyser } from "./useAudioPlayer";

// ============ SEEDED RNG ============
function hashToSeeds(str) {
  let h = 0;
  for (let i = 0; i < str.length; i++) h = ((h << 5) - h + str.charCodeAt(i)) | 0;
  const seeds = [];
  for (let i = 0; i < 30; i++) { h = (h * 16807 + 2147483647) | 0; seeds.push((h & 0x7fffffff) / 0x7fffffff); }
  return seeds;
}
function mulberry32(seed) {
  let t = seed + 0x6d2b79f5;
  return () => { t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
}

// ============ PALETTES ============
const PALETTES = {
  legendary: {
    bg: [8, 2, 12], energy: [150, 90, 255], energyAlt: [200, 120, 255],
    accent: [255, 180, 50], glow: [150, 90, 255],
  },
  rare: {
    bg: [2, 6, 10], energy: [0, 212, 176], energyAlt: [0, 160, 200],
    accent: [0, 255, 160], glow: [0, 212, 176],
  },
  common: {
    bg: [4, 6, 4], energy: [0, 180, 120], energyAlt: [0, 140, 100],
    accent: [0, 200, 130], glow: [0, 180, 120],
  },
};

// Skull bone colors — warm whites and greys like the references
const BONE = {
  light: [235, 225, 205],    // bright bone
  mid:   [200, 190, 170],    // mid bone
  shade: [150, 140, 125],    // shadow side
  dark:  [90, 80, 70],       // deep shadow
  black: [25, 20, 18],       // eye sockets, nose
  teeth: [220, 215, 195],    // teeth highlights
  gold:  [220, 180, 60],     // gold teeth accent
};

// ============ SKULL GENERATOR — proper pixel art skulls ============
// Base always looks like a skull. Traits make each one unique.
function generateSkull(rng, pal) {
  const W = 16, H = 18;
  // 0=empty, L=light, M=mid, S=shade, D=dark, B=black(socket), G=glow, T=teeth, X=gold
  const grid = Array.from({ length: H }, () => Array(W).fill(0));

  // Solid skull base — always recognizable
  // This is the canonical skull shape, hand-crafted to look right
  const BASE = [
    "    LLMMMMLL    ", // 0
    "   LLLLMMMMLL   ", // 1
    "  LLLLLLMMMMSS  ", // 2
    " LLLLLLLMMMMSSS ", // 3
    " LLLLLLLMMMMSSS ", // 4
    "LLLLLLLLMMMMSSSS", // 5
    "LLLLLLLLMMMMSSSS", // 6
    "LLLLBBLLLLBBSSSS", // 7 eyes
    "LLLLBBLLLLBBSSSS", // 8 eyes
    "LLLLLLLLMMMMSSSS", // 9
    " LLLLLLLMMMMSSS ", // 10
    " LLLLLLBBMMMSSS ", // 11 nose
    " LLLLLLBBMMMSSS ", // 12
    "  LLLLMMMMMSSS  ", // 13
    "  TT TT TT TS  ", // 14 teeth
    "  TT TT TT TS  ", // 15 teeth
    "   LLMMMMMSSS   ", // 16 jaw
    "    LMMMMSSS    ", // 17 chin
  ];

  const charMap = { 'L': 'L', 'M': 'M', 'S': 'S', 'D': 'D', 'B': 'B', 'G': 'G', 'T': 'T', 'X': 'X', ' ': 0 };

  for (let y = 0; y < H; y++) {
    for (let x = 0; x < W; x++) {
      const ch = (BASE[y] || "")[x] || ' ';
      grid[y][x] = charMap[ch] || 0;
    }
  }

  // ======= UNIQUE TRAITS FROM HASH =======

  // Eye style
  const eyeStyle = Math.floor(rng() * 8);
  const eyeGlow = rng() > 0.4;
  const eyePositions = [[7,4],[7,5],[8,4],[8,5],[7,10],[7,11],[8,10],[8,11]];
  eyePositions.forEach(([y,x]) => { grid[y][x] = 'B'; });

  if (eyeStyle === 1) [[6,4],[6,5],[6,10],[6,11]].forEach(([y,x]) => { grid[y][x] = 'B'; }); // tall
  else if (eyeStyle === 2) [[7,3],[8,3],[7,12],[8,12]].forEach(([y,x]) => { grid[y][x] = 'B'; }); // wide
  else if (eyeStyle === 3) { grid[7][5] = 'S'; grid[7][10] = 'S'; } // angry
  else if (eyeStyle === 4) { grid[7][4] = 'L'; grid[7][11] = 'M'; grid[8][5] = 'L'; grid[8][10] = 'M'; } // dots
  else if (eyeStyle === 5) { grid[7][4] = 'L'; grid[7][11] = 'M'; grid[8][4] = 'L'; grid[8][11] = 'M'; } // diamond
  else if (eyeStyle === 6) [[6,4],[6,5],[6,6],[7,6],[8,6],[6,9],[6,10],[6,11],[7,9],[8,9]].forEach(([y,x]) => { grid[y][x] = 'B'; }); // big
  else if (eyeStyle === 7) { grid[7][4]='L';grid[7][5]='L';grid[7][10]='M';grid[7][11]='M'; } // slit

  if (eyeGlow) { for (let y=6;y<=9;y++) for (let x=0;x<W;x++) if (grid[y][x]==='B') grid[y][x]='G'; }

  // Nose
  const noseStyle = Math.floor(rng() * 4);
  if (noseStyle===1) { grid[11][6]='B';grid[11][7]='B';grid[11][8]='B';grid[12][6]='B';grid[12][7]='B';grid[12][8]='B'; }
  else if (noseStyle===2) { grid[12][7]='M';grid[12][8]='M'; }
  else if (noseStyle===3) { grid[11][7]='B';grid[12][7]='B';grid[12][8]='B'; }

  // Teeth
  const teethStyle = Math.floor(rng() * 6);
  if (teethStyle===1) { const mt=2+Math.floor(rng()*4)*3;if(mt<W){grid[14][mt]=0;grid[14][mt+1]=0;grid[15][mt]=0;grid[15][mt+1]=0;} }
  else if (teethStyle===2) { const gx=2+Math.floor(rng()*4)*3;if(gx+1<W){grid[14][gx]='X';grid[14][gx+1]='X';grid[15][gx]='X';grid[15][gx+1]='X';} }
  else if (teethStyle===3) { grid[16][3]='T';grid[16][12]='T'; }
  else if (teethStyle===4) { for(let x=2;x<14;x++){grid[14][x]=grid[14][x]==='T'?'B':grid[14][x];grid[15][x]=grid[15][x]==='T'?'B':grid[15][x];} }
  else if (teethStyle===5) { grid[16][4]='T';grid[16][5]='T';grid[16][7]='T';grid[16][8]='T';grid[16][10]='T';grid[16][11]='T'; }

  // Scar
  if (rng() > 0.6) { const sx=2+Math.floor(rng()*5); for(let i=0;i<4;i++){const sy=2+i,ssx=sx+(rng()>0.5?i:0);if(sy<H&&ssx<W&&grid[sy][ssx]!==0)grid[sy][ssx]='D';} }

  // Crack
  if (rng() > 0.6) { let ccx=6+Math.floor(rng()*4),ccy=0; for(let i=0;i<5;i++){if(ccy<H&&ccx>=0&&ccx<W&&grid[ccy][ccx]!==0)grid[ccy][ccx]='D';ccy++;ccx+=Math.floor(rng()*3)-1;} }

  // Eyepatch
  const hasEyepatch = rng() > 0.88;
  if (hasEyepatch) { for(let y=6;y<=9;y++) for(let x=3;x<=6;x++) if(y<H&&x<W) grid[y][x]='D'; }

  // ======= OVERLAYS (drawn on top during render) =======
  // These are pixel art accessories stored as {pixels: [{x,y,color}], name}
  const overlays = [];

  // Color helpers
  const BLK = [20,18,15], WHT = [240,235,225], RED = [200,40,30], BLU = [40,80,200];
  const GRN = [30,160,80], YEL = [220,200,50], ORG = [220,130,30], PNK = [220,100,160];
  const BRN = [120,80,40], DBRN = [80,50,25], GLD = [220,180,60], SIL = [170,175,185];
  const TEAL = [0,212,176], PURP = [150,90,255], GRAY = [120,120,120], LGRY = [180,180,180];
  const CYN = [0,200,220], MAG = [200,50,200];

  // === HATS (pick 0-1) ===
  const hatRoll = rng();
  const hatType = Math.floor(rng() * 30);

  if (hatRoll > 0.3) { // 70% chance of hat
    const px = (x,y,c) => overlays.push({x,y,c});

    if (hatType === 0) { // Cowboy hat
      for(let x=0;x<16;x++) px(x,-2,BRN);
      for(let x=1;x<15;x++) px(x,-3,BRN);
      for(let x=3;x<13;x++) px(x,-4,BRN);
      for(let x=4;x<12;x++) px(x,-5,DBRN);
      for(let x=5;x<11;x++) px(x,-6,BRN);
      for(let x=5;x<11;x++) px(x,-3,DBRN); // band
    } else if (hatType === 1) { // Top hat
      for(let x=3;x<13;x++) for(let y=-8;y<-1;y++) px(x,y,BLK);
      for(let x=2;x<14;x++) px(x,-2,BLK);
      for(let x=4;x<12;x++) px(x,-3,DBRN); // band
    } else if (hatType === 2) { // Beanie
      for(let x=3;x<13;x++) px(x,-2,RED);
      for(let x=4;x<12;x++) px(x,-3,RED);
      for(let x=5;x<11;x++) px(x,-4,RED);
      for(let x=6;x<10;x++) px(x,-5,RED);
      px(8,-6,RED); // pom
    } else if (hatType === 3) { // Baseball cap
      for(let x=2;x<14;x++) px(x,-2,BLU);
      for(let x=3;x<13;x++) px(x,-3,BLU);
      for(let x=4;x<12;x++) px(x,-4,BLU);
      for(let x=0;x<8;x++) px(x,-1,BLU); // brim
    } else if (hatType === 4) { // Crown
      for(let x=3;x<13;x++) px(x,-2,GLD);
      px(4,-3,GLD);px(6,-4,GLD);px(8,-5,GLD);px(10,-4,GLD);px(12,-3,GLD);
      px(6,-3,GLD);px(8,-4,GLD);px(8,-3,GLD);px(10,-3,GLD);
      px(8,-4,RED); // jewel
    } else if (hatType === 5) { // Pirate hat
      for(let x=2;x<14;x++) px(x,-2,BLK);
      for(let x=3;x<13;x++) px(x,-3,BLK);
      for(let x=1;x<5;x++) px(x,-4,BLK);
      for(let x=11;x<15;x++) px(x,-4,BLK);
      for(let x=5;x<11;x++) px(x,-5,BLK);
      px(7,-4,WHT);px(8,-4,WHT); // skull on hat
    } else if (hatType === 6) { // Sailor hat
      for(let x=3;x<13;x++) px(x,-2,WHT);
      for(let x=4;x<12;x++) px(x,-3,WHT);
      for(let x=5;x<11;x++) px(x,-4,WHT);
      for(let x=3;x<13;x++) px(x,-2,BLU); // brim stripe
    } else if (hatType === 7) { // Trucker cap
      for(let x=2;x<14;x++) px(x,-2,ORG);
      for(let x=3;x<13;x++) px(x,-3,ORG);
      for(let x=4;x<12;x++) px(x,-4,WHT); // mesh back
      for(let x=0;x<8;x++) px(x,-1,ORG);
    } else if (hatType === 8) { // Fedora
      for(let x=1;x<15;x++) px(x,-2,GRAY);
      for(let x=3;x<13;x++) px(x,-3,GRAY);
      for(let x=4;x<12;x++) px(x,-4,GRAY);
      for(let x=4;x<12;x++) px(x,-5,GRAY);
      for(let x=3;x<13;x++) px(x,-3,BLK); // band
    } else if (hatType === 9) { // Wizard hat
      for(let x=3;x<13;x++) px(x,-2,PURP);
      for(let x=4;x<12;x++) px(x,-3,PURP);
      for(let x=5;x<11;x++) px(x,-4,PURP);
      for(let x=6;x<10;x++) px(x,-5,PURP);
      for(let x=7;x<9;x++) px(x,-6,PURP);
      px(7,-7,PURP);px(8,-8,PURP);
      px(7,-5,GLD); // star
    } else if (hatType === 10) { // Headband
      for(let x=1;x<15;x++) px(x,4,RED);
      px(0,5,RED);px(0,6,RED); // tail
    } else if (hatType === 11) { // Mohawk
      for(let y=-6;y<0;y++) { px(7,y,GRN); px(8,y,GRN); }
    } else if (hatType === 12) { // Viking helmet
      for(let x=2;x<14;x++) px(x,-2,SIL);
      for(let x=3;x<13;x++) px(x,-3,SIL);
      for(let x=4;x<12;x++) px(x,-4,SIL);
      px(1,-3,WHT);px(0,-4,WHT);px(-1,-5,WHT); // left horn
      px(14,-3,WHT);px(15,-4,WHT);px(16,-5,WHT); // right horn
    } else if (hatType === 13) { // Chef hat
      for(let x=3;x<13;x++) px(x,-2,WHT);
      for(let x=3;x<13;x++) for(let y=-6;y<-2;y++) px(x,y,WHT);
    } else if (hatType === 14) { // Bandana
      for(let x=1;x<15;x++) px(x,-1,RED);
      for(let x=2;x<14;x++) px(x,-2,RED);
      px(14,0,RED);px(15,1,RED); // knot
    } else if (hatType === 15) { // Halo
      for(let x=4;x<12;x++) px(x,-4,GLD);
      px(3,-3,GLD);px(12,-3,GLD);
    } else if (hatType === 16) { // Bucket hat
      for(let x=1;x<15;x++) px(x,-2,GRN);
      for(let x=3;x<13;x++) px(x,-3,GRN);
      for(let x=4;x<12;x++) px(x,-4,GRN);
    } else if (hatType === 17) { // Santa hat
      for(let x=3;x<13;x++) px(x,-2,RED);
      for(let x=4;x<12;x++) px(x,-3,RED);
      for(let x=5;x<11;x++) px(x,-4,RED);
      for(let x=10;x<13;x++) px(x,-5,RED);
      px(13,-5,WHT); // pom
      for(let x=3;x<13;x++) px(x,-2,WHT); // brim
    } else if (hatType === 18) { // Afro
      for(let x=1;x<15;x++) for(let y=-5;y<1;y++) px(x,y,BLK);
    } else if (hatType === 19) { // Devil horns
      px(2,-2,RED);px(1,-3,RED);px(0,-4,RED);
      px(13,-2,RED);px(14,-3,RED);px(15,-4,RED);
    } else if (hatType === 20) { // Army helmet
      for(let x=2;x<14;x++) px(x,-2,GRN);
      for(let x=3;x<13;x++) px(x,-3,GRN);
      for(let x=4;x<12;x++) px(x,-4,GRN);
      for(let x=5;x<11;x++) px(x,-5,GRN);
    } else if (hatType === 21) { // Sombrero
      for(let x=-1;x<17;x++) px(x,-2,YEL);
      for(let x=3;x<13;x++) px(x,-3,ORG);
      for(let x=4;x<12;x++) px(x,-4,YEL);
      for(let x=5;x<11;x++) px(x,-5,ORG);
    } else if (hatType === 22) { // Backwards cap
      for(let x=2;x<14;x++) px(x,-2,RED);
      for(let x=3;x<13;x++) px(x,-3,RED);
      for(let x=9;x<16;x++) px(x,-1,RED); // backwards brim
    } else if (hatType === 23) { // Durag
      for(let x=2;x<14;x++) px(x,-1,BLU);
      for(let x=3;x<13;x++) px(x,-2,BLU);
      px(14,0,BLU);px(15,1,BLU);px(15,2,BLU); // tail
    } else if (hatType === 24) { // Bowler hat
      for(let x=2;x<14;x++) px(x,-2,BLK);
      for(let x=4;x<12;x++) for(let y=-5;y<-2;y++) px(x,y,BLK);
    } else if (hatType === 25) { // Straw hat
      for(let x=0;x<16;x++) px(x,-2,YEL);
      for(let x=3;x<13;x++) px(x,-3,YEL);
      for(let x=4;x<12;x++) px(x,-4,YEL);
      for(let x=3;x<13;x++) px(x,-3,BRN); // band
    } else if (hatType === 26) { // Space helmet
      for(let x=1;x<15;x++) px(x,-2,LGRY);
      for(let x=1;x<15;x++) px(x,-3,LGRY);
      for(let x=2;x<14;x++) px(x,-4,LGRY);
      for(let x=3;x<13;x++) px(x,-5,LGRY);
      for(let x=2;x<14;x++) px(x,-2,CYN); // visor line
    } else if (hatType === 27) { // Fire
      px(6,-2,ORG);px(7,-3,RED);px(8,-4,YEL);px(9,-3,ORG);px(7,-5,ORG);px(8,-6,RED);px(5,-3,RED);px(10,-2,YEL);
    } else if (hatType === 28) { // Propeller hat
      for(let x=3;x<13;x++) px(x,-2,BLU);
      for(let x=4;x<12;x++) px(x,-3,BLU);
      px(7,-4,RED);px(8,-4,RED); // propeller center
      px(5,-5,RED);px(6,-4,RED);px(9,-4,RED);px(10,-5,RED);
    } else if (hatType === 29) { // Toque/winter hat
      for(let x=3;x<13;x++) px(x,-2,TEAL);
      for(let x=4;x<12;x++) px(x,-3,WHT);
      for(let x=4;x<12;x++) px(x,-4,TEAL);
      for(let x=5;x<11;x++) px(x,-5,TEAL);
      for(let x=6;x<10;x++) px(x,-6,TEAL);
    }
  }

  // === EYEWEAR (pick 0-1) ===
  const glassesRoll = rng();
  const glassesType = Math.floor(rng() * 12);

  if (glassesRoll > 0.45) { // 55% chance
    const px = (x,y,c) => overlays.push({x,y,c});

    if (glassesType === 0) { // Pit vipers
      for(let x=2;x<7;x++) px(x,7,CYN); for(let x=9;x<14;x++) px(x,7,MAG);
      for(let x=2;x<7;x++) px(x,8,CYN); for(let x=9;x<14;x++) px(x,8,MAG);
      for(let x=7;x<9;x++) px(x,7,BLK); // bridge
    } else if (glassesType === 1) { // Aviators
      for(let x=2;x<7;x++) px(x,7,GLD); for(let x=9;x<14;x++) px(x,7,GLD);
      for(let x=3;x<6;x++) px(x,8,[60,50,40]); for(let x=10;x<13;x++) px(x,8,[60,50,40]);
      for(let x=7;x<9;x++) px(x,7,GLD);
    } else if (glassesType === 2) { // 3D glasses
      for(let x=2;x<7;x++) px(x,7,RED); for(let x=9;x<14;x++) px(x,7,CYN);
      for(let x=2;x<7;x++) px(x,8,RED); for(let x=9;x<14;x++) px(x,8,CYN);
      for(let x=7;x<9;x++) px(x,7,BLK);
    } else if (glassesType === 3) { // Heart glasses
      px(3,7,PNK);px(4,6,PNK);px(5,7,PNK);px(4,8,PNK);
      px(10,7,PNK);px(11,6,PNK);px(12,7,PNK);px(11,8,PNK);
      for(let x=6;x<10;x++) px(x,7,PNK);
    } else if (glassesType === 4) { // Nerd glasses
      for(let x=2;x<7;x++) { px(x,6,BLK);px(x,9,BLK); }
      for(let x=9;x<14;x++) { px(x,6,BLK);px(x,9,BLK); }
      px(2,7,BLK);px(2,8,BLK);px(6,7,BLK);px(6,8,BLK);
      px(9,7,BLK);px(9,8,BLK);px(13,7,BLK);px(13,8,BLK);
      for(let x=7;x<9;x++) px(x,7,BLK);
    } else if (glassesType === 5) { // Monocle
      px(9,6,GLD);px(13,6,GLD);px(9,9,GLD);px(13,9,GLD);
      for(let x=10;x<13;x++) { px(x,6,GLD);px(x,9,GLD); }
      px(13,10,GLD);px(13,11,GLD); // chain
    } else if (glassesType === 6) { // Cyclops visor
      for(let x=1;x<15;x++) px(x,7,RED);
      for(let x=1;x<15;x++) px(x,8,[150,30,20]);
    } else if (glassesType === 7) { // Thug life
      for(let x=2;x<7;x++) px(x,8,BLK); for(let x=9;x<14;x++) px(x,8,BLK);
      for(let x=2;x<7;x++) px(x,7,BLK); for(let x=9;x<14;x++) px(x,7,BLK);
      for(let x=7;x<9;x++) px(x,8,BLK);
    } else if (glassesType === 8) { // Star glasses
      px(4,7,GLD);px(3,7,GLD);px(5,7,GLD);px(4,6,GLD);px(4,8,GLD);
      px(11,7,GLD);px(10,7,GLD);px(12,7,GLD);px(11,6,GLD);px(11,8,GLD);
      for(let x=6;x<10;x++) px(x,7,GLD);
    } else if (glassesType === 9) { // VR headset
      for(let x=1;x<15;x++) for(let y=6;y<=9;y++) px(x,y,BLK);
      for(let x=3;x<6;x++) px(x,7,CYN); for(let x=10;x<13;x++) px(x,7,CYN);
    } else if (glassesType === 10) { // Laser eyes (no frame, just beams drawn in render)
      // Handled in render with glow
    } else if (glassesType === 11) { // Round lennon glasses
      px(3,6,GLD);px(6,6,GLD);px(3,9,GLD);px(6,9,GLD);
      px(10,6,GLD);px(13,6,GLD);px(10,9,GLD);px(13,9,GLD);
      for(let x=7;x<10;x++) px(x,7,GLD);
    }
  }

  // === MOUTH ITEMS ===
  const mouthRoll = rng();
  const mouthType = Math.floor(rng() * 8);

  if (mouthRoll > 0.6) { // 40% chance
    const px = (x,y,c) => overlays.push({x,y,c});

    if (mouthType === 0) { // Cigarette
      for(let x=12;x<16;x++) px(x,14,WHT);
      px(16,14,ORG); // lit end
      px(16,13,[180,180,180]); // smoke
      px(17,12,[150,150,150]);
    } else if (mouthType === 1) { // Pipe
      px(13,15,BRN);px(14,15,BRN);px(14,14,BRN);px(15,14,BRN);
      px(15,13,BRN);px(15,12,BRN);px(14,12,BRN); // pipe bowl
      px(14,11,[180,180,180]); // smoke
    } else if (mouthType === 2) { // Cigar
      for(let x=12;x<17;x++) px(x,14,BRN);
      px(17,14,ORG); px(17,13,[180,180,180]);
    } else if (mouthType === 3) { // Rose
      px(13,14,GRN);px(14,14,GRN);px(15,14,RED);px(15,13,RED);px(14,13,RED);
    } else if (mouthType === 4) { // Lollipop
      px(13,14,WHT);px(14,14,WHT);px(15,13,PNK);px(15,14,PNK);px(14,13,PNK);
    } else if (mouthType === 5) { // Fangs dripping
      px(4,16,[200,0,0]);px(11,16,[200,0,0]);
    } else if (mouthType === 6) { // Bubble gum
      px(8,16,PNK);px(9,16,PNK);px(8,17,PNK);px(9,17,PNK);
    } else if (mouthType === 7) { // Tongue out
      px(7,16,[200,80,80]);px(8,16,[200,80,80]);px(8,17,[200,80,80]);
    }
  }

  // === NECK/BODY ITEMS ===
  const neckRoll = rng();
  const neckType = Math.floor(rng() * 6);

  if (neckRoll > 0.5) { // 50% chance
    const px = (x,y,c) => overlays.push({x,y,c});

    if (neckType === 0) { // Gold chain
      for(let x=3;x<13;x++) px(x,17,GLD);
      px(7,18,GLD);px(8,18,GLD);px(7,19,GLD);px(8,19,GLD); // pendant
    } else if (neckType === 1) { // Silver chain
      for(let x=3;x<13;x++) px(x,17,SIL);
      px(8,18,SIL); // pendant
    } else if (neckType === 2) { // Bowtie
      px(6,17,RED);px(7,17,BLK);px(8,17,BLK);px(9,17,RED);
      px(5,17,RED);px(10,17,RED);
    } else if (neckType === 3) { // Bandana neck
      for(let x=4;x<12;x++) px(x,16,RED);
      for(let x=5;x<11;x++) px(x,17,RED);
    } else if (neckType === 4) { // Hoodie
      for(let x=0;x<4;x++) for(let y=8;y<18;y++) px(x,y,GRAY);
      for(let x=12;x<16;x++) for(let y=8;y<18;y++) px(x,y,GRAY);
      for(let x=4;x<12;x++) for(let y=16;y<20;y++) px(x,y,GRAY);
      // Hood behind head
      for(let x=1;x<15;x++) for(let y=-2;y<3;y++) px(x,y,[100,100,100]);
    } else if (neckType === 5) { // Pearl necklace
      for(let x=3;x<13;x+=2) px(x,17,WHT);
    }
  }

  // Store glasses type for render (laser eyes need special treatment)
  const hasLaserEyes = glassesRoll > 0.45 && glassesType === 10;

  return { grid, W, H, overlays, eyeGlow, hasLaserEyes };
}

// ============ SOUL SIGNATURE ============
export default function SoulSignature({ txHash = "default", rarity = "legendary", type = "swap", width = 420, height = 560, isPlaying = false }) {
  const canvasRef = useRef(null);
  const animRef = useRef(null);
  const audioRef = useRef({ bass:0, mid:0, high:0, bassHit:0, lastBass:0, bassThreshold:0.4, ringQueue:[] });

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    const dpr = Math.min(2, window.devicePixelRatio || 1);
    canvas.width = width * dpr;
    canvas.height = height * dpr;
    ctx.scale(dpr, dpr);

    const seeds = hashToSeeds(txHash);
    const rng = mulberry32(Math.floor(seeds[0] * 999999));
    const pal = PALETTES[rarity] || PALETTES.common;
    const skull = generateSkull(rng, pal);

    // Pixel energy blocks orbiting
    const BLOCKS = [];
    const blockCount = 20 + Math.floor(seeds[1] * 15);
    for (let i = 0; i < blockCount; i++) {
      BLOCKS.push({
        angle: rng() * Math.PI * 2,
        radius: 40 + rng() * 100,
        speed: (rng() - 0.5) * 0.015,
        size: 2 + Math.floor(rng() * 4),
        alpha: 0.1 + rng() * 0.35,
        pulse: rng() * Math.PI * 2,
        useAlt: rng() > 0.5,
      });
    }

    // Portal ring count
    const portalRings = 5 + Math.floor(seeds[2] * 5);
    const portalSpeed = seeds[3] * 0.2 + 0.08;

    // Soul pixels floating up
    const SOULS = [];
    for (let i = 0; i < 6 + Math.floor(seeds[4] * 6); i++) {
      SOULS.push({
        x: (rng() - 0.5) * width * 0.6,
        y: rng() * height * 0.4,
        speed: 0.2 + rng() * 0.5,
        size: 2 + Math.floor(rng() * 3),
        alpha: 0.1 + rng() * 0.25,
        wobble: rng() * Math.PI * 2,
      });
    }

    const freqData = new Uint8Array(128);

    function sampleAudio() {
      const ad = audioRef.current;
      const analyser = getAnalyser();
      if (analyser && isPlaying) {
        analyser.getByteFrequencyData(freqData);
        let bs = 0; for (let i = 0; i < 7; i++) bs += freqData[i]; const br = bs / (7 * 255);
        let ms = 0; for (let i = 7; i < 31; i++) ms += freqData[i]; const mr = ms / (24 * 255);
        let hs = 0; for (let i = 31; i < 80; i++) hs += freqData[i]; const hr = hs / (49 * 255);
        ad.bass = ad.bass * 0.3 + br * 0.7; ad.mid = ad.mid * 0.4 + mr * 0.6; ad.high = ad.high * 0.5 + hr * 0.5;
        if (br - ad.lastBass > 0.08 && br > ad.bassThreshold * 0.7) { ad.bassHit = 1.0; ad.ringQueue.push({ birth: performance.now(), i: br }); if (ad.ringQueue.length > 5) ad.ringQueue.shift(); }
        ad.lastBass = br; ad.bassHit *= 0.82; ad.bassThreshold = ad.bassThreshold * 0.995 + br * 0.005;
      } else { ad.bass *= 0.95; ad.mid *= 0.95; ad.high *= 0.95; ad.bassHit *= 0.92; }
    }

    let time = 0;
    function render() {
      time += 0.016;
      sampleAudio();
      const ad = audioRef.current;
      const cx = width / 2, cy = height / 2 - 15;
      const bR = ad.bass * 0.8;
      const [bgr, bgg, bgb] = pal.bg;
      const [er, eg, eb] = pal.energy;
      const [ar, ag, ab] = pal.energyAlt;

      // Background
      ctx.fillStyle = `rgb(${bgr},${bgg},${bgb})`;
      ctx.fillRect(0, 0, width, height);

      // === PIXEL PORTAL (rotating concentric rectangles) ===
      for (let ring = portalRings; ring >= 0; ring--) {
        const baseS = 30 + ring * 22 + bR * 10;
        const rot = time * portalSpeed * (ring % 2 === 0 ? 1 : -1) * 0.5;
        const a = (0.03 + (portalRings - ring) * 0.006) * (1 + bR * 1.5);
        ctx.save();
        ctx.translate(cx, cy);
        ctx.rotate(rot);
        ctx.strokeStyle = ring % 2 === 0 ? `rgba(${er},${eg},${eb},${a})` : `rgba(${ar},${ag},${ab},${a})`;
        ctx.lineWidth = 1;
        ctx.strokeRect(-baseS/2, -baseS/2 * 0.7, baseS, baseS * 0.7);
        ctx.restore();
      }

      // Center glow (blocky)
      const glowS = 50 + bR * 20 + Math.sin(time * 1.5) * 5;
      for (let i = 3; i >= 0; i--) {
        const gs = glowS + i * 15;
        const ga = (0.02 + bR * 0.03) * (1 - i * 0.2);
        ctx.fillStyle = `rgba(${er},${eg},${eb},${ga})`;
        ctx.fillRect(cx - gs/2, cy - gs/2 * 0.7, gs, gs * 0.7);
      }

      // === PIXEL ENERGY BLOCKS ===
      BLOCKS.forEach(b => {
        b.angle += b.speed * (1 + ad.high * 2);
        b.pulse += 0.02;
        const r = b.radius + bR * 20 + Math.sin(b.pulse) * 8;
        const bx = cx + Math.cos(b.angle) * r;
        const by = cy + Math.sin(b.angle) * r * 0.55;
        const [cr, cg, cb] = b.useAlt ? pal.energyAlt : pal.energy;
        const ba = b.alpha * (0.4 + 0.6 * Math.sin(time * 1.5 + b.pulse)) * (1 + bR);
        ctx.fillStyle = `rgba(${cr},${cg},${cb},${ba})`;
        ctx.fillRect(Math.floor(bx), Math.floor(by), b.size, b.size);
      });

      // === PIXEL SKULL (centerpiece — NO background square) ===
      const skullScale = Math.min(width, height) * 0.024 * (1 + bR * 0.08);
      const skullW = skull.W * skullScale;
      const skullH = skull.H * skullScale;
      const skullX = cx - skullW / 2;
      const skullY = cy - skullH / 2 + 5;

      // Subtle glow behind skull (small, not a square)
      for (let i = 2; i >= 0; i--) {
        const gs = i * 4;
        const ga = 0.02 + ad.bassHit * 0.04;
        ctx.fillStyle = `rgba(${er},${eg},${eb},${ga})`;
        // Only glow where skull pixels exist (approximate with smaller rect)
        ctx.fillRect(skullX + skullScale * 2 - gs, skullY + skullScale - gs, skullW - skullScale * 4 + gs*2, skullH - skullScale * 2 + gs*2);
      }

      // Draw skull pixels
      const glowPulse = 0.6 + Math.sin(time * 3) * 0.4;
      for (let y = 0; y < skull.H; y++) {
        for (let x = 0; x < skull.W; x++) {
          const v = skull.grid[y][x];
          if (v === 0) continue;

          let color;
          if (v === 'L') color = BONE.light;
          else if (v === 'M') color = BONE.mid;
          else if (v === 'S') color = BONE.shade;
          else if (v === 'D') color = BONE.dark;
          else if (v === 'B') color = BONE.black;
          else if (v === 'T') color = BONE.teeth;
          else if (v === 'X') color = BONE.gold;
          else if (v === 'G') {
            // Glowing — uses energy color, pulsing
            color = [
              Math.floor(er * glowPulse + 60 * (1 - glowPulse)),
              Math.floor(eg * glowPulse + 60 * (1 - glowPulse)),
              Math.floor(eb * glowPulse + 60 * (1 - glowPulse)),
            ];
          }
          else continue;

          const [cr, cg, cb] = color;
          ctx.fillStyle = `rgb(${cr},${cg},${cb})`;
          ctx.fillRect(
            Math.floor(skullX + x * skullScale),
            Math.floor(skullY + y * skullScale),
            Math.ceil(skullScale) + 1,
            Math.ceil(skullScale) + 1,
          );
        }
      }

      // === OVERLAYS (accessories drawn on top of skull) ===
      skull.overlays.forEach(o => {
        const ox = Math.floor(skullX + o.x * skullScale);
        const oy = Math.floor(skullY + o.y * skullScale);
        ctx.fillStyle = `rgb(${o.c[0]},${o.c[1]},${o.c[2]})`;
        ctx.fillRect(ox, oy, Math.ceil(skullScale) + 1, Math.ceil(skullScale) + 1);
      });

      // Laser eyes effect
      if (skull.hasLaserEyes) {
        const laserA = 0.4 + Math.sin(time * 5) * 0.3;
        ctx.fillStyle = `rgba(${er},${eg},${eb},${laserA})`;
        for (let i = 0; i < 20; i++) {
          ctx.fillRect(skullX + 4*skullScale - i*skullScale*0.5, skullY + 7*skullScale + i*skullScale*0.3, skullScale*2, skullScale);
          ctx.fillRect(skullX + 10*skullScale + i*skullScale*0.5, skullY + 7*skullScale + i*skullScale*0.3, skullScale*2, skullScale);
        }
      }

      // === SOUL PIXELS (float upward) ===
      SOULS.forEach(s => {
        s.y -= s.speed * (1 + bR * 2);
        s.wobble += 0.02;
        if (s.y < -20) s.y = height * 0.35 + Math.random() * 20;
        const sx = cx + s.x + Math.sin(s.wobble) * 10;
        const sy = cy - 30 - s.y;
        const sa = s.alpha * (0.5 + Math.sin(time + s.wobble) * 0.5);
        ctx.fillStyle = `rgba(${er},${eg},${eb},${sa * 0.3})`;
        ctx.fillRect(Math.floor(sx) - s.size, Math.floor(sy) - s.size, s.size * 3, s.size * 3);
        ctx.fillStyle = `rgba(255,255,255,${sa * 0.6})`;
        ctx.fillRect(Math.floor(sx), Math.floor(sy), s.size, s.size);
      });

      // === BASS HIT RINGS ===
      const now = performance.now();
      audioRef.current.ringQueue = audioRef.current.ringQueue.filter(ring => {
        const age = (now - ring.birth) / 1000;
        if (age > 2) return false;
        const r = age * 150 * ring.i;
        const a = (1 - age / 2) * 0.2 * ring.i;
        ctx.strokeStyle = `rgba(${er},${eg},${eb},${a})`;
        ctx.lineWidth = 1;
        ctx.strokeRect(cx - r, cy - r * 0.6, r * 2, r * 1.2);
        return true;
      });

      // === SCANLINES ===
      ctx.fillStyle = "rgba(0,0,0,0.025)";
      for (let sy = 0; sy < height; sy += 3) ctx.fillRect(0, sy, width, 1);

      animRef.current = requestAnimationFrame(render);
    }

    animRef.current = requestAnimationFrame(render);
    return () => cancelAnimationFrame(animRef.current);
  }, [txHash, rarity, type, width, height, isPlaying]);

  return (
    <canvas ref={canvasRef}
      style={{ width: "100%", height: "auto", display: "block", imageRendering: "pixelated", aspectRatio: `${width}/${height}` }}
    />
  );
}
