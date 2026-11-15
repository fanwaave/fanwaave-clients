#![forbid(unsafe_code)]

use std::path::Path;

const REQUIRED: [&str; 4] = [
    "@fanwaave/fanwaave-validation",
    "fanwaave-validation",
    "github.com/fanwaave/fanwaave-lib-core/validation/golang",
    "fanwaave_validation",
];

fn safe_text() -> String {
    REQUIRED.join("\n")
}

#[test]
fn admits_all_public_consumer_package_names() {
    assert!(fanwaave_validation_import_check::validate_contents(&safe_text()).is_ok());
}
#[test]
fn rejects_missing_public_imports_individually() {
    for missing in REQUIRED {
        let input = REQUIRED
            .iter()
            .filter(|dependency| **dependency != missing)
            .copied()
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            fanwaave_validation_import_check::validate_contents(&input).is_err(),
            "should reject missing {missing}"
        );
    }
}
#[test]
fn rejects_server_only_package_names() {
    for forbidden in [
        "fanwaave-validation-server",
        "golang-server",
        "fanwaave_validation_server",
    ] {
        let input = format!("{}\n{forbidden}", safe_text());
        assert!(fanwaave_validation_import_check::validate_contents(&input).is_err());
    }
}
#[test]
fn inspects_actual_repository_consumer_sources() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../validation-consumer");
    assert!(fanwaave_validation_import_check::verify_directory(&root).is_ok());
}
#[test]
fn missing_consumer_tree_fails_closed() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("nonexistent-consumer-tree");
    assert!(fanwaave_validation_import_check::verify_directory(&root).is_err());
}
