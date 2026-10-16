//! Every requirement ID from P2R-07 to P2R-49 and from P2R-100 to P2R-431 names at least one
//! case or stop folder (P2R-75).

use std::fs;
use std::path::Path;

/// The IDs `**P2R-nn**` that docs/spec.md defines, so a new requirement adds itself.
fn requirement_ids(spec: &str) -> Vec<String> {
    let mut ids: Vec<String> = spec
        .split("**P2R-")
        .skip(1)
        .filter_map(|rest| rest.split_once("**"))
        .filter_map(|(number, _)| number.parse::<u32>().ok())
        .filter(|number| (7..=49).contains(number) || (100..=431).contains(number))
        .map(|number| format!("P2R-{number:02}"))
        .collect();
    ids.dedup();
    ids
}

fn folder_names(root: &Path, kind: &str) -> Vec<String> {
    fs::read_dir(root.join("tests").join(kind))
        .expect("readable test folder")
        .map(|entry| entry.expect("readable test folder").path())
        .filter(|path| path.is_dir())
        .filter_map(|path| Some(path.file_name()?.to_str()?.to_owned()))
        .collect()
}

#[test]
fn every_requirement_has_a_case_or_stop_folder() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec = fs::read_to_string(root.join("docs/spec.md")).expect("docs/spec.md is readable");
    let ids = requirement_ids(&spec);
    assert!(
        ids.len() >= 20,
        "found only {} requirement IDs from P2R-07 to P2R-49 and P2R-100 to P2R-431 in docs/spec.md; the parser may be wrong",
        ids.len()
    );
    let mut folders = folder_names(root, "cases");
    folders.extend(folder_names(root, "stops"));
    let missing: Vec<&String> = ids
        .iter()
        .filter(|id| {
            let prefix = format!("{}-", id.to_lowercase());
            !folders.iter().any(|folder| folder.starts_with(&prefix))
        })
        .collect();
    assert!(
        missing.is_empty(),
        "no case or stop folder name starts with these requirement IDs (P2R-75): {missing:?}"
    );
}
