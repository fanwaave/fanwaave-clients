#![forbid(unsafe_code)]

use crate::error::ClientError;
use fanwaave_lib_core::fanwaave_config::{
    parse_fanwaave_config, ConfigValue, ResolvedFanwaaveConfig, FANWAAVE_CONFIG_FILENAME,
};
use fanwaave_lib_core::fanwaave_flags2env::{
    resolve_fanwaave_config_from_argv, FanwaaveFlags2EnvError,
};
use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;
use thiserror::Error;

const DEFAULT_MAX_RESPONSE_BYTES: usize = 64 * 1024;

#[derive(Clone, PartialEq, Eq)]
pub struct ClientConfig {
    pub base_url: String,
    pub bearer_token: Option<String>,
    pub max_response_bytes: usize,
}

impl fmt::Debug for ClientConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ClientConfig")
            .field("base_url", &self.base_url)
            .field(
                "bearer_token",
                &self.bearer_token.as_ref().map(|_| "[REDACTED]"),
            )
            .field("max_response_bytes", &self.max_response_bytes)
            .finish()
    }
}

impl ClientConfig {
    /// Read the canonical Fanwaave client environment names directly.
    /// Executables that accept argv should prefer [`RuntimeClientConfig::from_process`]
    /// so `.fanwaave-cfg.toml` and `flags-2-env` are both enforced.
    pub fn from_env() -> Result<Self, ClientError> {
        let base = std::env::var("FANWAAVE_API_BASE_URL").map_err(|_| ClientError::InvalidBase)?;
        if base.trim().is_empty() {
            return Err(ClientError::InvalidBase);
        }
        Ok(Self {
            base_url: base,
            bearer_token: std::env::var("FANWAAVE_AUTH_TOKEN")
                .ok()
                .filter(|value| !value.trim().is_empty()),
            max_response_bytes: DEFAULT_MAX_RESPONSE_BYTES,
        })
    }
}

/// Fanwaave client projection resolved from the admitted `.fanwaave-cfg.toml`
/// contract. `client` is `None` when no API base URL is configured, allowing a
/// service to ship the integration before its deployment receives Fanwaave
/// credentials. A configured authentication token is never exposed through
/// `Debug` and the shared resolver rejects it when it originates from argv.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeClientConfig {
    pub client: Option<ClientConfig>,
    pub tenant_id: Option<String>,
}

