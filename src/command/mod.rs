//! CLI command structures, parsing, and execution.

pub mod build;
mod generate;
mod login;
mod pack;
/// Data structures and functions for publishing a package.
pub mod publish;
pub mod test;
pub mod utils;

use self::build::{Build, BuildOptions};
use self::generate::generate;
use self::login::login;
use self::pack::pack;
use self::publish::{access::Access, publish};
use self::test::{Test, TestOptions};
use failure::Error;
use log::info;
use std::path::PathBuf;
use std::result;

/// The various kinds of commands that `wasm-pack` can execute.
#[derive(Debug, StructOpt)]
pub enum Command {
    /// 🏗️  build your npm package!
    #[structopt(name = "build", alias = "init")]
    Build(BuildOptions),

    #[structopt(name = "pack")]
    /// 🍱  create a tar of your npm package but don't publish!
    Pack {
        /// The path to the Rust crate.
        #[structopt(parse(from_os_str))]
        path: Option<PathBuf>,
    },

    #[structopt(name = "generate")]
    /// 🐑 create a new project with a template
    Generate {
        /// The URL to the template
        #[structopt(
            long = "template",
            short = "temp",
            default_value = "https://github.com/rustwasm/wasm-pack-template"
        )]
        template: String,
        /// The name of the project
        #[structopt(long = "name", short = "n", default_value = "hello-wasm")]
        name: String,
    },

    #[structopt(name = "publish")]
    /// 🎆  pack up your npm package and publish!
    Publish {
        #[structopt(long = "target", short = "t", default_value = "browser")]
        /// Sets the target environment. [possible values: browser, nodejs, no-modules]
        target: String,

        /// The access level for the package to be published
        #[structopt(long = "access", short = "a")]
        access: Option<Access>,

        /// The path to the Rust crate.
        #[structopt(parse(from_os_str))]
        path: Option<PathBuf>,
    },

    #[structopt(name = "login", alias = "adduser", alias = "add-user")]
    /// 👤  Add an npm registry user account! (aliases: adduser, add-user)
    Login {
        #[structopt(long = "registry", short = "r")]
        /// Default: 'https://registry.npmjs.org/'.
        /// The base URL of the npm package registry. If scope is also
        /// specified, this registry will only be used for packages with that
        /// scope. scope defaults to the scope of the project directory you're
        /// currently in, if any.
        registry: Option<String>,

        #[structopt(long = "scope", short = "s")]
        /// Default: none.
        /// If specified, the user and login credentials given will be
        /// associated with the specified scope.
        scope: Option<String>,

        #[structopt(long = "always-auth", short = "a")]
        /// If specified, save configuration indicating that all requests to the
        /// given registry should include authorization information. Useful for
        /// private registries. Can be used with --registry and / or --scope
        always_auth: bool,

        #[structopt(long = "auth-type", short = "t")]
        /// Default: 'legacy'.
        /// Type: 'legacy', 'sso', 'saml', 'oauth'.
        /// What authentication strategy to use with adduser/login. Some npm
        /// registries (for example, npmE) might support alternative auth
        /// strategies besides classic username/password entry in legacy npm.
        auth_type: Option<String>,
    },

    #[structopt(name = "test")]
    /// 👩‍🔬  test your wasm!
    Test(TestOptions),
}

/// Run a command with the given logger!
pub fn run_wasm_pack(command: Command) -> result::Result<(), Error> {
    // Run the correct command based off input and store the result of it so that we can clear
    // the progress bar then return it
    match command {
        Command::Build(build_opts) => {
            info!("Running build command...");
            Build::try_from_opts(build_opts).and_then(|mut b| b.run())
        }
        Command::Pack { path } => {
            info!("Running pack command...");
            info!("Path: {:?}", &path);
            pack(path)
        }
        Command::Generate { template, name } => {
            info!("Running generate command...");
            info!("Template: {:?}", &template);
            info!("Name: {:?}", &name);
            generate(template, name)
        }
        Command::Publish {
            target,
            path,
            access,
        } => {
            info!("Running publish command...");
            info!("Path: {:?}", &path);
            publish(&target, path, access)
        }
        Command::Login {
            registry,
            scope,
            always_auth,
            auth_type,
        } => {
            info!("Running login command...");
            info!(
                "Registry: {:?}, Scope: {:?}, Always Auth: {}, Auth Type: {:?}",
                &registry, &scope, &always_auth, &auth_type
            );
            login(registry, &scope, always_auth, &auth_type)
        }
        Command::Test(test_opts) => {
            info!("Running test command...");
            Test::try_from_opts(test_opts).and_then(|t| t.run())
        }
    }
}
