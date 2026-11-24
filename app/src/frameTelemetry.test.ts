import { describe, expect, it } from 'vitest';
import { FrameStats, workIn } from './frameTelemetry';

const scene = { units: 40, cities: 12, zoom: 2, canvasPixels: 2880 * 1800 };

describe('frame telemetry', () => {
  it('summarises frame gaps, inputs, long tasks and map draws for the engine', () => {
    const s = new FrameStats();
    for (let i = 0; i < 95; i++) s.frame(16.7);
    for (let i = 0; i < 5; i++) s.frame(120);
    s.input(20); s.input(180);
    s.longTask(90);
    s.draw(4); s.draw(8);
    const t = s.summary(126, scene);
    expect(t).toMatchObject({ turn: 126, frames: 100, frame_p50_ms10: 170, frame_p95_ms10: 170, frame_max_ms: 120, slow_frames: 5,
      inputs: 2, input_max_ms: 180, long_tasks: 1, long_task_ms: 90, map_draws: 2, map_draw_mean_ms10: 60, map_draw_max_ms: 8,
      units: 40, cities: 12, zoom100: 200, canvas_mp10: 52 });
    s.frame(120);
    expect(s.summary(126, scene).frame_p95_ms10).toBe(1200);
  });

  it('says what filled a long task, and never more than the task', () => {
    // A 100 ms task from t=1000: a 60 ms draw (a third of it terrain) and a 30 ms engine call inside it, one span outside.
    const spans = [
      { kind: 'draw' as const, start: 1000, end: 1060, share: 2 / 3 }, { kind: 'terrain' as const, start: 1000, end: 1060, share: 1 / 3 },
      { kind: 'engine' as const, start: 1065, end: 1095, share: 1 }, { kind: 'engine' as const, start: 1200, end: 1300, share: 1 },
      { kind: 'engine' as const, start: 990, end: 1002, share: 1 },
    ];
    const by = workIn(1000, 100, spans);
    expect(Math.round(by.draw!)).toBe(40); expect(Math.round(by.terrain!)).toBe(20); expect(Math.round(by.engine!)).toBe(32);
    const s = new FrameStats();
    s.longTask(100, by); s.longTask(60);
    s.longTask(50, { engine: 80, draw: 20 });
    const t = s.summary(3, scene);
    expect(t).toMatchObject({ long_tasks: 3, long_task_ms: 210, long_draw_ms: 50, long_terrain_ms: 20, long_engine_ms: 72 });
    expect(t.long_draw_ms + t.long_terrain_ms + t.long_engine_ms).toBeLessThanOrEqual(t.long_task_ms);
  });

  it('records an empty turn as zeros', () => {
    expect(new FrameStats().summary(1, scene)).toMatchObject({ frames: 0, frame_p50_ms10: 0, frame_p95_ms10: 0, inputs: 0, map_draws: 0, map_draw_mean_ms10: 0 });
  });
});
