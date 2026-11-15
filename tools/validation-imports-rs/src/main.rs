#![forbid(unsafe_code)]

use std::path::Path;

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../validation-consumer");
    if let Err(error) = fanwaave_validation_import_check::verify_directory(&root) {
        eprintln!("validation consumer import check failed: {error}");
        std::process::exit(1);
    }
    println!("all four client consumers import only public lib-core validation packages (Rust)");
}
