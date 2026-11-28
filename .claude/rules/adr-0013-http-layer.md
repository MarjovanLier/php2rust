---
paths:
  - "src/**"
  - "tests/differential/**"
---

# ADR 0013: The HTTP layer of a converted app

Rationale: `docs/adr/0013-the-http-layer-of-a-converted-app.md`. The ADR wins on conflict; change this file in the same commit as the ADR.

Not implemented yet, and the runtime dependency waits for the owner: these checks apply to the work that converts routes and controllers.

- Serve with `hyper` and `hyper-util` on `tokio`, at exact versions. Do not add axum or another web framework to the output.
- Write routing, route model binding, validation and response building as plain Rust in the output. Match Laravel's bytes: its 404, 405 and 422 bodies, its validation messages and its `data` wrapping.
- Keep Laravel's pipeline: the 422 of a form request passes the appended middleware, and the 404 of an unknown path, the 405 and the 404 of a missing bound model do not.
- Write JSON with php2rust's own helper, as PHP's `json_encode` writes it for that kind of answer (`\/`, `é`, or the exception handler's pretty print). Do not add `serde_json` or another JSON library to the output.
- A route or rule outside the spec's closed list stops the conversion; never guess its behaviour (ADR 0001).
