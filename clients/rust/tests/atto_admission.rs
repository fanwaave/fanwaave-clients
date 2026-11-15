#![forbid(unsafe_code)]
use serde_json::{json, Value};

fn source_json(source: &str) -> Value {
    serde_json::from_str(source).expect("valid JSON source")
}
fn binding() -> Value {
    source_json(include_str!("../../atto/contract-binding.v1.json"))
}
fn projection() -> Value {
    source_json(include_str!("../../../rpc/public-projection.v1.json"))
}
fn validate(binding: &Value, projection: &Value) -> Result<(), &'static str> {
    if binding["schema"] != "fanwaave.atto-client-binding/v1"
        || binding["language"] != "atto"
        || binding["admission"] != "contract-only"
        || binding["contractAuthority"] != "fanwaave/fanwaave-interfaces"
        || binding["transportImplemented"] != false
        || binding["operation"] != "fanwaave.health.get"
        || binding["audience"] != "browser"
        || binding["scope"] != "regular"
    {
        return Err("unauthorized binding");
    }
    let security = &binding["security"];
    if security["httpsOnly"] != true
        || security["bearerHeader"] != true
        || security["boundedResponse"] != true
    {
        return Err("missing security guarantees");
    }
    if projection["schema"] != "fanwaave.rpc-public-projection/v1"
        || projection["contractAuthority"] != binding["contractAuthority"]
        || projection["audience"] != "public"
        || projection["scope"] != "regular"
        || projection["rules"]["allowAdminScope"] != false
        || projection["rules"]["allowAudienceWidening"] != false
        || projection["rules"]["allowHandAuthoredGeneratedSources"] != false
    {
        return Err("RPC public authority mismatch");
    }
    let operations = projection["operations"].as_array().ok_or("invalid operations")?;
    if !operations.iter().any(|op| {
        op["key"] == binding["operation"]
            && op["requiredAudience"] == binding["audience"]
            && op["requiredScope"] == binding["scope"]
    }) {
        return Err("public operation not admitted");
    }
    Ok(())
}
#[test]
fn admits_public_health_contract_only() {
    assert_eq!(validate(&binding(), &projection()), Ok(()));
}
#[test]
fn support_matrix_is_explicitly_experimental() {
    let matrix = include_str!("../../support-matrix.toml");
    let atto = matrix.split("[languages.atto]").nth(1).expect("AttoLang entry");
    assert!(atto.contains("tier = 3"));
    assert!(atto.contains("status = \"contract-only\""));
}
#[test]
fn pure_policy_predicates_cover_bounds() {
    let source = include_str!("../../atto/health.atto");
    for expected in [
        "pub fnc valid_health_status(int status_code): bool",
        "pub fnc within_response_budget(int payload_bytes, int max_response_bytes): bool",
        "pub fnc accept_health_response(int status_code, int payload_bytes, int max_response_bytes): bool",
        "if payload_bytes < 0 then",
        "if max_response_bytes < 1 then",
        "return payload_bytes <= max_response_bytes;",
    ] {
        assert!(source.contains(expected), "missing invariant: {expected}");
    }
    assert!(!source.contains("pub fnc main("));
}
#[test]
fn rejects_admin_widening() {
    for (field, value) in [
        ("scope", json!("admin")),
        ("audience", json!("server")),
        ("operation", json!("fanwaave.admin.get")),
    ] {
        let mut bad = binding();
        bad[field] = value;
        assert!(validate(&bad, &projection()).is_err());
    }
}
#[test]
fn rejects_security_and_transport_downgrades() {
    for flag in ["httpsOnly", "bearerHeader", "boundedResponse"] {
        let mut bad = binding();
        bad["security"][flag] = json!(false);
        assert!(validate(&bad, &projection()).is_err());
    }
    let mut bad = binding();
    bad["transportImplemented"] = json!(true);
    assert!(validate(&bad, &projection()).is_err());
    let mut bad = binding();
    bad["security"] = Value::Null;
    assert!(validate(&bad, &projection()).is_err());
}
#[test]
fn rejects_missing_or_forged_rpc_operation() {
    let mut bad = projection();
    bad["operations"] = json!([]);
    assert!(validate(&binding(), &bad).is_err());
    let mut bad = projection();
    bad["rules"]["allowAdminScope"] = json!(true);
    assert!(validate(&binding(), &bad).is_err());
    let mut bad = projection();
    bad["operations"] = json!([{
        "key": "fanwaave.health.get",
        "requiredAudience": "server",
        "requiredScope": "regular"
    }]);
    assert!(validate(&binding(), &bad).is_err());
}
#[test]
fn rejects_untrusted_authority_and_invalid_operation_shape() {
    let mut bad = binding();
    bad["contractAuthority"] = json!("fanwaave/untrusted");
    assert!(validate(&bad, &projection()).is_err());
    let mut bad = projection();
    bad["operations"] = json!({});
    assert!(validate(&binding(), &bad).is_err());
}
