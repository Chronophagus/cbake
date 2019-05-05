use structopt::StructOpt;

use std::path::PathBuf;

// This function is to avoid a StructOpt import in main.rs
pub fn parse_command() -> Command {
    Command::from_args()
}

#[derive(Debug, StructOpt)]
#[structopt(name = "cbake", about = "Let me bake cmake for you")]
pub enum Command {
    #[structopt(name = "init")]
    /// UNIMPLEMENTED!
    /// Create a project in place in an existing empty directory
    /// OR Initialize cmake_project from existing project
    Init {},

    #[structopt(name = "new")]
    /// Create a project at <path>
    New { path: PathBuf },

    #[structopt(name = "build")]
    /// Build a project
    Build {
        #[structopt(long = "release")]
        /// Use release configuration
        release: bool,
    },

    #[structopt(name = "run")]
    /// Build and run a project
    Run {
        #[structopt(long = "release")]
        /// Use release configuration
        release: bool,
    },

    #[structopt(name = "clean")]
    /// UNIMPLEMENTED! Clean up cmake cache
    Clean {},
}
