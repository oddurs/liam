use std::process::ExitCode;

fn main() -> ExitCode {
    let arg = std::env::args().nth(1);
    match arg.as_deref() {
        Some("--version" | "-V") => {
            println!("{}", version_line());
            ExitCode::SUCCESS
        }
        None | Some("--help" | "-h") => {
            println!("{}\n\n{}", version_line(), USAGE);
            ExitCode::SUCCESS
        }
        Some(other) => {
            eprintln!("liam: unknown argument {other}\n\n{USAGE}");
            ExitCode::from(1)
        }
    }
}

const USAGE: &str = "usage: liam [--version | --help]

Nothing builds images yet. The plan is in ROADMAP.md.";

fn version_line() -> String {
    return format!("liam {}", env!("CARGO_PKG_VERSION"));
}
