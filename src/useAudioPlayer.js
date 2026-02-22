import { useState, useRef, useEffect, useCallback } from "react";

let globalAudio = null;
let audioCtx = null;
let analyser = null;
let sourceNode = null;
let connected = false;

export function getAnalyser() {
  return analyser;
}

function ensureAudioContext() {
  if (!globalAudio) {
    globalAudio = new Audio();
    globalAudio.crossOrigin = "anonymous";
  }
  if (!audioCtx) {
    audioCtx = new (window.AudioContext || window.webkitAudioContext)();
    analyser = audioCtx.createAnalyser();
    analyser.fftSize = 256;
    analyser.smoothingTimeConstant = 0.7;
    analyser.connect(audioCtx.destination);
  }
  if (!connected && globalAudio) {
    try {
      sourceNode = audioCtx.createMediaElementSource(globalAudio);
      sourceNode.connect(analyser);
      connected = true;
    } catch (e) {
      // Already connected
    }
  }
}

export function useAudioPlayer() {
  const [currentTrack, setCurrentTrack] = useState(null);
  const [playing, setPlaying] = useState(false);
  const [progress, setProgress] = useState(0);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);

  useEffect(() => {
    ensureAudioContext();
    const audio = globalAudio;

    const onTime = () => {
      if (audio.duration) {
        setProgress((audio.currentTime / audio.duration) * 100);
        setCurrentTime(audio.currentTime);
        setDuration(audio.duration);
      }
    };
    const onEnd = () => { setPlaying(false); setProgress(0); setCurrentTime(0); };
    const onPlay = () => {
      if (audioCtx?.state === "suspended") audioCtx.resume();
      setPlaying(true);
    };
    const onPause = () => setPlaying(false);

    audio.addEventListener("timeupdate", onTime);
    audio.addEventListener("ended", onEnd);
    audio.addEventListener("play", onPlay);
    audio.addEventListener("pause", onPause);

    return () => {
      audio.removeEventListener("timeupdate", onTime);
      audio.removeEventListener("ended", onEnd);
      audio.removeEventListener("play", onPlay);
      audio.removeEventListener("pause", onPause);
    };
  }, []);

  const play = useCallback((track) => {
    ensureAudioContext();
    const audio = globalAudio;
    if (!audio) return;

    if (currentTrack?.id === track.id) {
      if (audio.paused) audio.play().catch(() => {});
      else audio.pause();
      return;
    }

    audio.src = track.streamUrl;
    audio.play().catch(() => {});
    setCurrentTrack(track);
    setProgress(0);
    setCurrentTime(0);
  }, [currentTrack]);

  const seek = useCallback((pct) => {
    if (globalAudio && globalAudio.duration) {
      globalAudio.currentTime = pct * globalAudio.duration;
    }
  }, []);

  const pause = useCallback(() => {
    globalAudio?.pause();
  }, []);

  return { currentTrack, playing, progress, currentTime, duration, play, pause, seek };
}
