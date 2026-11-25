// The post-game "Rival plans" tab: what each AI empire was working towards, turn by turn. The engine hands
// over each AI seat's own decision log once the game is finished (`Game.ai_timeline`, never during play);
// this file turns it into the sentences the report shows. Nothing here guesses a motive from what a rival did.
import type { PostGame, ReportEmpire } from './postgame';
import type { PlayerId, PlayerView } from './types';

export interface AiCityRef { id: number; name: string | null; owner: PlayerId | null }
export type AiPlanKind = 'economy' | 'peace' | 'campaign' | 'contain' | 'snipe' | 'field_strike' | 'hold';
export type AiPlanReason = 'start' | 'better' | 'target_gone' | 'captured' | 'peace_signed' | 'campaign_ended' | 'settler_taken' | 'snipe_missed' | 'war_illegal' | 'infeasible' | 'search';
export type AiCampaignEnd = 'captured' | 'negative_value' | 'insufficient_army' | 'walls_before_arrival' | 'target_gone' | 'peace' | 'deadline' | 'plan_changed';
export interface AiPlanStep { turn: number; plan: AiPlanKind; against: PlayerId[]; city: AiCityRef | null; reason: AiPlanReason; /** 0..=10,000 */ chance: number }
export interface AiCampaignStep { turn: number; city: AiCityRef; end: AiCampaignEnd | null }
export type AiThreatStep =
  | { kind: 'top_threat'; turn: number; rival: PlayerId }
  | { kind: 'walls'; turn: number; city: AiCityRef; rival: PlayerId }
  | { kind: 'treaty'; turn: number; with: PlayerId[]; treaty: 'Peace' | 'Alliance'; accepted: boolean; rival: PlayerId }
  | { kind: 'second_front'; turn: number; against: PlayerId[]; rival: PlayerId };
export interface AiSeatTimeline { player: PlayerId; plans: AiPlanStep[]; campaigns: AiCampaignStep[]; threats: AiThreatStep[] }
export interface AiTimeline { seats: AiSeatTimeline[] }

type Tone = 'mine' | 'against' | 'neutral';
export interface RivalEntry {
  turn: number;
  icon: string;
  text: string;
  /** Aimed at the viewer's team: shown in the warning tone. */
  tone: Tone;
  /** Other empires the line names, for their portraits. */
  who: PlayerId[];
  /** The rival's own estimate of its chance to win when it changed plans, in percent. */
  chance?: number;
}
export interface RivalStory {
  empire: ReportEmpire;
  entries: RivalEntry[];
  /** Lines aimed at the viewer's team. */
  againstYou: number;
  marches: number;
  taken: number;
  /** One sentence for the head of the story. */
  summary: string;
}

/** The engine's record, or null when there is none (an unfinished game, an online game, an older engine). */
export function parseAiTimeline(json: string | null | undefined): AiTimeline | null {
  if (!json) return null;
  try {
    const t = JSON.parse(json) as AiTimeline;
    return Array.isArray(t?.seats) ? t : null;
  } catch {
    return null;
  }
}

const REASON: Partial<Record<AiPlanReason, string>> = {
  target_gone: 'once its target was gone',
  captured: 'after taking its target',
  peace_signed: 'once peace was signed',
  campaign_ended: 'once its campaign was over',
  settler_taken: 'after catching the Settler',
  snipe_missed: 'after the Settler got away',
  war_illegal: 'because it could not declare war',
  infeasible: 'because its last plan could no longer work',
};
const ENDED: Record<Exclude<AiCampaignEnd, 'captured'>, string> = {
  negative_value: 'it was no longer worth the cost',
  insufficient_army: 'its army was too small',
  walls_before_arrival: 'Walls went up before it arrived',
  target_gone: 'the city was gone or had changed hands',
  peace: 'peace was signed',
  deadline: 'it was taking too long',
  plan_changed: 'it changed plans',
};

const list = (names: string[]) => names.length <= 1 ? names[0] ?? 'a rival' : `${names.slice(0, -1).join(', ')} and ${names.at(-1)}`;

