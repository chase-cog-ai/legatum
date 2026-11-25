import type { Event, TeamId, WarReason } from './types';

/**
 * Why an empire says it declared war (`Event.WarReasonStated`), in words. `target` is the empire
 * it declared on, or null when that is the reader ("you").
 */
export function warReasonText(reason: WarReason, target: string | null): string {
  const you = target === null;
  switch (reason) {
    case 'VictoryHold': return you ? 'You hold your VP target, and they mean to break your countdown to victory.'
      : `${target} holds its VP target, and they mean to break its countdown to victory.`;
    case 'NearVictory': return you ? 'You are close to victory, and they mean to stop you.' : `${target} is close to victory, and they mean to stop it.`;
    case 'Threat': return you ? 'They see you as the greatest threat to their empire.' : `They see ${target} as the greatest threat to their empire.`;
    case 'FactionGoal': return 'Their faction goal calls for this war.';
    case 'Opportunity': return you ? 'Their army is in place to take one of your cities.' : `Their army is in place to take one of ${target}'s cities.`;
    case 'Conquest': return you ? 'They want your land and think they can win.' : `They want ${target}'s land and think they can win.`;
  }
}

/** The reason `by` stated this turn for its war on `against`, if it gave one. */
export function statedWarReason(events: readonly Event[], by: TeamId, against: TeamId): WarReason | undefined {
  for (const e of events) if (e.type === 'WarReasonStated' && e.by === by && e.against === against) return e.reason;
  return undefined;
}
