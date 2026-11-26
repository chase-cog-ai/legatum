// @vitest-environment happy-dom
import { AnimationMixer, Group, Texture, TextureLoader } from 'three';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import type { PlayerView, TileView } from '../types';
import type { CombatPlan } from '../map/combatFx';
import { MapScene } from './scene';

vi.mock('three', async (original) => ({
  ...await original<typeof import('three')>(),
  WebGLRenderer: class {
    shadowMap = {};
    render = vi.fn();
    setPixelRatio = vi.fn();
    setSize = vi.fn();
    dispose = vi.fn();
    constructor({ canvas }: { canvas: HTMLCanvasElement }) { this.domElement = canvas; }
    readonly domElement: HTMLCanvasElement;
  },
}));

const view = {
  width: 38, height: 12, players: [], units: [], cities: [],
  tiles: Array.from({ length: 38 * 12 }, (_, i): TileView => ({
    terrain: i % 38 >= 34 ? 'Ocean' : 'Plains', explored: true, visible: false, owner: null, ruin: false,
  })),
} as Pick<PlayerView, 'width' | 'height' | 'players' | 'units' | 'cities' | 'tiles'> as PlayerView;

let frames: Map<number, FrameRequestCallback>;
let scene: MapScene;
let canvas: HTMLCanvasElement;
const drawMarks = vi.fn((_now: number) => false);
const frame = (now = performance.now()) => {
  const callbacks = [...frames.values()];
  frames.clear();
  callbacks.forEach((callback) => callback(now));
};

beforeEach(() => {
  frames = new Map();
  let next = 0;
  vi.stubGlobal('requestAnimationFrame', (callback: FrameRequestCallback) => { frames.set(++next, callback); return next; });
  vi.stubGlobal('cancelAnimationFrame', (id: number) => frames.delete(id));
  drawMarks.mockReset();
  drawMarks.mockReturnValue(false);
  canvas = document.createElement('canvas');
  scene = new MapScene(canvas, drawMarks);
  scene.resize(800, 600);
  scene.dist = 4;
  scene.target.set(5, 0, 4);
  scene.placeCamera();
  scene.setView(view);
});

afterEach(() => {
  scene.dispose();
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
});

it('coalesces invalidations and stays idle with offscreen water', () => {
  expect(frames.size).toBe(1);
  frame();
  frame();
  expect(scene.renderer.render).toHaveBeenCalledTimes(1);
  expect(drawMarks).toHaveBeenCalledTimes(1);
  expect(frames.size).toBe(0);

  scene.setOverlays({ hover: { q: 5, r: 4 }, selected: null, route: [] });
  frame();
  expect(scene.renderer.render).toHaveBeenCalledTimes(2);
  expect(drawMarks).toHaveBeenCalledTimes(1);

  scene.resize(1000, 600);
  frame();
  expect(scene.renderer.render).toHaveBeenCalledTimes(3);
  expect(drawMarks).toHaveBeenCalledTimes(2);
  expect(frames.size).toBe(0);
});

it('keeps frames running for explored water in view without redrawing marks', () => {
  scene.target.x = 36;
  scene.placeCamera();
  frame();
  frame();
  frame();
  expect(scene.renderer.render).toHaveBeenCalledTimes(3);
  expect(drawMarks).toHaveBeenCalledTimes(1);
  expect(frames.size).toBe(1);

  scene.target.x = 5;
  scene.placeCamera();
  frame();
  expect(scene.renderer.render).toHaveBeenCalledTimes(4);
  expect(drawMarks).toHaveBeenCalledTimes(2);
  expect(frames.size).toBe(0);
});

it('resumes the water at its last phase after it leaves the view', () => {
  scene.target.x = 36;
  scene.placeCamera();
  frame(0);
  frame(16);
  const phase = scene['uniforms'].uTime;
  expect(phase.value).toBeCloseTo(0.016);

  scene.target.x = 5;
  scene.placeCamera();
  frame(32);
  expect(frames.size).toBe(0);

  scene.target.x = 36;
  scene.placeCamera();
  frame(5_000);
  expect(phase.value).toBeCloseTo(0.016);
  frame(5_016);
  expect(phase.value).toBeCloseTo(0.032);
});

it('redraws after a unit texture loads or WebGL is restored', () => {
  let loaded: (() => void) | undefined;
  vi.spyOn(TextureLoader.prototype, 'load').mockImplementation((_url, onLoad) => {
    const texture = new Texture();
    loaded = () => onLoad?.(texture);
    return texture;
  });
  scene.setView({ ...view,
    tiles: view.tiles.map((t) => ({ ...t, visible: true })),
    units: [{ id: 1, owner: 0, kind: 'Scout', pos: { q: 5, r: 4 } } as PlayerView['units'][number]],
  });
  frame();
  expect(frames.size).toBe(0);
  expect(loaded).toBeDefined();
  loaded?.();
  frame();
  canvas.dispatchEvent(new Event('webglcontextrestored'));
  frame();
  expect(scene.renderer.render).toHaveBeenCalledTimes(3);
  expect(drawMarks).toHaveBeenCalledTimes(1);
  expect(frames.size).toBe(0);
});

it('idles with paused model animation and wakes when motion is enabled', () => {
  const mixer = new AnimationMixer(new Group());
  scene['mixers'].push(mixer);
  scene.setStill(true);
  frame(0);
  expect(mixer.time).toBe(0);
  expect(frames.size).toBe(0);

  scene.setStill(false);
  frame(1_000);
  frame(1_016);
  expect(mixer.time).toBeCloseTo(0.016);
  expect(frames.size).toBe(1);

  scene.setStill(true);
  frame(1_032);
  expect(mixer.time).toBeCloseTo(0.016);
  expect(frames.size).toBe(0);
});

it('animates route marks without rendering WebGL and stops when the route clears', () => {
  frame(0);
  drawMarks.mockReturnValue(true);
  scene.invalidateMarks();
  scene.invalidateMarks();
  expect(frames.size).toBe(1);
  frame(16);
  frame(32);
  expect(drawMarks).toHaveBeenCalledTimes(3);
  expect(scene.renderer.render).toHaveBeenCalledTimes(1);
  expect(frames.size).toBe(1);

  drawMarks.mockReturnValue(false);
  scene.invalidateMarks();
  frame(48);
  expect(drawMarks).toHaveBeenCalledTimes(4);
  expect(frames.size).toBe(0);
});

it('redraws the scene and moving marks through the battle cleanup frame', () => {
  frame(0);
  const fig = new Group();
  const home = scene.target.clone();
  fig.position.copy(home);
  fig.userData.home = home;
  scene['unitGroups'].set(1, fig);
  const plan: CombatPlan = {
    volley: 0, stepMs: 280, total: 280, shots: [], deaths: [], captures: [],
    moves: new Map(), ghosts: new Map(), cueDelay: new Map(), hp: new Map(),
    clashes: [{
      attacker: 1, defender: null, from: { x: 0, y: 0 }, to: { x: 64, y: 0 }, t: 150,
      lunge: true, mounted: false, guns: false, atCity: false, hit: false, seed: 1,
    }],
  };
  const anchors: number[] = [];
  drawMarks.mockImplementation(() => { anchors.push(fig.position.x); return false; });
  scene.setAnim({ start: 1_000, duration: 280, paths: new Map(), plan });
  frame(1_150);
  expect(anchors[0]).not.toBe(home.x);
  expect(frames.size).toBe(1);
  frame(1_600);
  expect(anchors[1]).toBe(home.x);
  expect(scene.renderer.render).toHaveBeenCalledTimes(3);
  expect(frames.size).toBe(0);
});
