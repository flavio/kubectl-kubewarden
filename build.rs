use std::{fs, path::Path};

use wasi_outbound_http_defs::WIT_FILES;

const WIT_DIRECTORY: &str = "wit/ephemeral/*";

fn main() {
    println!("cargo:rerun-if-changed={}", WIT_DIRECTORY);

    dump_wit_files();
}

fn dump_wit_files() {
    for (name, contents) in WIT_FILES.iter() {
        let target = Path::new("wit").join("ephemeral").join(name);
        fs::write(target, contents).unwrap();
    }
}
