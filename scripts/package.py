"""Package an already-built native executable using mesh plugin conventions."""
import hashlib
from pathlib import Path
import subprocess
import tarfile
import tomllib
import zipfile

root = Path(__file__).resolve().parents[1]
version = tomllib.loads((root / "Cargo.toml").read_text())["package"]["version"]
target = next(line.split(": ", 1)[1] for line in subprocess.check_output(
    ["rustc", "-vV"], text=True).splitlines() if line.startswith("host: "))
windows = "windows" in target
binary = "lexe-wallet.exe" if windows else "lexe-wallet"
files = [(root / "target/release" / binary, "pay-lightning.exe" if windows else "pay-lightning")] + [
    (root / name, name) for name in ("plugin.toml", "README.md", "LICENSE")]
for source, _ in files:
    if not source.is_file():
        raise SystemExit(f"Missing {source}; run just build first")
(root / "dist").mkdir(exist_ok=True)
manifest = root / "dist/plugin-manifest.json"
manifest.write_bytes(subprocess.check_output([str(root / "target/release" / binary), "--manifest"]))
files.append((manifest, "plugin-manifest.json"))
archive = root / "dist" / f"pay-lightning-v{version}-{target}.{'zip' if windows else 'tar.gz'}"
if windows:
    with zipfile.ZipFile(archive, "w", zipfile.ZIP_DEFLATED) as bundle:
        for source, name in files:
            bundle.write(source, f"pay-lightning/{name}")
else:
    with tarfile.open(archive, "w:gz") as bundle:
        for source, name in files:
            bundle.add(source, arcname=f"pay-lightning/{name}")
archive.with_name(archive.name + ".sha256").write_text(
    f"{hashlib.sha256(archive.read_bytes()).hexdigest()}  {archive.name}\n")
print(archive)
