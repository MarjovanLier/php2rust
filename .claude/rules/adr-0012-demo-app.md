---
paths:
  - "tests/differential/**"
---

# ADR 0012: A demo Laravel app written by hand, tested over HTTP

Rationale: `docs/adr/0012-a-demo-laravel-app-tested-over-http.md`. The ADR wins on conflict; change this file in the same commit as the ADR.

Not implemented yet: these checks apply to the work that adds the `http` case and to work after it.

- The demo app lives in `tests/differential/apps/demo`. Write its code by hand; never copy a file from another app or from the `laravel/laravel` skeleton.
- Pin its packages in its own `composer.lock` with the platform at PHP 8.3.0. Never commit its `vendor/`, `storage/`, `bootstrap/cache/*.php` or SQLite files.
- An `http` case compares the status, the headers it names and the body, PHP first. Never compare `Date`, `X-Powered-By` or `Host`, which change by run and build.
- Run the PHP side from a new SQLite file migrated with `php artisan migrate`, through `php -S` with `public/index.php`, and run a fixture's cases in order against one server.
- A slice that changes what the demo app's files give quotes the stop report again in its PR.
