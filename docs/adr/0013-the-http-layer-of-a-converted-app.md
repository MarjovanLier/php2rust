# Serve a converted app with hyper, and convert routing, validation and responses as php2rust's own code

Status: proposed on 28 Nov 2026; it waits for the owner's decision on the runtime dependencies below.

A converted Laravel app must answer each request as the PHP app does (ADR 0012), so the Rust needs an HTTP server, and the parts of Laravel that turn a request into a response (routes, controllers, form requests, validation errors, JSON resources, middleware) need a Rust form. The recommendation is to serve with `hyper` 1.x and `hyper-util` on `tokio`, pinned to exact versions as the output's other crates are (P2R-243, P2R-263), and to have php2rust write the routing, the validation and the response building as plain Rust in the output, matching Laravel's bytes, instead of mapping them onto a web framework's own forms. Adding a runtime dependency to every converted app is the owner's decision; this record states the options and the measurements.

Recommended forms, for the spec to fix one slice at a time:

- `routes/api.php`: each `Route::get|post|put|patch|delete(path, [Controller::class, 'method'])` becomes one arm of a generated `match` on the method and the path segments, under the `/api` prefix that `withRouting(api: ..)` gives. A path that matches no route answers Laravel's 404, and a known path with another method Laravel's 405, each with the body Laravel gives.
- Route model binding (`{project}` with a parameter typed `Project`): a `find` on the store (ADR 0014), and Laravel's 404 with `No query results for model [App\Models\Project] 9` when it finds nothing (P2R-421 already holds that message).
- A controller method becomes a function from the request and its bound models to a response: a status, headers and a body.
- A form request's `rules()` becomes validation code over the request's JSON for a closed list of rules (`required`, `nullable`, `string`, `integer`, `boolean`, `min`, `max` to start), with Laravel's English messages and its 422 body: `message`, the first error with `(and N more error)` or `errors`, and `errors` by field.
- A JSON resource's `toArray()` becomes a function to a JSON object, wrapped in `data`; a collection wraps a list.
- A middleware's `handle()` becomes a function around the next handler, in the order `bootstrap/app.php` appends them. Which answers pass it follows Laravel's pipeline, as measured below: the 422 of a form request does, and the 404 of an unknown path, the 405 of a wrong method and the 404 of a missing bound model do not.
- JSON is written by php2rust's own helper (P2R-160), not a library, because the bytes must be those of PHP's `json_encode` with the flags Laravel uses for each kind of answer: a `JsonResponse` escapes `/` as `\/` and a non-ASCII letter as `é`, and the exception handler pretty-prints with four spaces; a library such as `serde_json` writes neither form by default.

Facts this rests on, checked on 28 Nov 2026 on macOS with rustc 1.100.0, each a server that answers `GET /api/projects` with `{"data":[]}` and 404 otherwise, built from a clean target after `cargo fetch`:

| Server | Versions | Crates in the lock | Clean debug build | Clean release build | Release binary |
|---|---|---|---|---|---|
| std only (`TcpListener`) | none | 1 | 0.56 s | 0.35 s | 448 KB |
| hyper | hyper 1.13.0, hyper-util 0.1.21, http-body-util 0.1.5, tokio 1.54.1 | 26 | 3.31 s | 4.05 s | 1.2 MB |
| axum | axum 0.8.9, tokio 1.54.1 | 52 | 5.27 s | 6.49 s | 1.6 MB |

All three answered 200 with the body and 404 for another path. The probes compare the servers alone: the hyper probe writes its body as a string and pulls in no JSON library, and the axum probe was built with axum's default features, which include its JSON extractor and `serde_json`; with JSON written by php2rust's own helper, hyper's 26 crates are the whole server.

The demo app under PHP 8.5.13, from a clean export of the committed app (ADR 0012), whole responses with `Date`, `Host`, `Connection`, `X-Powered-By`, `Cache-Control` and `Access-Control-Allow-Origin` left out:

| Request | Status | `X-Demo-App` | Body |
|---|---|---|---|
| `POST /api/projects` with the name `a/b é` | 201 | yes | `{"data":{"id":1,"name":"a\/b é","budget":5,"open_tasks":0}}` |
| `GET /api/nothing` | 404, status line `HTTP/1.0` | no | `{"message": "The route api/nothing could not be found."}`, pretty-printed |
| `DELETE /api/projects` | 405, status line `HTTP/1.0`, `Allow: GET, HEAD, POST` | no | `{"message": "The DELETE method is not supported for route api/projects. Supported methods: GET, HEAD, POST."}`, pretty-printed |
| `POST /api/projects/1/tasks` with `{"hours":0}` | 422 | yes | `{"message":"The title field is required. (and 1 more error)","errors":{"title":["The title field is required."],"hours":["The hours field must be at least 1."]}}` |
| `GET /api/projects/9` | 404 | no | `{"message": "No query results for model [App\\Models\\Project] 9"}`, pretty-printed |

Not checked: the flags of `json_encode` for each kind of answer beyond the five above (floats, empty objects against empty lists); keep-alive and chunked request bodies under `php -S`; and the build cost of a converted app with many routes.

This ADR is not implemented yet.

## Considered Options

- axum: its `Router`, extractors and tower middleware look like Laravel's routes, form requests and middleware, but each default answer differs from Laravel's (an empty 404 body, its own 405 and 415 answers, its JSON rejection messages), so php2rust would have to override each one, and the output would read as axum code with Laravel's behaviour bolted on. It doubles the crates of hyper (52 against 26) and the build time.
- A std-only server: rejected, because php2rust would then parse HTTP/1.1 itself (chunked bodies, keep-alive, header folding and limits), which is security-relevant code that hyper already gets right; the one-route probe skipped all of it.
- Convert Laravel's own routing and HTTP classes from `vendor/`: rejected, because that code is far outside what php2rust converts (ADR 0011's trial on WordPress and the demo app's stop report both show framework code stopping), and it would make the output a port of the framework.

## Consequences

- Every converted app depends on `hyper`, `hyper-util`, `http-body-util` and `tokio` at exact versions, as maps depend on `indexmap` (P2R-243), and on no JSON library; the README names them, and the support project of the tests builds them.
- php2rust owns the byte form of every Laravel answer it converts (404, 405, 422, resource wrapping), so each needs a folder or fixture that compares it with PHP's.
- Routes outside the closed list (closures as handlers, `Route::resource`, groups with their own middleware, named routes, `redirect`) stop until a slice adds them.

## Spec changes for the implementing work

A new section for the HTTP layer: the route table, route model binding, controller signatures, the response struct, validation rules and messages, resources and middleware, each with requirement IDs; P2R-243's list of output crates; P2R-82 for the README.
