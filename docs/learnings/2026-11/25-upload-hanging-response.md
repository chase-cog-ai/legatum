# Game uploads looked like hanging requests (2026-11-25)

In the browser's network panel every `PUT /v1/games/...` showed status 200 after about 1.5 s, then
`net::ERR_ABORTED` at 30 s. The record was stored, and the page counted it as a success. The cause was on the
page: `gameUpload.ts` checked `response.ok` and never read the body, so the `AbortSignal.timeout(30000)` it
passed to `fetch` later aborted the unread body stream. The page now cancels the body as soon as it has the
status. When a fetch carries a timeout signal, read or cancel the body, or devtools will show a failure that
did not happen.
