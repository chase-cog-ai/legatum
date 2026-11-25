import type { ReplayInfo, ReplayRequest, ReplayResponse } from './replay';
import type { PlayerView } from './types';
import type { WasmModule } from './wasmClient';

const packages = import.meta.glob<WasmModule>('./wasm-pkg/legatum.js', { eager: true });
let replay: InstanceType<WasmModule['GameReplay']> | null = null;
let profile = false;
const send = (message: ReplayResponse) => self.postMessage(message);

self.onmessage = async ({ data }: MessageEvent<ReplayRequest>) => {
  try {
    if (data.type === 'open') {
      profile = data.profile;
      const wasm = Object.values(packages)[0];
      if (!wasm) throw new Error('Replay engine unavailable');
      await wasm.default?.();
      replay = new wasm.GameReplay(data.recording, data.player);
      const info: ReplayInfo = JSON.parse(replay.info());
      send({ type: 'ready', info, view: JSON.parse(replay.frame(info.first)) as PlayerView });
    } else if (replay) {
      const start = profile ? performance.now() : 0;
      const raw = replay.frame(data.turn);
      const computed = profile ? performance.now() : 0;
      const view = JSON.parse(raw) as PlayerView;
      const frame: ReplayResponse = { type: 'frame', turn: data.turn, view };
      if (profile) {
        const parsed = performance.now();
        frame.timing = [computed - start, parsed - computed, performance.timeOrigin + parsed];
      }
      send(frame);
    }
  } catch {
    send({ type: 'failed' });
  }
};
