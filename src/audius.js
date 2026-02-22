const AUDIUS_HOST = "https://api.audius.co";
const APP = "CRYPT";

export async function searchTracks(query, limit = 5) {
  try {
    const res = await fetch(
      `${AUDIUS_HOST}/v1/tracks/search?query=${encodeURIComponent(query)}&app_name=${APP}`
    );
    const json = await res.json();
    return (json.data || []).slice(0, limit).map(t => ({
      id: t.id,
      title: t.title,
      artist: t.user.name,
      artwork: t.artwork?.["150x150"] || null,
      artworkLg: t.artwork?.["480x480"] || t.artwork?.["150x150"] || null,
      duration: t.duration,
      plays: t.play_count || 0,
      streamUrl: `${AUDIUS_HOST}/v1/tracks/${t.id}/stream?app_name=${APP}`,
    }));
  } catch (e) {
    console.warn("Audius search failed:", e);
    return [];
  }
}

// Search with fallback queries - tries each until one returns results
export async function searchWithFallback(queries) {
  for (const q of queries) {
    const results = await searchTracks(q, 3);
    if (results.length > 0) {
      return results[Math.floor(Math.random() * results.length)];
    }
  }
  return null;
}

export async function getTrending(limit = 10) {
  try {
    const res = await fetch(
      `${AUDIUS_HOST}/v1/tracks/trending?app_name=${APP}`
    );
    const json = await res.json();
    return (json.data || []).slice(0, limit).map(t => ({
      id: t.id,
      title: t.title,
      artist: t.user.name,
      artwork: t.artwork?.["150x150"] || null,
      artworkLg: t.artwork?.["480x480"] || t.artwork?.["150x150"] || null,
      duration: t.duration,
      plays: t.play_count || 0,
      streamUrl: `${AUDIUS_HOST}/v1/tracks/${t.id}/stream?app_name=${APP}`,
    }));
  } catch (e) {
    console.warn("Audius trending failed:", e);
    return [];
  }
}

export function formatDuration(secs) {
  if (!secs) return "0:00";
  const m = Math.floor(secs / 60);
  const s = Math.floor(secs % 60);
  return `${m}:${s < 10 ? "0" : ""}${s}`;
}
