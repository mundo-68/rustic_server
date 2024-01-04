//! RusticServer Subcommands
//!
//! This is where you specify the subcommands of your application.
//!
//! The default application comes with two subcommands:
//!
//! - `start`: launches the application
//! - `--version`: print application version
//!
//! See the `impl Configurable` below for how to specify the path to the
//! application's configuration file.

mod start;

use self::start::StartCmd;
use crate::config::RusticServerConfig;
use abscissa_core::{config::Override, Command, Configurable, FrameworkError, Runnable};
use clap::Parser;
use std::path::PathBuf;

/// RusticServer Configuration Filename
pub const CONFIG_FILE: &str = "rustic_server.toml";

/// RusticServer Subcommands
/// Subcommands need to be listed in an enum.
#[derive(clap::Parser, Command, Debug, Runnable)]
pub enum RusticServerCmd {
    /// The `start` subcommand
    Start(StartCmd),
}

/// A REST server build in rust for use with rustic and restic
#[derive(clap::Parser, Command, Debug)]
#[command(author, about, version)]
#[command(name = "rustic-server")]
#[command(bin_name = "rustic-server")]
pub struct EntryPoint {
    #[command(subcommand)]
    cmd: RusticServerCmd,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Use the specified config file
    #[arg(short, long)]
    pub config: Option<String>,

    /// listen address
    #[arg(short, long, default_value = "localhost:8000")]
    pub listen: String,
    /// data directory
    #[arg(short, long, default_value = "/tmp/restic")]
    pub path: PathBuf,
    /// disable .htpasswd authentication
    #[arg(long)]
    pub no_auth: bool,
    /// file to read per-repo ACLs from
    #[arg(long)]
    pub acl: Option<PathBuf>,
    /// set standard acl to append only mode
    #[arg(long)]
    pub append_only: bool,
    /// set standard acl to only access private repos
    #[arg(long)]
    pub private_repo: bool,
    /// turn on TLS support
    #[arg(long)]
    pub tls: bool,
    /// TLS certificate path
    #[arg(long)]
    pub cert: Option<String>,
    /// TLS key path
    #[arg(long)]
    pub key: Option<String>,
    /// logging level (Off/Error/Warn/Info/Debug/Trace)
    #[arg(long, default_value = "Info")]
    pub log: tide::log::LevelFilter,
}

impl Runnable for EntryPoint {
    fn run(&self) {
        self.cmd.run()
    }
}

/// This trait allows you to define how application configuration is loaded.
impl Configurable<RusticServerConfig> for EntryPoint {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        // Check if the config file exists, and if it does not, ignore it.
        // If you'd like for a missing configuration file to be a hard error
        // instead, always return `Some(CONFIG_FILE)` here.
        let filename = self
            .config
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(|| CONFIG_FILE.into());

        if filename.exists() {
            Some(filename)
        } else {
            None
        }
    }

    /// Apply changes to the config after it's been loaded, e.g. overriding
    /// values in a config file using command-line options.
    ///
    /// This can be safely deleted if you don't want to override config
    /// settings from command-line options.
    fn process_config(
        &self,
        config: RusticServerConfig,
    ) -> Result<RusticServerConfig, FrameworkError> {
        match &self.cmd {
            RusticServerCmd::Start(cmd) => cmd.override_config(config),
            //
            // If you don't need special overrides for some
            // subcommands, you can just use a catch all
            // _ => Ok(config),
        }
    }
}
