# Public lists show no typed text (2026-11-25)

**What:** the online game list showed each room under the name its host typed (cleaned of symbols, checked
against a short word blocklist) and showed each room's build label to players on another version. Both were free
text that any stranger could put in front of every player: a blocklist misses spaced, misspelt or plain-word
insults, and the build label allowed any letters. The list also held at most 64 rooms with 2 per /64, so one free
IPv6 tunnel account (a /48, 65,536 /64s) could fill it.

**Fix:** the relay names every room from its id with two fixed words and a number (`listedName`), drops any typed
name, accepts only the build labels the game makes (`buildOk`), caps rooms per network (IPv4 /24, IPv6 /48) at 8,
and raises the total to 256. The host sees the listed name in the game room.

**Rule:** any text a stranger can make every player see (lobby rows, labels, shared summaries) comes from a fixed
vocabulary chosen by the server, never from a filtered free-text field. A per-address cap needs a per-network cap
beside it when IPv6 is in play.
