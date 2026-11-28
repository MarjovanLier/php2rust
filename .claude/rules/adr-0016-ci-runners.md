---
paths:
  - ".github/**"
---

# ADR 0016: CI on the owner's server-140 runners

Rationale: `docs/adr/0016-ci-on-the-owners-runners.md`. The ADR wins on conflict; change this file in the same commit as the ADR.

- Every job chooses its runner with `github.event.pull_request.head.repo.fork && 'ubuntu-latest' || fromJSON('["self-hosted","linux","x64","php2rust"]')`. A job that names the self-hosted labels without that test would run a fork's code on the owner's server.
- Keep Actions' approval policy at `all_external_contributors`; do not loosen it while the repository is public and has self-hosted runners.
- Run the tests inside the image of `.github/ci/Dockerfile`, built in the job, so a hosted and a self-hosted run do the same steps.
- Pass every value from the event to a script through `env`, never inline in `run:`, as `conventional.yml` does for the title.
- A job that fails on a missing tool on server-140 is fixed in the Dockerfile, not by installing it on the server.
