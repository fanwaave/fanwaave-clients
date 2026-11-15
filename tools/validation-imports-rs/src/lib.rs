#![forbid(unsafe_code)]

use std::{fs, path::Path};

const REQUIRED: [&str; 4] = [
    "@fanwaave/fanwaave-validation",
    "fanwaave-validation",
    "github.com/fanwaave/fanwaave-lib-core/validation/golang",
    "fanwaave_validation",
];
const FORBIDDEN: [&str; 3] = [
    "fanwaave-validation-server",
    "golang-server",
    "fanwaave_validation_server",
];

pub fn validate_contents(all_text: &str) -> Result<(), String> {
    for dependency in REQUIRED {
        if !all_text.contains(dependency) {
            return Err(format!("missing public lib-core import: {dependency}"));
        }
    }
    for forbidden in FORBIDDEN {
        if all_text.contains(forbidden) {
            return Err(format!(
                "client imported server-only validation package: {forbidden}"
            ));
        }
    }
    Ok(())
}

fn collect_files(folder: &Path, contents: &mut String) -> Result<(), String> {
    let entries = fs::read_dir(folder).map_err(|e| format!("{}: {e}", folder.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("{}: {e}", folder.display()))?;
        let file_type = entry.file_type().map_err(|e| e.to_string())?;
        if file_type.is_dir() {
            collect_files(&entry.path(), contents)?;
        } else if file_type.is_file() {
            let bytes = fs::read(entry.path()).map_err(|e| e.to_string())?;
            contents.push_str(&String::from_utf8_lossy(&bytes));
            contents.push('\n');
        }
    }
    Ok(())
}

pub fn verify_directory(root: &Path) -> Result<(), String> {
    let mut contents = String::new();
    collect_files(root, &mut contents)?;
    validate_contents(&contents)
}
