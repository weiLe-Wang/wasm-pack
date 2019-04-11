use cache;
use failure::Error;
use generate;
use install;
use log::info;
use std::result;
use PBAR;

/// Executes the 'cargo-generate' command in the current directory
/// which generates a new rustwasm project from a template.
pub fn generate(template: Option<String>, name: Option<String>) -> result::Result<(), Error> {
    info!("Generating a new rustwasm project...");
    let template = template.unwrap_or_else(|| "https://github.com/rustwasm/wasm-pack-template".to_string());
    let name = name.unwrap_or_else(|| "hello-wasm".to_string());
    let download = install::install(
        "cargo-generate",
        &cache::get_wasm_pack_cache()?,
        "latest",
        true,
    )?;
    generate::generate(&template, &name, &download)?;

    let msg = format!("🐑 Generated new project at /{}", name);
    PBAR.info(&msg);
    Ok(())
}
