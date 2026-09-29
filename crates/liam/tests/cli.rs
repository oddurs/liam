use std::process::Command;

fn liam(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_liam"))
        .args(args)
        .output()
        .expect("the liam binary runs")
}

#[test]
fn version_prints_the_crate_version() {
    let out = liam(&["--version"]);
    assert!(out.status.success());
    assert_eq!(
        String::from_utf8_lossy(&out.stdout).trim(),
        format!("liam {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn an_unknown_argument_is_a_user_error() {
    let out = liam(&["--bogus"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("unknown argument --bogus"));
}
