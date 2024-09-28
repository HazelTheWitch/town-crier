use std::{env, fs::File, io::{BufWriter, Write}, path::PathBuf, process::Command};

const VERSION_ORDER: &[&str] = &["MAJOR", "MINOR", "PATCH"];

fn main() {
    let rustc = env::var("RUSTC").unwrap();

    let rustc_output = String::from_utf8(
        Command::new(rustc)
            .arg("-v")
            .arg("-V")
            .output()
            .expect("could not get rustc version")
            .stdout
    ).unwrap();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut info = BufWriter::new(File::create(out_dir.join("info.rs")).unwrap());

    for line in rustc_output.lines() {
        let line = line.trim();

        if let Some(release) = line.strip_prefix("release: ") {
            for (i, s) in release.split('.').enumerate().take(3) {
                writeln!(info, r#"pub const RUSTC_{}: &str = "{s}";"#, VERSION_ORDER[i]).unwrap();
            }
        }

        if let Some(release) = line.strip_prefix("LLVM version: ") {
            for (i, s) in release.split('.').enumerate().take(3) {
                writeln!(info, r#"pub const LLVM_{}: &str = "{s}";"#, VERSION_ORDER[i]).unwrap();
            }
        }

        if let Some(hash) = line.strip_prefix("commit-hash: ") {
            writeln!(info, r#"pub const RUSTC_COMMIT: &str = "{hash}";"#).unwrap();
        }
    }

    for (key, value) in ["OPT_LEVEL", "DEBUG", "NUM_JOBS", "TARGET", "HOST"]
        .iter()
        .map(|&name| (name, env::var(name).unwrap())) {
        writeln!(info, r#"pub const {key}: &str = "{value}";"#).unwrap();
    }
}
