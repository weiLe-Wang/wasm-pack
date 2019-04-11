use binary_install::{Cache, Download};
use child;
use emoji;
use failure::{self, ResultExt};
use log::debug;
use log::{info, warn};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use target;
use which::which;
use PBAR;

/// Install a cargo CLI tool
///
/// Prefers an existing local install, if any exists. Then checks if there is a
/// global install on `$PATH` that fits the bill. Then attempts to download a
/// tarball from the GitHub releases page, if this target has prebuilt
/// binaries. Finally, falls back to `cargo install`.
pub fn install(
    tool: &str,
    cache: &Cache,
    version: &str,
    install_permitted: bool,
) -> Result<Download, failure::Error> {
    // If the tool is installed globally and it has the right version, use
    // that. Assume that other tools are installed next to it.
    //
    // This situation can arise if the tool is already installed via
    // `cargo install`, for example.
    if let Ok(path) = which(tool) {
        debug!("found global {} binary at: {}", tool, path.display());
        if check_version(tool, &path, version) {
            return Ok(Download::at(path.parent().unwrap()));
        }
    }

    let msg = format!("{}Installing {}...", emoji::DOWN_ARROW, tool);
    PBAR.info(&msg);

    let dl = download_prebuilt(tool, &cache, version, install_permitted);
    match dl {
        Ok(dl) => return Ok(dl),
        Err(e) => {
            warn!(
                "could not download pre-built `{}`: {}. Falling back to `cargo install`.",
                tool, e
            );
        }
    }

    cargo_install(tool, &cache, version, install_permitted)
}

/// Check if the tool dependency is locally satisfied.
fn check_version(tool: &str, path: &PathBuf, expected_version: &str) -> bool {
    let mut cmd = Command::new(path);
    cmd.arg("--version");
    child::run_capture_stdout(cmd, tool)
        .map(|stdout| {
            stdout
                .trim()
                .split_whitespace()
                .nth(1)
                .map(|v| {
                    info!(
                        "Checking installed `{}` version == expected version: {} == {}",
                        tool,
                        v, expected_version
                    );
                    v == expected_version
                })
                .unwrap_or(false)
        })
        .unwrap_or(false)
}

/// Downloads a precompiled copy of the tool, if available.
pub fn download_prebuilt(
    tool: &str,
    cache: &Cache,
    version: &str,
    install_permitted: bool,
) -> Result<Download, failure::Error> {
    let url = match prebuilt_url(tool, version) {
        Ok(url) => url,
        Err(e) => bail!(
            "no prebuilt {} binaries are available for this platform: {}",
            tool,
            e,
        ),
    };
    match tool {
        "wasm-bindgen" => {
            let binaries = &["wasm-bindgen", "wasm-bindgen-test-runner"];
            match cache.download(install_permitted, "wasm-bindgen", binaries, &url)? {
                Some(download) => Ok(download),
                None => bail!("wasm-bindgen v{} is not installed!", version),
            }
        }
        "cargo-generate" => {
            let binaries = &["cargo-generate"];
            match cache.download(install_permitted, "cargo-generate", binaries, &url)? {
                Some(download) => Ok(download),
                None => bail!("cargo-generate v{} is not installed!", version),
            }
        }
        _ => bail!("Unrecognized tool name!")
    }
}

/// Returns the URL of a precompiled version of wasm-bindgen, if we have one
/// available for our host platform.
fn prebuilt_url(tool: &str, version: &str) -> Result<String, failure::Error> {
    let target = if target::LINUX && target::x86_64 {
        "x86_64-unknown-linux-musl"
    } else if target::MACOS && target::x86_64 {
        "x86_64-apple-darwin"
    } else if target::WINDOWS && target::x86_64 {
        "x86_64-pc-windows-msvc"
    } else {
        bail!("Unrecognized target!")
    };

    match tool {
        "wasm-bindgen" => {
            Ok(format!(
                "https://github.com/rustwasm/wasm-bindgen/releases/download/{0}/wasm-bindgen-{0}-{1}.tar.gz",
                version,
                target
            ))
        },
        "cargo-generate" => {
            Ok(format!(
                "https://github.com/ashleygwilliams/cargo-generate/releases/download/v{0}/cargo-generate-v{0}-{1}.tar.gz",
                version,
                target
            ))
        }
        _ => bail!("Unrecognized tool name!")
    }
}

/// Use `cargo install` to install the tool locally into the given
/// crate.
pub fn cargo_install(
    tool: &str,
    cache: &Cache,
    version: &str,
    install_permitted: bool,
) -> Result<Download, failure::Error> {
    debug!(
        "Attempting to use a `cargo install`ed version of `{}={}`",
        tool, version,
    );

    let dirname = format!("{}-cargo-install-{}", tool, version);
    let destination = cache.join(dirname.as_ref());
    if destination.exists() {
        debug!(
            "`cargo install`ed `{}={}` already exists at {}",
            tool,
            version,
            destination.display()
        );
        return Ok(Download::at(&destination));
    }

    if !install_permitted {
        bail!("{} v{} is not installed!", tool, version)
    }

    // Run `cargo install` to a temporary location to handle ctrl-c gracefully
    // and ensure we don't accidentally use stale files in the future
    let tmp = cache.join(format!(".{}", dirname).as_ref());
    drop(fs::remove_dir_all(&tmp));
    debug!("cargo installing {} to tempdir: {}", tool, tmp.display(),);

    let context = format!("failed to create temp dir for `cargo install {}`", tool);
    fs::create_dir_all(&tmp).context(context)?;

    let crate_name = if tool == "wasm-bindgen" {
        "wasm-bindgen-cli"
    } else {
        tool
    };
    let mut cmd = Command::new("cargo");
    cmd.arg("install")
        .arg("--force")
        .arg(crate_name)
        .arg("--version")
        .arg(version)
        .arg("--root")
        .arg(&tmp);

    let context = format!("Installing {} with cargo", tool);
    child::run(cmd, "cargo install").context(context)?;

    // `cargo install` will put the installed binaries in `$root/bin/*`, but we
    // just want them in `$root/*` directly (which matches how the tarballs are
    // laid out, and where the rest of our code expects them to be). So we do a
    // little renaming here.
    let binaries = if tool == "wasm-bindgen" {
        vec!["wasm-bindgen", "wasm-bindgen-test-runner"]
    } else {
        vec!["cargo-genrate"]
    };

    for b in binaries.iter().cloned() {
        let from = tmp
            .join("bin")
            .join(b)
            .with_extension(env::consts::EXE_EXTENSION);
        let to = tmp.join(from.file_name().unwrap());
        fs::rename(&from, &to).with_context(|_| {
            format!(
                "failed to move {} to {} for `cargo install`ed `{}`",
                from.display(),
                to.display(),
                b
            )
        })?;
    }

    // Finally, move the `tmp` directory into our binary cache.
    fs::rename(&tmp, &destination)?;

    Ok(Download::at(&destination))
}
