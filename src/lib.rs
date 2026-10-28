//! Lightning plugin backed by the reusable Lexe wallet adapter.
#![forbid(unsafe_code)]
mod lexe;
use anyhow::{Result, ensure};
use async_trait::async_trait;
use mesh_llm_wallet::{
    provider::WalletProvider,
    provisioning::{WalletFactory, WalletPin},
};
use std::{path::Path, sync::Arc};

pub const PLUGIN_NAME: &str = "pay-lightning";
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Opens only this plugin's isolated `wallet/` subdirectory. No host-state migration.
pub struct LexeFactory;
#[async_trait]
impl WalletFactory for LexeFactory {
    fn is_provisioned(&self, directory: &Path) -> bool {
        directory.join("wallet/seedphrase.txt").exists() || WalletPin::path(directory).exists()
    }
    async fn open(&self, directory: &Path) -> Result<Arc<dyn WalletProvider>> {
        let pin = WalletPin::load(directory)?;
        let wallet_directory = directory.join("wallet");
        ensure!(
            pin.is_none() || wallet_directory.join("seedphrase.txt").is_file(),
            "pinned wallet seed missing; refusing to provision replacement"
        );
        let opened = lexe::LexeProvider::open(&wallet_directory).await?;
        let actual = WalletPin {
            plugin: PLUGIN_NAME.into(),
            wallet_id: opened.identity.wallet_id,
            provider: opened.identity.provider,
            network: opened.identity.network,
        };
        if let Some(pin) = pin {
            ensure!(
                pin == actual,
                "wallet identity changed; refusing replacement"
            );
        } else {
            actual.store(directory)?;
        }
        Ok(Arc::new(opened.provider))
    }
}

/// Launch with explicit isolated configuration, never adopting former host state.
pub async fn run_plugin(name: String) -> Result<()> {
    mesh_llm_pay_lightning::run(
        name,
        VERSION,
        LexeFactory,
        mesh_llm_pay_lightning::Config::from_launch_env()?,
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn discovery_is_side_effect_free() {
        let directory = tempfile::tempdir().unwrap();
        let state = directory.path().join("absent");
        assert!(!LexeFactory.is_provisioned(&state));
        assert!(!state.exists());
        std::fs::create_dir_all(state.join("wallet")).unwrap();
        std::fs::write(
            state.join("wallet/seedphrase.txt"),
            "synthetic, never opened",
        )
        .unwrap();
        assert!(LexeFactory.is_provisioned(&state));
    }
}
