---
paths:
  - "src/**"
  - "tests/differential/**"
---

# ADR 0015: What a converted app leaves out

Rationale: `docs/adr/0015-what-a-converted-app-leaves-out.md`. The ADR wins on conflict; change this file in the same commit as the ADR.

Not implemented yet, and the scope of ADR 0001 for an app waits for the owner: these checks apply to the work that converts an app.

- Blade, sessions, cookies, auth, queues, events beyond P2R-460, mail, the cache, artisan, scheduling, broadcasting, storage and other service providers stop the conversion with a message that names the part. Never convert one without its own decision.
- A config file of literals and `env()` calls becomes a struct read at start; any other code in it stops.
- `env()` gives what Laravel's `Env::get` gives: the default when unset, the four words `true`, `false`, `empty` and `null` (any case, with or without parentheses), one pair of matching quotes stripped, otherwise the string. Read the process environment only, not `.env`.
- Read `bootstrap/app.php` for the route file, the prefix and the middleware order; do not convert it.
- Until the owner decides, convert towards the whole-app rule: the app converts only when every file and route converts.
