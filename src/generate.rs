use binary_install::Download;
use child;
use emoji;
use failure::{self, ResultExt};
use std::process::Command;

pub fn generate(template: &str, name: &str, download: &Download) -> Result<(), failure::Error> {
    let bin_path = download.binary("cargo-generate")?;
    let cargo_generate = format!(
        "{} generate --git {} --name {}",
        bin_path.to_string_lossy(),
        template,
        name
    );
    let cmd = Command::new(cargo_generate);

    println!(
        "{} Generating a new rustwasm worker project with name '{}'...",
        emoji::SHEEP,
        name
    );
    child::run(cmd, "cargo-generate").context("Running cargo-generate")?;
    Ok(())
}
