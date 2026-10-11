//! Every Mago crate, direct and transitive, resolves to the one pinned version (P2R-81).

const PIN: &str = "1.55.0";

#[test]
fn every_mago_crate_uses_the_pinned_version() {
    let lock = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.lock"))
        .expect("Cargo.lock is committed");
    let mut name = "";
    let mut mago = 0;
    let mut wrong = Vec::new();
    for line in lock.lines() {
        if let Some(value) = line.strip_prefix("name = ") {
            name = value.trim_matches('"');
        }
        if let Some(value) = line.strip_prefix("version = ")
            && name.starts_with("mago-")
        {
            mago += 1;
            if value.trim_matches('"') != PIN {
                wrong.push(format!("{name} {value}"));
            }
        }
    }
    assert!(
        mago >= 10,
        "found only {mago} mago crates in Cargo.lock; the parser may be wrong"
    );
    assert!(wrong.is_empty(), "Mago crates off the {PIN} pin: {wrong:?}");
}
