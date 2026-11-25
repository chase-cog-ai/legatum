## 2026-11-25: a new feel event needs its own sound file, so a new moment reuses a semantic call

- Cause: the treaty-signing moment wanted a sound. `app/src/feel/sound.test.ts` requires every `feel.*` name (and every row in `docs/design/haptics.md`) to map to a shipped `.m4a`, and each file to serve exactly one event, so a new `treatySigned` feel name could not borrow an existing file.
- What we did: the moment reuses an existing semantic call (`vpGained`, the "small lift") from `feedbackForTurn`, and `soundHooksForMoments` voices it with the card's tier sting (`sting.notable`), which replaces the semantic sound. The haptics contract gained a second `vpGained` row naming the treaty trigger.
- Also: `Extract<Event, { type: 'PeaceEstablished' }>` is `never` because core events that share a payload are declared as one member with a union `type` (`'WarDeclared' | 'PeaceEstablished'`); narrow with `x.type === '…'` inside a `flatMap` instead.
