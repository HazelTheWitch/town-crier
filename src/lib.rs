include!(concat!(env!("OUT_DIR"), "/info.rs"));

pub fn output_info() {
    println!("rustc: {RUSTC_MAJOR}.{RUSTC_MINOR}.{RUSTC_PATCH} ({RUSTC_COMMIT})");
    println!("llvm: {LLVM_MAJOR}.{LLVM_MINOR}.{LLVM_PATCH}");
    println!("opt_level: {OPT_LEVEL} debug: {DEBUG} num_jobs: {NUM_JOBS} target: {TARGET} host: {HOST}")
}

#[cfg(feature = "tracing")]
pub fn output_info_tracing() {
    use tracing::info;

    info!("rustc: {RUSTC_MAJOR}.{RUSTC_MINOR}.{RUSTC_PATCH} ({RUSTC_COMMIT})");
    info!("llvm: {LLVM_MAJOR}.{LLVM_MINOR}.{LLVM_PATCH}");
    info!("opt_level: {OPT_LEVEL} debug: {DEBUG} num_jobs: {NUM_JOBS} target: {TARGET} host: {HOST}")
}
