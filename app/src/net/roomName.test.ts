import { expect, it } from 'vitest';
import { listedRoomName } from './roomName';

it('names a room from its id with two plain words and a number', () => {
  expect(listedRoomName('abcdefghijkmnpqr')).toBe('Amber Bridge 67');
  expect(listedRoomName('99999999')).toBe('Scarlet Valley 23');
  expect(listedRoomName('abcdefghijkmnpqr')).toBe(listedRoomName('abcdzzzz'));
});

it('only ever uses its own words', () => {
  const alphabet = 'abcdefghijkmnpqrstuvwxyz23456789';
  for (const a of alphabet) for (const b of alphabet) expect(listedRoomName(`${a}${b}xy`)).toMatch(/^[A-Z][a-z]+ [A-Z][a-z]+ \d\d$/);
});
