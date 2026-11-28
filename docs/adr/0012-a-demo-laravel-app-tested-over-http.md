# Test a whole Laravel app through a demo app written by hand, over HTTP

php2rust's goal for Laravel is a converted app that answers every HTTP request as the PHP app does, on PHP 8.3 to 8.6 (ADR 0009). The harness of ADR 0010 calls functions and runs programs; it cannot yet send a request. So the repository gets a small public Laravel app, `tests/differential/apps/demo`, written by hand for php2rust and never copied from another app, and the e2e tier of the harness gets an `http` case: a method, a path, headers and a body, sent to the PHP app and to the Rust server, comparing the status, the headers the case names and the body, PHP first (P2R-604).

The demo app holds the parts a real app has: `routes/api.php` with five routes, two controllers, two form requests with validation rules, two models with migrations and a `hasMany`/`belongsTo` pair, a laravel-data class with `Optional` properties for a partial update, a JSON resource, one middleware and one config value (`config/demo.php`, sent by the middleware as a header). It runs on `laravel/framework` v13.44.1 and `spatie/laravel-data` 4.23.1, pinned by its own `composer.lock` with the platform set to PHP 8.3.0 (84 packages), apart from the harness's lock of ADR 0010, because an app's lock belongs to the app. Its `vendor/`, `storage/`, `bootstrap/cache/*.php` and SQLite files are not committed.

The PHP side of an `http` case runs the app as a user would: a new SQLite file, `php artisan migrate`, then `php -S` with `public/index.php` as the router, with `DB_CONNECTION=sqlite`, `APP_ENV=testing` and `APP_DEBUG=false` in the environment. The cases of one fixture run in order against one server, so a case can read what an earlier one wrote, as a user's requests do.

Facts this rests on, checked on 28 Nov 2026 with Homebrew's PHP 8.5.13:

- Nine requests (list, create, a validation error, show, a missing id, two task creations, a partial update, list again) gave the same status, `Content-Type`, `X-Demo-App` and body on two runs from a fresh database. `Date` and `X-Powered-By` differ between runs or PHP builds, so a case compares only the headers it names.
- The committed app, exported with `git archive` into an empty directory, installed with `composer install` and migrated, answered the same way; it needed no `storage/` folder for these requests.
- An unmatched path and a known path with another method answer with an `HTTP/1.0` status line under `php -S`, the others with `HTTP/1.1`, so a case compares the status code, not the status line. ADR 0013 lists which answers carry the middleware's header.
- The stop report of the demo app: php2rust run on each PHP file, all four targets by default, with `--include` naming its `vendor/` and `app/` and `--migrations` its migrations, took 23 s with a release build for 17 files, the 15 the app holds and the 2 that `package:discover` writes to `bootstrap/cache/`, which are not counted. 3 of the 15 convert: both models and the Data class. 12 stop, the same for every target (no stop line names a version). The counts are a lower bound, because P2R-03 prints only the outer problem of nested spans, so a class that stops on P2R-130 hides the problems inside it. Per file:

| File | Converts | Stopped by |
|---|---|---|
| `app/Data/TaskChange.php` | yes | |
| `app/Models/Project.php` | yes | |
| `app/Models/Task.php` | yes | |
| `app/Http/Controllers/ProjectController.php` | no | P2R-22 (5 problems) |
| `app/Http/Controllers/TaskController.php` | no | P2R-22 (6 problems) |
| `app/Http/Middleware/AddAppHeader.php` | no | P2R-22 (3 problems) |
| `app/Http/Requests/StoreProjectRequest.php` | no | P2R-130 |
| `app/Http/Requests/StoreTaskRequest.php` | no | P2R-130 |
| `app/Http/Resources/ProjectResource.php` | no | P2R-130, P2R-22 (2 problems) |
| `bootstrap/app.php` | no | P2R-07 |
| `config/demo.php` | no | P2R-07 |
| `database/migrations/2025_01_01_000001_create_projects_table.php` | no | P2R-07 |
| `database/migrations/2025_01_01_000002_create_tasks_table.php` | no | P2R-07 |
| `public/index.php` | no | P2R-48, P2R-43, P2R-07 |
| `routes/api.php` | no | P2R-153 (5 problems) |

Ranked by the files they block:

| Requirement | Files | What stops |
|---|---|---|
| P2R-07 | 5 | `return` outside a function (`bootstrap/app.php`, `config/demo.php`, both migrations) and an expression in `public/index.php` |
| P2R-22 | 4 | a framework type with no Rust type: `Request`, `JsonResponse`, `Response`, `AnonymousResourceCollection`, a closure of them, and the models named from an included file |
| P2R-130 | 3 | a class that extends another: both form requests and the resource |
| P2R-153 | 1 | `Route::get(..)` and the other route calls, static calls of a class outside the file |
| P2R-48 | 1 | `define()` in `public/index.php` |
| P2R-43 | 1 | an expression statement in `public/index.php` |

- Converted as one directory crate (P2R-170), `app/` resolves its own models, and stops on the framework's HTTP types (P2R-22), on the classes that extend `FormRequest` and `JsonResource` (P2R-130), on Mago's `mixed-argument` for `Project::create($request->validated())`, and on a call with more arguments than its function has (P2R-07); one run took 2.05 s.

Not checked: the demo app under PHP 8.3, 8.4 and 8.6 (its lock allows 8.3 and later; only 8.5.13 ran it), and Laravel's behaviour under `php -S` against a server such as nginx with PHP-FPM.

This ADR is not implemented yet: no `http` case, runner or Rust server exists until the implementing work lands.

## Considered Options

- Measure on another app by path, outside the repository, or on both: the owner chose the public demo app on 28 Nov 2026.
- The stock `laravel/laravel` skeleton: rejected, because its files are another app's, and most of them (Blade views, the welcome route, the default user model and its auth) are parts php2rust leaves out (ADR 0015).
- Laravel's own HTTP test client (`$this->getJson(..)` in PHPUnit) as the PHP side: rejected, because it bypasses the HTTP server and the Rust side has no equivalent; a request over a socket is the one interface both share.
- Compare every response header: rejected, because `Date`, `X-Powered-By` and `Host` differ by run and build, so a case names the headers it compares.

## Consequences

- The demo app is the measure of the Laravel goal: the PR of each slice quotes the per-file table again. A slice can raise a count as well as lower it, because clearing an outer problem shows the ones inside it.
- `migrations/` stays read by `--migrations` and is never converted, so its P2R-07 stops do not count against the app; the same holds for `bootstrap/app.php`, `public/index.php` and `config/*.php` once ADR 0013 and ADR 0015 decide how the app starts and reads its config.
- An `http` case needs PHP, Composer, the app's `vendor/` and a free local port; CI already installs Composer (P2R-608).

## Spec changes for the implementing work

New requirements beside P2R-607 for the `http` case (its keys, the server start, the order of cases, the compared headers) and for the demo app's location and lock; P2R-606 for a second Laravel lock; P2R-82 for the README.
