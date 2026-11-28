# Measure php2rust on a WordPress corpus that stays outside the repository

php2rust measures its progress on a real code base, a stable WordPress release, read from a local corpus that is never committed. A script downloads the release from wordpress.org and the `wordpress-develop` repository at the same tag, checks the release against the checksum wordpress.org publishes, and writes a manifest with each version, URL, checksum and the date. The corpus lives in `corpus/` at the repository root, which `.gitignore` is to list, or at the path in `PHP2RUST_CORPUS`. The code and the tests of WordPress are GPL, and php2rust is MIT or Apache (P2R-80), so no file of the corpus, and no fixture, folder or subject copied from it, enters the repository; every committed test input is written by hand (ADR 0010).

A second script makes the stop report. It runs php2rust once per file of `wp-includes/`, `wp-admin/` and the root `*.php` files, with `--include` naming the whole release so that every file's classes and functions resolve, and once per target PHP version (ADR 0009). It records each file's exit code (0 converts, 1 stops, 2 an error) and its stop lines, and writes a dated report: the number of files that convert and the number of stops per version, then a table of stop reasons ranked by the number of files they block, each row with its requirement ID, its message without the location, the counts of files and stops, and three example locations. The report also lists the differential fixtures that pass per tier and version (ADR 0010), and the change against the previous report of the same WordPress version. Reports are written to `corpus/reports/` and are not committed; a pull request quotes the counts and the rows it changes.

Facts this rests on, checked on 28 Nov 2026: the latest WordPress release is 7.1.5, its SHA-1 from wordpress.org matched the download, and it holds 1,512 PHP files, `wp-content/` included. It declares `$required_php_version = '7.4'`, so its code is written for PHP from 7.4 up. `wordpress-develop` has a tag `7.1.5`, and GitHub reports its licence as GPL-2.0. This record states facts and a project choice only; it draws no legal conclusion.

A trial run on the same day measured the method on two files with a release build. A file given both as input and inside `--include` is read as input with no error, and one run took 0.26 s (`kses.php`) and 0.77 s (`formatting.php`), so the report over the 1,315 core files takes about ten minutes per version. The trial also showed what the first report will say. Only 81 of the 1,315 core files declare `strict_types`, so P2R-12 stops each of the other 1,234. In the two files, most other stops are parameters and functions with no declared type (P2R-23: 284 lines in `formatting.php` and 113 in `kses.php`). WordPress declares types in docblocks and relies on PHP's conversion of scalar arguments, which P2R-12 exists to rule out. Closing either stop is not a slice of the usual kind: it changes what ADR 0001 lets php2rust assume about its input.

This ADR is not implemented yet: neither script, the manifest nor the `.gitignore` entry exists until the implementing change lands.

## Considered Options

- Commit WordPress, as files or as a git submodule: rejected, because the repository would then hold GPL code next to MIT or Apache code, and a release holds about 1,500 files that no test of the tool needs.
- Fetch WordPress through Composer (`johnpbloch/wordpress`): rejected, because it adds a PHP dependency manager to a Rust project for one download, and the result must stay uncommitted anyway.
- Convert the whole release as one directory input (P2R-170): rejected as the measure, because one stop then stops the whole crate and the report cannot count the files that convert. A directory run can follow once single files convert.
- Include `wp-content/` (the bundled themes and plugins): rejected, because they are not WordPress core and change with each release.

## Consequences

- The corpus and its reports exist only on a machine that ran the script. CI does not fetch the corpus, so a fixture whose subject is a corpus path runs locally only, and the report counts are evidence in a pull request, not a CI check.
- Each report names the WordPress version and the php2rust commit, and two reports are compared only for the same WordPress version.
- A move to a newer WordPress release is its own change: refresh the corpus, then start a new line of reports.
- A test fails when any tracked file lies under `corpus/`, so a copy cannot be committed by mistake.
- The report cannot show progress on WordPress until the owner decides how php2rust treats a file without `strict_types` and a parameter whose type is only in a docblock. Until then it counts the files that stop for each reason, and the differential fixtures of ADR 0010 run on hand-written subjects.

## Spec changes for the change that implements this

New requirements for the corpus script, the manifest, the report and its format, and the check that no tracked file lies under `corpus/`; P2R-80 (the licence, and that the corpus is not part of the repository); P2R-82 (the README names the two scripts and `PHP2RUST_CORPUS`).
