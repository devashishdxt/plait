#[test]
fn required_props_and_invalid_calls() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/*.rs");
}

#[test]
fn cross_crate_imports_and_reexports() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let output = std::process::Command::new(env!("CARGO"))
        .args(["run", "--locked", "--manifest-path"])
        .arg(root.join("tests/fixtures/defaults/Cargo.toml"))
        .args(["-p", "defaults-consumer"])
        .env("CARGO_TARGET_DIR", root.join("../target/cross-crate"))
        .output()
        .expect("run cross-crate consumer fixture");
    assert!(
        output.status.success(),
        "cross-crate fixture failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
