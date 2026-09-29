use std::process::Command;

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_liamd"))
        .args(args)
        .output()
        .expect("the liamd binary runs")
}

#[test]
fn version_prints_the_crate_version() {
    let out = run(&["--version"]);
    assert!(out.status.success());
    assert_eq!(
        String::from_utf8_lossy(&out.stdout).trim(),
        format!("liamd {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn without_arguments_it_refuses_to_pretend() {
    assert_eq!(run(&[]).status.code(), Some(1));
}
