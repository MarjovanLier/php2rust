---
paths:
  - "src/**"
  - "tests/differential/**"
---

# ADR 0014: The database of a converted app

Rationale: `docs/adr/0014-the-database-of-a-converted-app.md`. The ADR wins on conflict; change this file in the same commit as the ADR.

Not implemented yet, and the database waits for the owner: these checks apply to the work that replaces the store of P2R-410 for an app.

- A converted app reads and writes SQLite through `rusqlite` with its bundled SQLite, at an exact version. Do not add sqlx or a MySQL driver.
- Issue the SQL that Eloquent issues for each converted form, so ids, row order, `NULL` and stored booleans are SQLite's on both sides.
- A single program keeps the in-memory store of P2R-410; only an app gets the database.
- Transactions, locks and model events stop until a slice converts them.
