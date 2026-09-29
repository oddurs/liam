use std::process::ExitCode;

// musl's allocator serialises on a global lock, which a thread-per-core server
// would contend on constantly.
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> ExitCode {
    match std::env::args().nth(1).as_deref() {
        Some("--version" | "-V") => {
            println!("liamd {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("liamd: nothing to serve yet; see ROADMAP.md");
            ExitCode::from(1)
        }
    }
}
