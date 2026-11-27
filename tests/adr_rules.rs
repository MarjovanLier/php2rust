//! Each decision record in docs/adr has a rule file in .claude/rules that carries its checks
//! to an agent, and each such rule file has its record, so the two sets cannot drift apart.

use std::fs;
use std::path::Path;

/// The four-digit numbers after `prefix` at the start of the `.md` file names in `dir`.
fn numbers(dir: &Path, prefix: &str) -> Vec<String> {
    let mut numbers: Vec<String> = fs::read_dir(dir)
        .expect("readable folder")
        .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
        .filter(|name| name.ends_with(".md"))
        .filter_map(|name| name.strip_prefix(prefix)?.get(..4).map(str::to_owned))
        .filter(|number| number.bytes().all(|byte| byte.is_ascii_digit()))
        .collect();
    numbers.sort();
    numbers
}

#[test]
fn every_adr_has_a_rule_file_and_every_rule_file_its_adr() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let adrs = numbers(&root.join("docs/adr"), "");
    assert!(
        !adrs.is_empty(),
        "found no ADR in docs/adr; the file name parser may be wrong"
    );
    let rules = numbers(&root.join(".claude/rules"), "adr-");
    let missing: Vec<&String> = adrs.iter().filter(|n| !rules.contains(n)).collect();
    assert!(
        missing.is_empty(),
        "these ADRs have no .claude/rules/adr-NNNN-*.md file: {missing:?}"
    );
    let orphans: Vec<&String> = rules.iter().filter(|n| !adrs.contains(n)).collect();
    assert!(
        orphans.is_empty(),
        "these .claude/rules/adr-NNNN-*.md files have no ADR in docs/adr: {orphans:?}"
    );
}
