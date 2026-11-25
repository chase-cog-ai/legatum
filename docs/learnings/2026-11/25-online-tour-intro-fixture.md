## 2026-11-25: The real-relay tour needs the intro skip in each browser

- `testWithFailingRequests` does not install `skipPeopleIntro` like the regular fixture does. Without it the people's introduction intercepted the host's Ping click even though the turn counter already showed turn one.
- For multiplayer tours using that fixture, install the shared intro skip before navigation in each host and guest page. Disabling first-game hints alone does not dismiss the intro.
