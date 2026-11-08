//! Stub para horizon-mod-api-gen (baseado em horizon-rust/horizon MIT).

use std::path::Path;
use std::fs;

pub enum Target {
    Render,
}

pub fn generate(_root: &Path, out_dir: &Path, _target: Target) -> std::io::Result<()> {
    fs::create_dir_all(out_dir)?;
    fs::write(
        out_dir.join("native_render_generated.rs"),
        "// Gerado automaticamente por horizon-mod-api-gen (stub)\npub fn native_render_api() {}\n",
    )?;
    Ok(())
}
