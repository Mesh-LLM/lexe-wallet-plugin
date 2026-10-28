# Lexe-backed Lightning plugin (draft)

One `pay-lightning` plugin process links the reusable Mesh Lightning payment
runtime and the Lexe wallet adapter. The plugin owns credit, budgets, ledger,
admission and settlement; the host supplies generic exchange hooks and mesh
channels. There is no wallet subprocess and no default MCP payment tools.

## Installation and packaging

After release publication and catalog admission:

```sh
mesh-llm plugins install pay-lightning
```

The catalog name is `pay-lightning`, sourced from
`https://github.com/Mesh-LLM/lexe-wallet`. Direct repository installation is not
the supported identity. Cargo still builds `lexe-wallet`; native archives are
`pay-lightning-v0.1.0-<target>.tar.gz` (ZIP on Windows) and contain
`pay-lightning/pay-lightning`, `plugin.toml`, and `plugin-manifest.json`.
No release or catalog publication is performed by this draft.

## Configuration and safety

Supply `MESH_LLM_PLUGIN_CONFIG` pointing to an explicit JSON configuration,
or place `config.json` under `MESH_LLM_PLUGIN_DATA_DIR`. The configuration must
name an absolute isolated data directory. Example with buying disabled:

```json
{"data_dir":"/absolute/isolated/payment-state","prices":{},"buy_prices":{},"buy_peers":[],"cap_msat":0,"policy":{"mode":"free_only"}}
```

The runtime uses `pay.ln.v1`; installing a different wallet build under the same
identity occupies the same slot. Configuration loading and `--manifest` do not
open wallet state. **Normal initialization opens the wallet and may provision
it, including contacting Lexe.** Free-only policy does not prevent provisioning.
Use only explicitly authorized isolated state for live validation.

Wallet state is under the configured data directory's `wallet/` subdirectory,
with a durable `wallet-provider.json` identity pin. Existing host-wallet state
is not adopted or migrated. Never copy seeds, delete pins, or remove existing
wallet directories to make this draft start. A missing seed under a pin refuses
replacement. The adapter retains its directory lock and restricts plaintext
seed permissions on Unix; Windows permissions require platform validation.
After possible submission, reconcile by payment hash; never blindly retry.
Plugins are trusted local executables, not sandboxes.

## Development and evidence

Use `just check`, `just test`, `just build`, `just package`, and `just clean`.
Mesh dependencies are pinned to
`f97bd0e990711ccea67fc3731f83d336a86e8273`; Lexe SDK is pinned to 0.1.24.
Packaging supports the native build target only.

Offline tests check side-effect-free discovery, manifest inspection and missing
host launch context. They never initialize the real wallet runtime. Installer,
IPC and forced restart coverage belongs to Mesh's synthetic offline mock, not
this executable. These tests do not establish paid inference, live recovery,
performance, or cross-platform compatibility.

## Provenance

Backend and watcher extracted from Mesh-LLM/mesh-llm at
`2129f4bb19862dfb425d27eaf9230cae90dbb850`, under `crates/mesh-wallet-lexe`.
Shared contracts and payment runtime are dependencies, not copied forks.
The copied LICENSE contains Apache-2.0; inherited Cargo metadata declares
`MIT OR Apache-2.0`. That discrepancy needs resolution before publication;
this draft does not supply a missing MIT grant.
