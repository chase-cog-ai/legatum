import { readFileSync } from 'node:fs';
import type { GameSetup, Order, PlayerView } from '../types';

const loadEngine = async () => {
  const mod = await import('../wasm-pkg/legatum.js');
  await mod.default({ module_or_path: readFileSync(new URL('../wasm-pkg/legatum_bg.wasm', import.meta.url)) });
  return mod;
};

export async function replayFixture() {
  const { Game, advisor_orders, explore_step } = await loadEngine();
  const setup = JSON.parse(Game.standard_setup('42', 'Sparta', 'India', 'Ada', 'India')) as GameSetup;
  setup.width = 48;
  setup.height = 32;
  setup.turn_limit = 300;
  setup.players = setup.players.slice(0, 4);
  setup.players[1] = { ...setup.players[1], is_human: false, team: 3 };
  setup.rule_experiments = { start_settlers: 1 };
  const game = new Game(JSON.stringify(setup));
  try {
    let raw = game.view(0);
    let view = JSON.parse(raw) as PlayerView;
    while (!view.finished) {
      const scout = view.units.find((unit) => unit.owner === 0 && unit.kind === 'Scout');
      const step = scout ? JSON.parse(explore_step(raw, scout.id)) as Order | null : null;
      const orders = JSON.parse(advisor_orders(raw)) as Order[];
      if (scout && step && !orders.some((order) => 'unit' in order && order.unit === scout.id)) orders.push(step);
      game.end_turn(JSON.stringify({ 0: orders }));
      raw = game.view(0);
      view = JSON.parse(raw) as PlayerView;
    }
    return { game: game.save(), turn: view.turn };
  } finally {
    game.free();
  }
}
