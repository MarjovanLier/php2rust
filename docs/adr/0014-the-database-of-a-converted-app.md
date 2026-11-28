# Give a converted app SQLite through rusqlite, the engine its PHP oracle runs on

Status: proposed on 28 Nov 2026; it waits for the owner's decision on the database, its runtime dependency and the schema option below.

Today a converted model reads and writes an in-memory store that the output holds (P2R-410): it starts empty in each process, numbers rows in creation order and keeps nothing after the process ends. A converted app is a server that answers many requests, whose PHP side keeps its rows in a database between them (ADR 0012), so the store must become a database, or the Rust and PHP sides of an `http` fixture stop agreeing from the second request that reads what the first wrote. The recommendation is SQLite through `rusqlite` with its bundled SQLite, pinned to an exact version: the Rust then runs the same database engine as the PHP oracle, so ids, the order of rows, `NULL` and the stored form of a boolean (`tinyint(1)`) are SQLite's on both sides, and the Rust issues the SQL that Eloquent issues for the forms php2rust converts. Which database a converted app uses, and the new runtime dependency, are the owner's decision.

How migrations become a schema, for the spec to decide:

- php2rust already reads the migrations for each table's columns (P2R-400). The demo app's two migrations give, on SQLite, `create table "projects" ("id" integer primary key autoincrement not null, "name" varchar not null, "budget" integer not null)` and `create table "tasks" ("id" integer primary key autoincrement not null, "project_id" integer not null, "title" varchar not null, "done" tinyint(1) not null, "hours" integer)`, as `php artisan migrate --pretend` printed them.
- Option one: the Rust app opens a database that `php artisan migrate` created, so the schema has one source, Laravel's migrations, but a deployment still needs PHP once. Option two: php2rust writes the same `create table` statements from the migrations into the output, which copies Laravel's SQLite schema grammar for the closed list of column types P2R-403 knows. The first is smaller; the second makes the Rust app stand alone.
- Eloquent's transactions (`DB::transaction`), `lockForUpdate` and events around a save stop until a slice converts them.

Facts this rests on, checked on 28 Nov 2026 on macOS with rustc 1.100.0, each a program that creates the `projects` table in memory, inserts two rows and prints them in id order, built from a clean target after `cargo fetch`:

| Library | Versions | Crates in the lock | Clean debug build | Output |
|---|---|---|---|---|
| rusqlite, bundled SQLite | rusqlite 0.43.0 | 23 | 3.18 s | `1 Roof 500`, `2 Door 80` |
| sqlx, SQLite | sqlx 0.9.1, tokio 1.54.1 | 151 | 8.77 s | the same |

The demo app's tasks table stores `done` as `tinyint(1)`, and the model's `boolean` cast makes it `true` or `false` in JSON (the PATCH response `{"id":1,"title":"Tiles","done":true}`).

Not checked: rusqlite's bundled SQLite version against the one PHP 8.3 to 8.6 link (`SQLite3::version()`), which decides features such as `RETURNING`; how rusqlite's synchronous calls sit inside hyper's async handlers (one connection behind a mutex, or `spawn_blocking`); and MySQL at all.

This ADR is not implemented yet.

## Considered Options

- Keep the in-memory store: rejected for an app, because the rows of one request are gone for the next process and are never shared between server processes, while PHP's are in the database; it stays right for a single program (P2R-410).
- sqlx for SQLite and MySQL: rejected for now. It is async and offers MySQL, but its lock holds 151 crates against rusqlite's 23 and it built in 8.77 s against 3.18 s, and MySQL would need a MySQL oracle in CI before any converted query could be trusted on it.
- MySQL first, as most production Laravel apps run: rejected for now for the same reason; it can follow as its own decision once SQLite converts, with a MySQL service in CI.

## Consequences

- Every converted app with a model depends on `rusqlite` at an exact version, which builds SQLite's C sources, so the output needs a C compiler at build time.
- The PHP oracle and the Rust share one engine, so a difference in an `http` fixture is a conversion error, not an engine difference.
- A converted app does not run on MySQL until a later decision; a Laravel app that needs MySQL-only SQL stops.

## Spec changes for the implementing work

P2R-410 and section 42 (the store becomes SQLite), P2R-400 and P2R-403 (the schema), a requirement for the database path a converted app opens, P2R-243's list of output crates and P2R-82.
