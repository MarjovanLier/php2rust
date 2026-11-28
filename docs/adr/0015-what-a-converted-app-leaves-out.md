# Leave the stateful and rendering parts of Laravel out, read config and env as Laravel does, and convert an app whole or not at all

Status: proposed on 28 Nov 2026; the scope of ADR 0001 for an app is the owner's to decide.

A converted app answers JSON requests (ADRs 0012 to 0014). Most of the rest of Laravel keeps state outside the request or renders HTML, and php2rust converts none of it: Blade views, sessions and cookies, authentication and authorisation (guards, policies, gates), queues and jobs, events and listeners (other than the observers of P2R-460), mail and notifications, the cache, artisan commands and scheduling, broadcasting, the filesystem and storage, localisation beyond the English validation messages of ADR 0013, and service providers other than the app's own `bootstrap/app.php`. A file or route that uses one of them stops the conversion (ADR 0001), with a message that names the part, so the stop report of ADR 0012 shows what each app needs next.

Config and env, for the spec to fix:

- A config file that returns an array of literals and `env()` calls (the demo app's `config/demo.php` returns `['name' => env('DEMO_NAME', 'demo')]`) becomes a Rust struct that the app reads once at start, and `config('demo.name')` a read of its field. Any other code in a config file stops.
- `env('KEY', default)` must give what Laravel's `Env::get` gives: an unset variable gives the default; the values `true`, `false`, `empty` and `null`, in any letter case and with or without parentheses (`(true)`), give `true`, `false`, `''` and `null`; a value inside one pair of matching single or double quotes loses them; any other value stays the string. Laravel also reads `.env` through `vlucas/phpdotenv`; the converted app reads the process environment only, so a `.env` file stops until a slice reads it.
- The app's `bootstrap/app.php` is read, not converted: its `withRouting(api: ..)` gives the route file and the `/api` prefix, and `withMiddleware(..)` the order of middleware (ADR 0013). Any other call there stops.

Open decision for the owner, which this record does not take: does ADR 0001's rule (convert all or write nothing) apply to the whole app or per file?

- Whole app (recommended): the app converts only when every file and route converts, as a directory input already does (P2R-170). A user gets a server only when every request it answers behaves as PHP's.
- Per route, beside PHP: the Rust serves the routes that convert and passes the others to the PHP app, so an app moves over route by route. This changes ADR 0001, because the output is then half of an app, and it needs a proxy and a shared database.
- Per file, as single-file inputs are today: the files that convert are written and the rest are not, which gives a user Rust they cannot run as a server.

Facts this rests on, checked on 28 Nov 2026: Laravel 13.44.1's `Illuminate\Support\Env::getOption` maps the four words and strips the quotes as above (read in `vendor/laravel/framework/src/Illuminate/Support/Env.php`). The demo app's stop report (ADR 0012) stops `config/demo.php` and `bootstrap/app.php` on P2R-07 (`return` outside a function), because no rule reads them yet.

Not checked: which of the excluded parts the owner's own apps use most, and `config:cache`, which makes Laravel read a cached PHP file in place of `config/*.php`.

This ADR is not implemented yet.

## Considered Options

- Convert sessions, cookies and auth in the first app slices: rejected, because they keep state that the PHP and Rust sides would have to share byte for byte (session ids, encrypted cookies under `APP_KEY`), and the demo app needs none of them to answer its routes.
- Convert Blade: rejected, because a view is a second language with its own compiler, and an API app answers JSON.
- Read `.env` in the converted app from the start: rejected for now, because phpdotenv's quoting, interpolation and comment rules would be a second parser to match; the process environment is what `php -S` and a container give both sides.

## Consequences

- The first apps php2rust converts are JSON APIs without auth; an app with a login page stops, and says why.
- Each excluded part that a later slice adds gets its own decision; none comes in silently.
- Until the owner decides the scope question, the work converts towards the whole-app rule, which needs no change to ADR 0001.

## Spec changes for the implementing work

A requirement per excluded part and its stop message, requirements for config files, `env()` and `bootstrap/app.php`, and, once the owner decides, the scope of ADR 0001 for an app (P2R-170).
