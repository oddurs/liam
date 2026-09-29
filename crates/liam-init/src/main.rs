use std::process::ExitCode;

// No custom allocator: PID 1 barely allocates, and every byte counts against the
// size budget.
fn main() -> ExitCode {
    match std::env::args().nth(1).as_deref() {
        Some("--version" | "-V") => {
            println!("liam-init {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("liam-init: nothing to start yet; see ROADMAP.md");
            ExitCode::from(1)
        }
    }
}
