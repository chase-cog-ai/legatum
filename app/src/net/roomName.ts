// The name a hosted game shows in the public game list. The list never shows text a player typed: the relay
// names each room from its id (infra/relay/src/shared.ts `listedName`), and this is a copy of that rule, so the
// host can tell friends which game to join. Keep the two in step.

const ADJECTIVES = [
  'Amber', 'Bronze', 'Cedar', 'Cobalt', 'Copper', 'Crimson', 'Golden', 'Granite',
  'Indigo', 'Ivory', 'Jade', 'Marble', 'Olive', 'Saffron', 'Silver', 'Scarlet',
];
const NOUNS = [
  'Banner', 'Bridge', 'Citadel', 'Compass', 'Delta', 'Forum', 'Garden', 'Harbour',
  'Highland', 'Lantern', 'Meadow', 'Oasis', 'Ridge', 'River', 'Steppe', 'Valley',
];
const ID_ALPHABET = 'abcdefghijkmnpqrstuvwxyz23456789';

/** The name the game list shows for the room with this id. */
export function listedRoomName(room: string): string {
  const at = (i: number) => Math.max(0, ID_ALPHABET.indexOf(room[i] ?? 'a'));
  const tag = String((at(2) * 32 + at(3)) % 100).padStart(2, '0');
  return `${ADJECTIVES[at(0) % ADJECTIVES.length]} ${NOUNS[at(1) % NOUNS.length]} ${tag}`;
}
