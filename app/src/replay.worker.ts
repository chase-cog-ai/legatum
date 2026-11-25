import type { ReplayInfo, ReplayRequest, ReplayResponse } from './replay';
import type { PlayerView } from './types';
import type { WasmModule } from './wasmClient';

const packages = import.meta.glob<WasmModule>('./wasm-pkg/legatum.js', { eager: true });
let replay: InstanceType<WasmModule['GameReplay']> | null = null;
const send = (message: ReplayResponse) => self.postMessage(message);

self.onmessage = async ({ data }: MessageEvent<ReplayRequest>) => {
  try {
    if (data.type === 'open') {
      const wasm = Object.values(packages)[0];
      if (!wasm) throw new Error('Replay engine unavailable');
      await wasm.default?.();
      replay = new wasm.GameReplay(data.recording, data.player);
      const info: ReplayInfo = JSON.parse(replay.info());
      send({ type: 'ready', info, view: JSON.parse(replay.frame(info.first)) as PlayerView });
    } else if (replay) {
      send({ type: 'frame', turn: data.turn, view: JSON.parse(replay.frame(data.turn)) as PlayerView });
    }
  } catch {
    send({ type: 'failed' });
  }
};
