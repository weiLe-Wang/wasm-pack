use failure::Error;
use generate;
use log::info;
use std::result;
use PBAR;

/// Executes the 'cargo-generate' command in the current directory
/// which generates a new rustwasm project from a template.
pub fn generate(template: Option<String>, name: Option<String>) -> result::Result<(), Error> {
    info!("Generating a new rustwasm project...");
    let template = template.unwrap_or("https://github.com/rustwasm/wasm-pack-template".to_string());
    let name = name.unwrap_or("hello-wasm".to_string());

    generate::generate(template, name)?;

    let msg = format!("🐑 Generated new project at /{}", name);
    PBAR.info(&msg);
    Ok(())
}
