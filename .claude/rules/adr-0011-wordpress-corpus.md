---
paths:
  - ".gitignore"
  - "scripts/*.sh"
  - "tests/**"
---

# ADR 0011: A WordPress corpus outside the repository

Rationale: `docs/adr/0011-wordpress-corpus-outside-the-repository.md`. The ADR wins on conflict; change this file in the same commit as the ADR.

Not implemented yet: the scripts, the manifest and the `.gitignore` entry come with the implementing change.

- Never commit a file from the corpus, and never copy WordPress code or WordPress test data into `tests/`. Write every committed input by hand.
- Read the corpus from `corpus/` or from `PHP2RUST_CORPUS`. Fetch it only with the corpus script, which checks the wordpress.org checksum and writes the manifest.
- Run the stop report per file over `wp-includes/`, `wp-admin/` and the root `*.php` files, with `--include` naming the whole release, once per target PHP version. Leave `wp-content/` out.
- Compare two reports only for the same WordPress version, and name the php2rust commit in each.
- Reports stay in `corpus/reports/`; a pull request quotes their counts.
