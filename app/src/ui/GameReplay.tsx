import { useEffect, useRef, useState, type KeyboardEvent } from 'react';
import { isBarbarian } from '../barbarians';
import { FACTION_INFO } from '../data';
import { empireOfPlayer, empireTitle, UNKNOWN_EMPIRE } from '../empires';
import { ReplayMap } from '../map/ReplayMap';
import { nextReplayDeadline, playbackTurn, replayProfilingEnabled, replayTurnLabel, type ReplayInfo, type ReplayMoment, type ReplayRequest, type ReplayResponse } from '../replay';
import type { PlayerView } from '../types';
import { Glyph } from './ringMedallion';
import './gameReplay.css';

const SPEEDS = [1, 2, 4];

function momentLabel(moment: ReplayMoment, view: PlayerView): string {
  const playerName = (id: number | null) => id === null ? 'An unknown empire' : isBarbarian(id) ? 'Barbarians' : empireOfPlayer(view, id);
  const teamName = (id: number | null, start = false) => id === null ? (start ? 'An unknown empire' : UNKNOWN_EMPIRE) : empireTitle(view, id);
  if (moment.kind === 'founded' || moment.kind === 'captured') {
    return `${playerName(moment.player)} ${moment.kind} ${moment.city}`;
  }
  if (moment.kind === 'war') {
    if (moment.a === null && moment.b === null) return 'Two unknown empires went to war';
    return `${teamName(moment.a, true)} and ${teamName(moment.b)} went to war`;
  }
  if (moment.kind === 'declaredWar') return `${teamName(moment.from, true)} declared war on ${teamName(moment.against)}`;
  if (moment.kind === 'eliminated') return `${playerName(moment.player)} fell`;
  if (moment.kind === 'scored') return `You gained ${moment.gained} VP (${moment.total} total)`;
  if (moment.kind === 'hold' || moment.kind === 'won') {
    const name = teamName(moment.team, true);
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
  const [chosenMoment, setChosenMoment] = useState<number>();
  const seek = useRef<(turn: number, prefetch?: boolean) => void>(() => {});
  const deadline = useRef<number | null>(null);
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
    let ahead: number | null = null;
    let shown: number | null = null;
    let inFlight: number | null = null;
    let buffered: ReplayFrame | null = null;
    const send = (message: ReplayRequest) => worker.postMessage(message);
    const flush = () => {
      if (buffered !== null && queued === buffered.turn && queued !== shown) {
        shown = buffered.turn;
        setFrame(buffered);
        buffered = null;
      }
      const next = queued !== shown ? queued : ahead;
      if (inFlight !== null || next === null || next === shown || next === buffered?.turn) return;
      inFlight = next;
      send({ type: 'seek', turn: next });
    };
    seek.current = (next, prefetch = false) => {
      if (prefetch) ahead = next;
      else { queued = next; ahead = null; }
      flush();
    };
    worker.onmessage = ({ data }: MessageEvent<ReplayResponse>) => {
      if (data.type === 'ready') {
        const first = data.info.first;
        shown = queued = first;
        setInfo(data.info);
        setTurn(first);
        setFrame({ turn: first, view: data.view });
      } else if (data.type === 'frame') {
        if (data.timing) performance.mark('replay-worker', { detail: [data.timing, performance.timeOrigin + performance.now(), data.turn] });
        inFlight = null;
        if (queued === data.turn || ahead === data.turn) buffered = { turn: data.turn, view: data.view };
        flush();
      } else fail();
    };
    worker.onerror = fail;
    send({ type: 'open', recording, player, profile: replayProfilingEnabled() });
    return () => {
      worker.terminate();
      seek.current = () => {};
    };
  }, [player, recording]);

  const move = (next: number) => {
    setChosenMoment(undefined);
    setTurn(next);
    seek.current(next);
  };
  const jump = (next: number) => { deadline.current = null; setPlaying(false); move(next); };

  useEffect(() => {
    if (!playing || !info || turn !== frame?.turn) return;
    if (turn === info.last) {
      deadline.current = null;
      setPlaying(false);
      return;
    }
    seek.current(turn + 1, true);
    const now = performance.now();
    const due = nextReplayDeadline(deadline.current, now, speed);
    const id = window.setTimeout(() => {
      deadline.current = due;
      move(turn + 1);
    }, due - now);
    return () => window.clearTimeout(id);
  }, [playing, info, turn, frame, speed]);

  useEffect(() => {
    const node = activeMoment.current;
    const list = node?.parentElement;
    if (!node || !list) return;
    if (node.offsetTop < list.scrollTop || node.offsetTop + node.offsetHeight > list.scrollTop + list.clientHeight) list.scrollTop = node.offsetTop;
    if (node.offsetLeft < list.scrollLeft || node.offsetLeft + node.offsetWidth > list.scrollLeft + list.clientWidth) list.scrollLeft = node.offsetLeft;
  }, [frame?.turn, chosenMoment]);

  const togglePlay = () => {
    if (!info) return;
    if (playing) { deadline.current = null; setPlaying(false); return; }
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
  const empire = frame?.view.players.find((p) => p.id === frame.view.me);
  const active = frame && info ? (chosenMoment !== undefined && info.milestones[chosenMoment]?.turn === frame.turn
    ? chosenMoment : info.milestones.findLastIndex((m) => m.turn <= frame.turn)) : -1;

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
                      aria-current={index === active ? 'step' : undefined} onClick={() => { jump(at); setChosenMoment(index); }}>
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
                <select value={speed} onChange={(e) => { deadline.current = null; setSpeed(Number(e.currentTarget.value)); }}>
                  {SPEEDS.map((rate) => <option key={rate} value={rate}>{rate}×</option>)}
                </select>
              </label>
            </footer>
          </>}
    </section>
  );
}
