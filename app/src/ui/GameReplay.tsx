import { useEffect, useRef, useState, type KeyboardEvent } from 'react';
import { FACTION_INFO } from '../data';
import { empireOfPlayer, empireTitle } from '../empires';
import { ReplayMap } from '../map/ReplayMap';
import { playbackTurn, replayTurnLabel, type ReplayInfo, type ReplayMoment, type ReplayRequest, type ReplayResponse } from '../replay';
import type { PlayerView } from '../types';
import { Glyph } from './ringMedallion';
import './gameReplay.css';

const SPEEDS = [1, 4, 16];
const TURN_MS = 1200;

function momentLabel(moment: ReplayMoment, view: PlayerView): string {
  if (moment.kind === 'founded' || moment.kind === 'captured') {
    return `${empireOfPlayer(view, moment.player)} ${moment.kind} ${moment.city}`;
  }
  if (moment.kind === 'hold' || moment.kind === 'won') {
    const name = empireTitle(view, moment.team);
    return `${name} ${moment.kind === 'hold' ? 'reached the VP target' : 'won'}`;
  }
  return 'The game ended in a draw';
}

interface ReplayFrame { turn: number; view: PlayerView }

export function GameReplay({ player, recording, titleId, onBack }: {
  player: number;
  recording: string | null;
  titleId: string;
  onBack(): void;
}) {
  const [info, setInfo] = useState<ReplayInfo | null>(null);
  const [frame, setFrame] = useState<ReplayFrame | null>(null);
  const [turn, setTurn] = useState(0);
  const [playing, setPlaying] = useState(false);
  const [speed, setSpeed] = useState(1);
  const [failed, setFailed] = useState(false);
  const seek = useRef<(turn: number) => void>(() => {});
  const activeMoment = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    if (!recording) return;
    let worker: Worker;
    const fail = () => { setFailed(true); setPlaying(false); };
    try {
      worker = new Worker(new URL('../replay.worker.ts', import.meta.url), { type: 'module' });
    } catch {
      fail();
      return;
    }
    let queued: number | null = null;
    let shown: number | null = null;
    let inFlight = false;
    const send = (message: ReplayRequest) => worker.postMessage(message);
    const flush = () => {
      if (inFlight || queued === null || queued === shown) return;
      inFlight = true;
      send({ type: 'seek', turn: queued });
    };
    seek.current = (next) => { queued = next; flush(); };
    worker.onmessage = ({ data }: MessageEvent<ReplayResponse>) => {
      if (data.type === 'ready') {
        const first = data.info.first;
        shown = first;
        setInfo(data.info);
        setTurn(first);
        setFrame({ turn: first, view: data.view });
      } else if (data.type === 'frame') {
        inFlight = false;
        if (queued === data.turn) {
          shown = data.turn;
          setFrame({ turn: data.turn, view: data.view });
        }
        flush();
      } else fail();
    };
    worker.onerror = fail;
    send({ type: 'open', recording, player });
    return () => {
      worker.terminate();
      seek.current = () => {};
    };
  }, [player, recording]);

  const move = (next: number) => {
    setTurn(next);
    seek.current(next);
  };
  const jump = (next: number) => { setPlaying(false); move(next); };

  useEffect(() => {
    if (!playing || !info || turn !== frame?.turn) return;
    if (turn === info.last) {
      setPlaying(false);
      return;
    }
    const id = window.setTimeout(() => move(turn + 1), TURN_MS / speed);
    return () => window.clearTimeout(id);
  }, [playing, info, turn, frame, speed]);

  useEffect(() => {
    const node = activeMoment.current;
    const list = node?.parentElement;
    if (!node || !list) return;
    if (node.offsetTop < list.scrollTop || node.offsetTop + node.offsetHeight > list.scrollTop + list.clientHeight) list.scrollTop = node.offsetTop;
    if (node.offsetLeft < list.scrollLeft || node.offsetLeft + node.offsetWidth > list.scrollLeft + list.clientWidth) list.scrollLeft = node.offsetLeft;
  }, [frame?.turn]);

  const togglePlay = () => {
    if (!info) return;
    if (playing) { setPlaying(false); return; }
    if (turn === info.last) move(info.first);
    setPlaying(true);
  };
  const keys = (e: KeyboardEvent<HTMLElement>) => {
    if (!info || failed || e.defaultPrevented || e.altKey || e.ctrlKey || e.metaKey || e.shiftKey) return;
    const target = e.target as HTMLElement;
    if (target.closest('input, select')) return;
    if (e.key === ' ' && !target.closest('button')) {
      e.preventDefault();
      togglePlay();
    } else {
      const next = playbackTurn(e.key, turn, info.first, info.last);
      if (next !== undefined) { e.preventDefault(); jump(next); }
    }
  };

  const unavailable = failed || !recording;
  const label = info ? replayTurnLabel(turn, info.first, info.partial) : '';
  const empire = frame?.view.players.find((p) => p.id === player);
  const active = frame && info ? info.milestones.findLastIndex((m) => m.turn <= frame.turn) : -1;

  return (
    <section className="match-replay" aria-labelledby={titleId} onKeyDown={keys} data-testid="match-replay">
      <header className="replay-head">
        <h1 id={titleId} tabIndex={0} data-replay-focus>Game replay</h1>
        <p>{empire ? `${FACTION_INFO[empire.faction].name} · Your view` : 'Your empire’s view'}
          {info?.partial && ` · Recording starts before turn ${info.first + 1}`}</p>
        <button type="button" className="btn quiet" onClick={onBack}>Back to report</button>
      </header>
      {unavailable ? <div className="replay-placeholder" role="alert">Couldn’t open this replay.</div>
        : !info || !frame ? <div className="replay-placeholder" role="status">Preparing your replay…</div>
          : <>
            <div className="replay-body">
              <div className="replay-atlas" data-testid="replay-atlas" data-turn={frame.turn}>
                <ReplayMap view={frame.view} />
                {empire && <span className="replay-vp"><Glyph name="vp" size={16} /> {empire.vp} VP</span>}
              </div>
              <aside className="replay-moments" aria-label="Turning points">
                <h2>Turning points</h2>
                <div className="replay-moment-list">
                  {info.milestones.map(({ turn: at, moment }, index) => (
                    <button key={index} type="button" ref={index === active ? activeMoment : undefined}
                      aria-current={index === active ? 'step' : undefined} onClick={() => jump(at)}>
                      <span>Turn {at}</span>{momentLabel(moment, frame.view)}
                    </button>
                  ))}
                </div>
              </aside>
            </div>
            <footer className="replay-controls">
              <button type="button" className="btn primary" onClick={togglePlay}>{playing ? 'Pause' : turn === info.last ? 'Play again' : 'Play'}</button>
              <button type="button" className="btn replay-step" aria-label="Previous turn" disabled={turn === info.first} onClick={() => jump(turn - 1)}>‹</button>
              <button type="button" className="btn replay-step" aria-label="Next turn" disabled={turn === info.last} onClick={() => jump(turn + 1)}>›</button>
              <label className="replay-seek">
                <span>{label}</span>
                <input type="range" min={info.first} max={info.last} value={turn} aria-label="Replay turn" aria-valuetext={label}
                  onChange={(e) => jump(e.currentTarget.valueAsNumber)} />
              </label>
              <label className="replay-speed">Speed
                <select value={speed} onChange={(e) => setSpeed(Number(e.currentTarget.value))}>
                  {SPEEDS.map((rate) => <option key={rate} value={rate}>{rate}×</option>)}
                </select>
              </label>
            </footer>
          </>}
    </section>
  );
}
