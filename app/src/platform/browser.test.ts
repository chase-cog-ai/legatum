import { describe, expect, it } from 'vitest';
import { browserNote } from './browser';

const CHROME = 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36';
const EDGE = `${CHROME} Edg/142.0.0.0`;
const SAFARI = 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/19.1 Safari/605.1.15';
const FIREFOX = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:145.0) Gecko/20100101 Firefox/145.0';
const CHROME_IPHONE = 'Mozilla/5.0 (iPhone; CPU iPhone OS 19_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/142.0.0.0 Mobile/15E148 Safari/604.1';

describe('browser note', () => {
  it('says nothing in Chrome and browsers built on it', () => {
    expect(browserNote({ userAgent: CHROME, platform: 'MacIntel', maxTouchPoints: 0 })).toBeNull();
    expect(browserNote({ userAgent: EDGE, platform: 'Win32' })).toBeNull();
    expect(browserNote({ userAgent: 'something new', userAgentData: { brands: [{ brand: 'Chromium' }] } })).toBeNull();
  });
  it('points Safari and Firefox at Chrome', () => {
    expect(browserNote({ userAgent: SAFARI, platform: 'MacIntel', maxTouchPoints: 0 })).toBe('Runs best in Chrome');
    expect(browserNote({ userAgent: FIREFOX, platform: 'Win32' })).toBe('Runs best in Chrome');
  });
  it('points an iPhone or iPad at a computer, whatever the browser is called', () => {
    expect(browserNote({ userAgent: CHROME_IPHONE, platform: 'iPhone', maxTouchPoints: 5 })).toBe('Runs best in Chrome on a computer');
    expect(browserNote({ userAgent: SAFARI, platform: 'MacIntel', maxTouchPoints: 5 })).toBe('Runs best in Chrome on a computer');
  });
  it('says nothing where there is no browser', () => { expect(browserNote(null)).toBeNull(); });
});