export function rivalStories(view: PlayerView, pg: PostGame, timeline: AiTimeline | null): RivalStory[] {
  if (!timeline) return [];
  const byId = new Map(pg.empires.map((e) => [e.id, e]));
  const mine = (ids: PlayerId[]) => ids.some((id) => byId.get(id)?.team === view.team);
  // "you" inside a sentence; the empire's name otherwise.
  const name = (id: PlayerId) => { const e = byId.get(id); return !e ? `Empire ${id + 1}` : e.mine ? 'you' : e.name; };
  const names = (ids: PlayerId[]) => list(ids.map(name));
  const cityOf = (c: AiCityRef, owners: PlayerId[] = []) => c.name ?? (owners.length ? `a city of ${names(owners).replace(/^you$/, 'yours')} (since razed)` : 'a city since razed');

  const stories: RivalStory[] = [];
  for (const seat of timeline.seats) {
    const empire = byId.get(seat.player);
    if (!empire) continue;
    const entries: (RivalEntry & { order: number })[] = [];
    const aim = (ids: PlayerId[]): Tone => mine(ids) ? 'against' : 'neutral';
    // Who a marched-on city belonged to: the plan that named it says.
    const defenders = new Map<number, PlayerId[]>();

    for (const p of seat.plans) {
      const who = p.against.filter((id) => id !== seat.player);
      if (p.city && who.length) defenders.set(p.city.id, who);
      const why = REASON[p.reason];
      let icon = 'emblem-growth', text: string, tone: Tone = 'neutral';
      switch (p.plan) {
        case 'economy': text = p.reason === 'start' ? 'Began by growing its own empire' : 'Went back to growing its own empire'; break;
        case 'peace': icon = 'treaty-peace'; text = `Decided to seek peace with ${names(who)}`; break;
        case 'campaign': icon = 'emblem-captured'; tone = aim(who); text = `Set out to take ${p.city ? cityOf(p.city, who) : 'a city'}${p.city?.name ? ` from ${names(who)}` : ''}`; break;
        case 'contain': icon = 'unit-spearman'; tone = aim(who); text = `Dug in to hold ${names(who)} back`; break;
        case 'snipe': icon = 'unit-spearman'; tone = aim(who); text = `Sent a raid after a Settler of ${names(who).replace(/^you$/, 'yours')}`; break;
        case 'field_strike': icon = 'unit-spearman'; tone = aim(who); text = `Went after the army of ${names(who).replace(/^you$/, 'yours')} in the field`; break;
        case 'hold': icon = 'city'; text = `Pulled back to defend ${p.city ? cityOf(p.city) : 'a city'}`; break;
      }
      entries.push({ turn: p.turn, order: 1, icon, text: why && p.reason !== 'start' ? `${text}, ${why}` : text, tone, who, chance: Math.round(p.chance / 100) });
    }

    let marches = 0, taken = 0;
    const planned = (turn: number) => seat.plans.filter((p) => p.turn <= turn).at(-1);
    for (const c of seat.campaigns) {
      const who = defenders.get(c.city.id) ?? [];
      if (c.end === null) {
        marches++;
        // The plan line already says so when the plan named this city on or before this turn.
        const p = planned(c.turn);
        if (p?.plan === 'campaign' && p.city?.id === c.city.id) continue;
        entries.push({ turn: c.turn, order: 2, icon: 'emblem-captured', text: `Marched on ${cityOf(c.city, who)}`, tone: aim(who), who });
      } else if (c.end === 'captured') {
        taken++;
        entries.push({ turn: c.turn, order: 0, icon: 'emblem-captured', text: `Took ${cityOf(c.city, who)}`, tone: aim(who), who });
      } else {
        entries.push({ turn: c.turn, order: 0, icon: 'ui-cross', text: `Called off the march on ${cityOf(c.city, who)}: ${ENDED[c.end]}`, tone: 'neutral', who });
      }
    }

    // A rival often asks the same empire again and again; the first ask tells the story.
    const asked = new Set<string>();
    let feared: PlayerId | null = null;
    for (const t of seat.threats) {
      if (t.kind === 'top_threat') {
        feared = t.rival;
        entries.push({ turn: t.turn, order: 0, icon: 'ui-alert', text: `Came to fear ${name(t.rival)} most`, tone: aim([t.rival]), who: [t.rival] });
      } else if (t.kind === 'walls') {
        entries.push({ turn: t.turn, order: 3, icon: 'city', text: `Started Walls in ${cityOf(t.city)} against ${name(t.rival)}`, tone: aim([t.rival]), who: [t.rival] });
      } else if (t.kind === 'treaty') {
        const deal = t.treaty === 'Alliance' ? 'an Alliance' : 'Peace';
        const key = `${t.accepted}:${t.treaty}:${t.with.join(',')}:${t.rival}`;
        if (asked.has(key)) continue;
        asked.add(key);
        entries.push({ turn: t.turn, order: 3, icon: 'treaty-peace', tone: aim([t.rival]), who: t.with,
          text: t.accepted ? `Accepted ${deal} with ${names(t.with)}, wary of ${name(t.rival)}` : `Offered ${names(t.with)} ${deal}, wary of ${name(t.rival)}` });
      } else {
        entries.push({ turn: t.turn, order: 3, icon: 'unit-spearman', text: `Declared war on ${names(t.against)}, the rival it feared most`, tone: aim(t.against), who: t.against });
      }
    }

    entries.sort((a, b) => a.turn - b.turn || a.order - b.order);
    const parts: string[] = [];
    if (feared !== null) parts.push(`Feared ${name(feared)} most at the end`);
    if (marches) parts.push(`${parts.length ? 'marched' : 'Marched'} on ${marches} ${marches === 1 ? 'city' : 'cities'} and took ${taken}`);
    else if (entries.length) parts.push(`${parts.length ? 'never' : 'Never'} marched on a city`);
    stories.push({
      empire, marches, taken,
      entries: entries.map(({ order: _order, ...e }) => e),
      againstYou: entries.filter((e) => e.tone === 'against').length,
      summary: parts.length ? `${parts.join('; ')}.` : 'This rival kept no record of its plans.',
    });
  }
  // A save from before the record was kept loads with every log empty: that is no record at all.
  return stories.some((s) => s.entries.length) ? stories : [];
}
