export function shortenAddress(addr) {
  if (!addr) return "";
  return addr.slice(0, 4) + "..." + addr.slice(-4);
}

export function detectWallets() {
  const wallets = [];
  if (typeof window === "undefined") return wallets;

  // Phantom
  const phantom = window.phantom?.solana || (window.solana?.isPhantom ? window.solana : null);
  if (phantom) {
    wallets.push({ name: "Phantom", icon: "https://phantom.app/img/phantom-icon-purple.svg", provider: phantom });
  }

  // Solflare - check multiple injection points
  const solflare = window.solflare || window.SolflareApp;
  if (solflare) {
    wallets.push({ name: "Solflare", icon: "https://solflare.com/favicon.ico", provider: solflare });
  }

  // Backpack
  if (window.backpack) {
    wallets.push({ name: "Backpack", icon: "https://backpack.app/favicon.ico", provider: window.backpack });
  }

  // Glow
  if (window.glowSolana) {
    wallets.push({ name: "Glow", provider: window.glowSolana });
  }

  // Brave
  if (window.braveSolana) {
    wallets.push({ name: "Brave", provider: window.braveSolana });
  }

  return wallets;
}

export function connectWallet(provider) {
  return new Promise((resolve) => {
    let resolved = false;
    
    const finish = (addr) => {
      if (resolved) return;
      resolved = true;
      console.log("Wallet connected:", addr);
      resolve(addr);
    };

    // Listen for connect event (Solflare uses this)
    const onConnect = () => {
      const pk = provider.publicKey;
      if (pk) finish(typeof pk === "string" ? pk : pk.toString());
    };
    
    if (provider.on) provider.on("connect", onConnect);

    // Check if already connected
    if (provider.isConnected && provider.publicKey) {
      finish(provider.publicKey.toString());
      return;
    }

    // Call connect
    try {
      const result = provider.connect();
      if (result && result.then) {
        result.then((resp) => {
          // Phantom returns { publicKey }
          if (resp?.publicKey) {
            finish(typeof resp.publicKey === "string" ? resp.publicKey : resp.publicKey.toString());
          }
          // Check provider after connect resolves
          else if (provider.publicKey) {
            finish(provider.publicKey.toString());
          }
        }).catch((err) => {
          console.warn("Connect promise rejected:", err);
          // Still check provider - might have connected via event
          if (provider.publicKey) {
            finish(provider.publicKey.toString());
          }
        });
      }
    } catch (e) {
      console.warn("Connect threw:", e);
      if (provider.publicKey) {
        finish(provider.publicKey.toString());
      }
    }

    // Timeout fallback - check provider after 3s
    setTimeout(() => {
      if (!resolved && provider.publicKey) {
        finish(provider.publicKey.toString());
      } else if (!resolved) {
        console.warn("Connect timed out");
        resolve(null);
      }
    }, 3000);
  });
}
