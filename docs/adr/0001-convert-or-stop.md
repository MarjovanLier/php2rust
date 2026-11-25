# Convert only what is known to behave the same; stop on everything else

php2rust writes Rust only when every construct of the input converts, and a converted program must print what the PHP prints (P2R-07, P2R-72). Any other construct stops the conversion with a `path:line:column: message` line and exit 1, and no output file is written (P2R-03, P2R-05). We chose this over a best-effort output with gaps or guesses, because a user cannot review a translation for silent changes in behaviour, and each stop names what to add to the tool next. The tool uses no AI for the same reason: the same input must always give the same bytes (P2R-06).

## Consequences

- A refusal is the expected result for unsupported PHP, not an error, and `docs/spec.md` lists which forms convert.
- Where PHP and Rust differ in a way that cannot be closed, the difference is documented as a known difference (P2R-60), not converted silently.