#[derive(Debug, Error)]
pub enum RuntimeConfigError {
    #[error("cannot read Fanwaave runtime config {path}: {source}")]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error(transparent)]
    Resolve(#[from] FanwaaveFlags2EnvError),
    #[error("Fanwaave binding {0} resolved with an unexpected type")]
    BindingType(&'static str),
}

impl RuntimeClientConfig {
    /// Load repository-root `.fanwaave-cfg.toml`, audit/parse argv through its
    /// declared `.cli-flags.toml` contract, and resolve argv > environment >
    /// non-secret Fanwaave defaults. Secret values remain environment-only.
    pub fn from_process() -> Result<Self, RuntimeConfigError> {
        Self::from_path_and_sources(
            Path::new(FANWAAVE_CONFIG_FILENAME),
            &std::env::vars().collect(),
            &std::env::args().collect::<Vec<_>>(),
        )
    }

    pub fn from_path_and_sources(
        path: &Path,
        ambient: &BTreeMap<String, String>,
        argv: &[String],
    ) -> Result<Self, RuntimeConfigError> {
        let text = std::fs::read_to_string(path).map_err(|source| RuntimeConfigError::Read {
            path: path.display().to_string(),
            source,
        })?;
        let config = parse_fanwaave_config(&text).map_err(FanwaaveFlags2EnvError::from)?;
        let resolved = resolve_fanwaave_config_from_argv(&config, ambient, argv)?;
        Self::from_resolved(&resolved)
    }

    pub fn from_resolved(resolved: &ResolvedFanwaaveConfig) -> Result<Self, RuntimeConfigError> {
        let base_url = optional_url(resolved, "api_base_url")?;
        let tenant_id = optional_string(resolved, "tenant_id")?;
        let bearer_token = optional_string(resolved, "auth_token")?;

        let client = base_url.map(|base_url| ClientConfig {
            base_url,
            bearer_token,
            max_response_bytes: DEFAULT_MAX_RESPONSE_BYTES,
        });

        Ok(Self { client, tenant_id })
    }
}

fn optional_url(
    resolved: &ResolvedFanwaaveConfig,
    name: &'static str,
) -> Result<Option<String>, RuntimeConfigError> {
    match resolved.binding(name).map(|binding| binding.value()) {
        None => Ok(None),
        Some(ConfigValue::Url(value)) => Ok(Some(value.clone())),
        Some(_) => Err(RuntimeConfigError::BindingType(name)),
    }
}

fn optional_string(
    resolved: &ResolvedFanwaaveConfig,
    name: &'static str,
) -> Result<Option<String>, RuntimeConfigError> {
    match resolved.binding(name).map(|binding| binding.value()) {
        None => Ok(None),
        Some(ConfigValue::String(value)) => Ok(Some(value.clone())),
        Some(_) => Err(RuntimeConfigError::BindingType(name)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fanwaave_lib_core::fanwaave_config::{
        parse_fanwaave_config, resolve_fanwaave_config, FANWAAVE_CONFIG_CONTRACT_REVISION,
        FANWAAVE_CONFIG_TJSV_REVISION,
    };

    const CLIENT_POLICY: &str = r#"
version = 1
mode = "client"
strict = true

[flags2env]
contract = ".cli-flags.toml"
require_audit = true
precedence = "argv-over-env"

[client]
enabled = true
api_base_url_binding = "api_base_url"
tenant_id_binding = "tenant_id"
auth_token_binding = "auth_token"

[[env]]
name = "api_base_url"
key = "FANWAAVE_API_BASE_URL"
kind = "url"
required = false
secret = false

[[env]]
name = "tenant_id"
key = "FANWAAVE_TENANT_ID"
kind = "string"
required = false
secret = false

[[env]]
name = "auth_token"
key = "FANWAAVE_AUTH_TOKEN"
kind = "string"
required = false
secret = true
"#;

    #[test]
    fn admitted_client_projection_uses_canonical_environment_names() {
        let policy = parse_fanwaave_config(CLIENT_POLICY).expect("client policy");
        let ambient = BTreeMap::from([
            (
                "FANWAAVE_API_BASE_URL".to_owned(),
                "https://api.fanwaave.example".to_owned(),
            ),
            ("FANWAAVE_TENANT_ID".to_owned(), "tenant-a".to_owned()),
            ("FANWAAVE_AUTH_TOKEN".to_owned(), "secret-value".to_owned()),
        ]);
        let resolved = resolve_fanwaave_config(&policy, &ambient, &BTreeMap::new())
            .expect("Fanwaave policy resolves");
        let runtime = RuntimeClientConfig::from_resolved(&resolved).expect("runtime projection");
        let client = runtime.client.expect("configured client");
        assert_eq!(client.base_url, "https://api.fanwaave.example");
        assert_eq!(runtime.tenant_id.as_deref(), Some("tenant-a"));
        assert_eq!(client.bearer_token.as_deref(), Some("secret-value"));
        assert!(!format!("{client:?}").contains("secret-value"));
    }

    #[test]
    fn secret_auth_token_cannot_cross_the_argv_boundary() {
        let policy = parse_fanwaave_config(CLIENT_POLICY).expect("client policy");
        let error = resolve_fanwaave_config(
            &policy,
            &BTreeMap::new(),
            &BTreeMap::from([(
                "FANWAAVE_AUTH_TOKEN".to_owned(),
                "must-not-be-argv".to_owned(),
            )]),
        )
        .expect_err("secret argv must fail closed");
        assert!(error.to_string().contains("may not be supplied through argv"));
    }

    #[test]
    fn client_is_pinned_to_tjsv_admitted_contract_provenance() {
        assert_eq!(
            FANWAAVE_CONFIG_CONTRACT_REVISION,
            "e27695091a5b8276543a6f435156a25043f297a9"
        );
        assert_eq!(
            FANWAAVE_CONFIG_TJSV_REVISION,
            "4a5d049218adc2740d4cf78f612caf7f38f6f64c"
        );
    }
}
