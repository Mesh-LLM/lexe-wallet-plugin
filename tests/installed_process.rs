//! Manifest inspection is offline, even with invalid runtime configuration.
#[test]
fn manifest_never_opens_wallet_state() {
    let root = tempfile::tempdir().unwrap();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_lexe-wallet"))
        .arg("--manifest")
        .env("HOME", root.path())
        .env("MESH_LLM_PLUGIN_CONFIG", root.path().join("absent.json"))
        .output()
        .unwrap();
    assert!(output.status.success());
    let manifest: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(
        manifest["operations"]
            .as_array()
            .is_none_or(|ops| ops.is_empty())
    );
    assert_eq!(std::fs::read_dir(root.path()).unwrap().count(), 0);
}
