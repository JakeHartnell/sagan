//! `sagan.toml` monitoring agent configuration settings

use crate::error::{Error, ErrorKind};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tendermint::{config::TendermintConfig, net};

/// Tendermint node-related config settings from `sagan.toml`
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgentConfig {
    /// Location of monitored Tendermint node's `--home` directory
    pub node_home: PathBuf,

    /// Location of collector
    pub collector: CollectorAddr,
}

impl AgentConfig {
    /// Path to the node's configuration directory
    pub fn config_dir(&self) -> PathBuf {
        self.node_home.join("config")
    }

    /// Path to the node's `config.toml` file
    pub fn config_toml_path(&self) -> PathBuf {
        self.config_dir().join("config.toml")
    }

    /// Load `TendermintConfig` using this node configuration
    pub fn load_tendermint_config(&self) -> Result<TendermintConfig, Error> {
        Ok(TendermintConfig::load_toml_file(&self.config_toml_path())
            .map_err(|e| format_err!(ErrorKind::ConfigError, "{}", e))?)
    }
}

/// Collector config
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum CollectorAddr {
    /// Collector HTTP config
    #[serde(rename = "http")]
    Http(HttpConfig),
}

/// Http config
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HttpConfig {
    /// Address of collector http service
    pub addr: net::Address,
}
