use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pkgmgr")]
#[command(about = "A Cargo-like package manager", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Install dependencies")]
    Install {
        #[arg(help = "Specific package to install")]
        package: Option<String>,
    },
    
    #[command(about = "Update dependencies")]
    Update,
    
    #[command(about = "Display dependency tree")]
    Tree,
    
    #[command(about = "Initialize a new package")]
    Init {
        #[arg(help = "Package name")]
        name: String,
    },
    
    #[command(about = "Registry operations")]
    Registry {
        #[command(subcommand)]
        subcommand: RegistryCommands,
    },
}

#[derive(Subcommand)]
pub enum RegistryCommands {
    #[command(about = "List all packages")]
    List,
    
    #[command(about = "Search packages")]
    Search {
        #[arg(help = "Search query")]
        query: String,
    },
    
    #[command(about = "Show package info")]
    Info {
        #[arg(help = "Package name")]
        package: String,
    },
}
