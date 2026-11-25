import { useLayoutEffect, useMemo, useRef, type PointerEvent } from 'react';
import { onIconLoad } from '../icons';
import { playerColor } from '../theme';
import type { PlayerView } from '../types';
import { replayProfilingEnabled } from '../replay';
import { onCityArtLoad } from './cityArt';
import { emptyGesture, gestureStep, zoomCamera, type GestureEvent } from './gestures';
import { onImprovementArtLoad } from './improvementArt';
import { onLandmarkArtLoad } from './landmarks';
import { onMapFeatureLoad } from './mapFeatures';
import { draw, type Camera } from './render';
import { fitReplayCamera, replayBounds } from './replayCamera';
import { onWonderArtLoad } from './wonderArt';

export function ReplayMap({ view }: { view: PlayerView }) {
  const canvas = useRef<HTMLCanvasElement>(null);
  const camera = useRef<Camera | null>(null);
  const manualCamera = useRef(false);
  const size = useRef({ w: 0, h: 0 });
  const gesture = useRef(emptyGesture());
  const scope = useRef({});
  const repaint = useRef<() => void>(() => {});
  const bounds = useMemo(() => replayBounds(view), [view]);
  const frame = useRef({ view, bounds });

  useLayoutEffect(() => {
    const el = canvas.current;
    const ctx = el?.getContext('2d');
    if (!el || !ctx) return;
    let pending = 0;
    let attempts = 0;
    const profile = replayProfilingEnabled();
    const paint = () => {
      pending = 0;
      const { view, bounds } = frame.current;
      const drawStart = profile ? performance.now() : 0;
      const w = el.clientWidth, h = el.clientHeight;
      if (!w || !h) return;
      const dpr = Math.min(2, window.devicePixelRatio || 1);
      const width = Math.round(w * dpr), height = Math.round(h * dpr);
      if (el.width !== width) el.width = width;
      if (el.height !== height) el.height = height;
      if (size.current.w !== w || size.current.h !== h) {
        size.current = { w, h };
        manualCamera.current = false;
      }
      if (!manualCamera.current || !camera.current) camera.current = fitReplayCamera(bounds, w, h);
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
      ctx.clearRect(0, 0, w, h);
      const result = draw(ctx, {
        view, terrainScope: scope.current, cam: camera.current, width: w, height: h, dpr, time: 0,
        hover: null, selectedUnit: null, selectedAir: null, selectedCity: null,
        path: null, pathAttack: false, pathUnreachable: null, pendingLines: [], pendingMarks: new Map(),
        rangedTiles: null, unitPos: new Map(), seatColor: playerColor(view.me), reducedMotion: true,
      });
      if (profile) performance.measure('replay-draw', { start: drawStart, detail: [view.turn - 1, result.pending] });
      if (result.pending && attempts++ < 90) pending = requestAnimationFrame(paint);
    };
    const schedule = () => {
      attempts = 0;
      if (!pending) pending = requestAnimationFrame(paint);
    };
    repaint.current = schedule;
    const resize = new ResizeObserver(schedule);
    resize.observe(el);
    const subscriptions = [onIconLoad, onMapFeatureLoad, onCityArtLoad, onImprovementArtLoad, onLandmarkArtLoad, onWonderArtLoad]
      .map((subscribe) => subscribe(schedule));
    const wheel = (e: WheelEvent) => {
      if (!camera.current) return;
      e.preventDefault();
      const rect = el.getBoundingClientRect();
      const point = { x: e.clientX - rect.left, y: e.clientY - rect.top };
      manualCamera.current = true;
      camera.current = zoomCamera(camera.current, { w: rect.width, h: rect.height }, point, point, Math.exp(-e.deltaY * .0015));
      schedule();
    };
    el.addEventListener('wheel', wheel, { passive: false });
    schedule();
    return () => {
      cancelAnimationFrame(pending);
      resize.disconnect();
      subscriptions.forEach((unsubscribe) => unsubscribe());
      el.removeEventListener('wheel', wheel);
    };
  }, []);

  useLayoutEffect(() => {
    frame.current = { view, bounds };
    repaint.current();
  }, [view, bounds]);

  const pointer = (type: 'down' | 'move' | 'up' | 'cancel', e: PointerEvent<HTMLCanvasElement>) => {
    if (!camera.current) return;
    const el = e.currentTarget;
    const rect = el.getBoundingClientRect();
    const point = { x: e.clientX - rect.left, y: e.clientY - rect.top };
    if (type === 'down') el.setPointerCapture(e.pointerId);
    const event: GestureEvent = type === 'down'
      ? { type, id: e.pointerId, point, kind: e.pointerType, button: e.button, pathDrag: false }
      : type === 'cancel' ? { type, id: e.pointerId } : { type, id: e.pointerId, point };
    const next = gestureStep(gesture.current, event, camera.current, { w: rect.width, h: rect.height });
    gesture.current = next.state;
    if (camera.current !== next.camera) {
      manualCamera.current = true;
      camera.current = next.camera;
      repaint.current();
    }
  };

  return <canvas ref={canvas} className="replay-map" role="img" aria-label="Map as your empire saw it"
    onPointerDown={(e) => pointer('down', e)} onPointerMove={(e) => pointer('move', e)}
    onPointerUp={(e) => pointer('up', e)} onPointerCancel={(e) => pointer('cancel', e)} />;
}
