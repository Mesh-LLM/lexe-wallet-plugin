//! Thin launcher; manifest inspection never opens configuration or wallet state.
use anyhow::{Context, Result};
use std::io::Write;
#[tokio::main]
async fn main() -> Result<()> {
    if std::env::args().any(|arg| arg == "--manifest") {
        writeln!(
            mesh_llm_events::machine_out(),
            "{}",
            mesh_llm_pay_lightning::manifest_json(mesh_wallet_lexe::PLUGIN_NAME)?
        )?;
        return Ok(());
    }
    let name = std::env::var("MESH_LLM_PLUGIN_NAME")
        .context("Launch this executable through mesh-llm plugins, not directly")?;
    mesh_wallet_lexe::run_plugin(name).await
}
