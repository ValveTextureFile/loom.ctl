use clap::{Parser, Subcommand, ValueEnum};
// use crate::commands;

pub mod ctx {
    use std::{path::PathBuf};
    use directories::ProjectDirs;

    #[derive(Debug)]
    pub struct Context {
        pub verbose: u8,
        pub config_dir: PathBuf,
        pub profiles_dir: PathBuf,
    }

    impl Context {
        pub fn new(verbose: u8) -> anyhow::Result<Self> {
            let dirs = ProjectDirs::from("dev", "vtf", "loomctl")
                .ok_or_else(|| anyhow::anyhow!("can't configure config directory"))?;

            let config_dir = dirs.config_dir().to_path_buf();
            let profiles_dir = config_dir.join("profiles");

            std::fs::create_dir_all(&profiles_dir)?;

            Ok( Self {
                verbose,
                config_dir,
                profiles_dir
            })
            
        }
    }
}

pub mod cli {
    use super::args::{DoctorArgs, ListArgs, PrintCmdArgs, ProfileArgs, RunArgs, ScanArgs};
    use super::{Parser, Subcommand};

    #[derive(Parser, Debug)]
    #[command(author = "@ValveTextureFile", version, about, long_about = "doom launcher with mod support")]
    pub struct Cli {
        /// Increase verbosity (-v, -vv)
        #[arg(short, long, action = clap::ArgAction::Count)]
        pub verbose: u8,

        /// Emit JSON output where supported
        #[arg(long, global = true)]
        pub json: bool,

        #[command(subcommand)]
        pub cmd: Command,
    }

    #[derive(Subcommand, Debug)]
    pub enum Command {
        /// Run a profile or ad-hoc launch
        Run(RunArgs),

        /// List IWADs, mods, engines, or profiles
        List(ListArgs),

        /// Scan folders for WADs and engines
        Scan(ScanArgs),

        /// Validate config and profiles
        Doctor(DoctorArgs),

        /// Print the engine command without running it
        PrintCmd(PrintCmdArgs),

        /// Manage profiles
        Profile(ProfileArgs),
    }
}

pub mod args {
    use super::{Parser, Subcommand, ValueEnum};

    #[derive(Parser, Debug)]
    /// Run a Doom engine using a profile or ad-hoc arguments
    pub struct RunArgs {
        /// Profile name to run
        pub profile: Option<String>,

        /// Engine ID (gzdoom, dsda, etc.)
        #[arg(long)]
        pub engine: Option<String>,

        /// IWAD path
        #[arg(long)]
        pub iwad: Option<String>,

        /// Mods for ad-hoc runs (in load order)
        #[arg(long, short = 'm')]
        pub mods: Vec<String>,

        /// Everything after `--` is passed to the engine
        #[arg(last = true, trailing_var_arg = true)]
        pub passthrough: Vec<String>,
    }

    #[derive(Parser, Debug)]
    pub struct ListArgs {
        /// What to list
        #[arg(value_enum, default_value_t = ListArgsWhat::Iwads)]
        pub what: ListArgsWhat,
    }

    #[derive(ValueEnum, Clone, Debug)]
    pub enum ListArgsWhat {
        Iwads,
        Mods,
        Engines,
        Profiles,
    }

    #[derive(Parser, Debug)]
    /// Scan folders for IWADs, mods, and engines
    pub struct ScanArgs {
        pub folders: Vec<String>,
    }

    #[derive(Parser, Debug)]
    /// Validate config and profiles
    pub struct DoctorArgs {}

    #[derive(Parser, Debug)]
    /// Print the exact engine command for a profile
    pub struct PrintCmdArgs {
        pub profile: String,
    }

    #[derive(Parser, Debug)]
    pub struct ProfileArgs {
        #[command(subcommand)]
        pub cmd: ProfileCmd,
    }

    #[derive(Subcommand, Debug)]
    pub enum ProfileCmd {
        /// Create a new profile
        New(ProfileNew),

        /// Delete a profile
        Delete(ProfileDelete),

        /// Set profile fields
        Set(ProfileSet),

        /// Add mods to a profile
        AddMod(ProfileAddMod),

        /// Remove a mod from a profile
        RemoveMod(ProfileRemoveMod),
    }

    #[derive(Parser, Debug)]
    pub struct ProfileNew {
        pub name: String,
    }

    #[derive(Parser, Debug)]
    pub struct ProfileDelete {
        pub name: String,
    }

    #[derive(Parser, Debug)]
    pub struct ProfileSet {
        pub name: String,

        #[arg(long)]
        pub engine: Option<String>,

        #[arg(long)]
        pub iwad: Option<String>,
    }

    #[derive(Parser, Debug)]
    pub struct ProfileAddMod {
        pub name: String,
        pub mods: Vec<String>,
    }

    #[derive(Parser, Debug)]
    pub struct ProfileRemoveMod {
        pub name: String,
        /// Index (1-based) or exact path
        pub target: String,
    }
}

pub mod logging {
    use tracing_subscriber;

    pub fn initlogging(verbose: u8) {
    let level = match verbose {
        0 => tracing::Level::WARN,
        1 => tracing::Level::INFO,
        2 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .pretty()
        .init();
}
}