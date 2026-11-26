// @vitest-environment happy-dom
import { AnimationMixer, Group, Texture, TextureLoader } from 'three';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import type { PlayerView, TileView } from '../types';
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
const placeBanners = vi.fn();
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
  placeBanners.mockClear();
  canvas = document.createElement('canvas');
  scene = new MapScene(canvas, placeBanners);
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
  expect(placeBanners).toHaveBeenCalledTimes(1);
  expect(frames.size).toBe(0);

  scene.setOverlays({ hover: { q: 5, r: 4 }, selected: null, route: [] });
  frame();
  expect(scene.renderer.render).toHaveBeenCalledTimes(2);
  expect(placeBanners).toHaveBeenCalledTimes(1);

  scene.resize(1000, 600);
  frame();
  expect(scene.renderer.render).toHaveBeenCalledTimes(3);
  expect(placeBanners).toHaveBeenCalledTimes(2);
  expect(frames.size).toBe(0);
});

it('keeps frames running for explored water in view, without reprojecting banners', () => {
  scene.target.x = 36;
  scene.placeCamera();
  frame();
  frame();
  frame();
  expect(scene.renderer.render).toHaveBeenCalledTimes(3);
  expect(placeBanners).toHaveBeenCalledTimes(1);
  expect(frames.size).toBe(1);

  scene.target.x = 5;
  scene.placeCamera();
  frame();
  expect(scene.renderer.render).toHaveBeenCalledTimes(4);
  expect(placeBanners).toHaveBeenCalledTimes(2);
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
  expect(placeBanners).toHaveBeenCalledTimes(1);
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
