use fanwaave_client::RuntimeClientConfig;
use std::collections::BTreeMap;
use std::path::Path;

fn repository_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("Rust client manifest must remain under clients/rust")
}

#[test]
fn root_contract_resolves_argv_through_flags2env() {
    let config_path = repository_root().join(".fanwaave-cfg.toml");
    let argv = vec![
        "fanwaave-client".to_owned(),
        "--api-base".to_owned(),
        "https://api.example.test".to_owned(),
        "--tenant-id".to_owned(),
        "tenant-a".to_owned(),
    ];

    let runtime = RuntimeClientConfig::from_path_and_sources(
        &config_path,
        &BTreeMap::new(),
        &argv,
    )
    .expect("repository-root Fanwaave + flags2env contracts must resolve");

    let client = runtime.client.expect("argv API base configures a client");
    assert_eq!(client.base_url, "https://api.example.test");
    assert_eq!(runtime.tenant_id.as_deref(), Some("tenant-a"));
    assert_eq!(client.bearer_token, None);
}

#[test]
fn root_contract_keeps_auth_token_environment_only() {
    let config_path = repository_root().join(".fanwaave-cfg.toml");
    let ambient = BTreeMap::from([
        (
            "FANWAAVE_API_BASE_URL".to_owned(),
            "https://api.example.test".to_owned(),
        ),
        ("FANWAAVE_AUTH_TOKEN".to_owned(), "secret-value".to_owned()),
    ]);
    let argv = vec!["fanwaave-client".to_owned()];

    let runtime = RuntimeClientConfig::from_path_and_sources(&config_path, &ambient, &argv)
        .expect("environment-only secret must resolve");
    let client = runtime.client.expect("ambient API base configures a client");

    assert_eq!(client.bearer_token.as_deref(), Some("secret-value"));
    assert!(!format!("{client:?}").contains("secret-value"));
}

#[test]
fn root_contract_rejects_unknown_flags() {
    let config_path = repository_root().join(".fanwaave-cfg.toml");
    let argv = vec![
        "fanwaave-client".to_owned(),
        "--definitely-unknown".to_owned(),
        "value".to_owned(),
    ];

    assert!(
        RuntimeClientConfig::from_path_and_sources(&config_path, &BTreeMap::new(), &argv).is_err(),
        "unknown argv must fail closed"
    );
}
