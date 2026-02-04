use clap::Parser;
use anyhow::Result;

mod cli;
mod resolver;
mod registry;
mod installer;
mod lockfile;
mod models;

use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { package } => {
            install_command(package)?;
        }
        Commands::Update => {
            update_command()?;
        }
        Commands::Tree => {
            tree_command()?;
        }
        Commands::Init { name } => {
            init_command(name)?;
        }
        Commands::Registry { subcommand } => {
            registry_command(subcommand)?;
        }
    }

    Ok(())
}

fn install_command(_package: Option<String>) -> Result<()> {
    use colored::Colorize;
    
    println!("{}", "🔍 Reading manifest...".cyan());
    let manifest = models::Manifest::from_file("Package.toml")?;
    
    println!("{}", "📦 Resolving dependencies...".cyan());
    let registry = registry::Registry::new("registry-data")?;
    let resolved = resolver::resolve_dependencies(&manifest, &registry)?;
    
    println!("{} {} packages to install", "✓".green(), resolved.len());
    
    println!("{}", "📥 Installing packages...".cyan());
    installer::install_packages(&resolved)?;
    
    println!("{}", "🔒 Generating lock file...".cyan());
    lockfile::generate_lockfile(&resolved, "Package.lock")?;
    
    println!("{}", "✨ Installation complete!".green().bold());
    Ok(())
}

fn update_command() -> Result<()> {
    use colored::Colorize;
    
    println!("{}", "🔄 Updating dependencies...".cyan());
    
    if std::path::Path::new("Package.lock").exists() {
        std::fs::remove_file("Package.lock")?;
        println!("{}", "🗑️  Removed old lock file".yellow());
    }
    
    install_command(None)?;
    Ok(())
}

fn tree_command() -> Result<()> {
    use colored::Colorize;
    
    println!("{}", "🌳 Dependency tree:".cyan().bold());
    println!();
    
    let lockfile = lockfile::Lockfile::from_file("Package.lock")?;
    let graph = resolver::build_dependency_graph(&lockfile)?;
    
    resolver::print_dependency_tree(&graph)?;
    
    println!();
    println!("{} {} total packages", "✓".green(), lockfile.packages.len());
    Ok(())
}

fn init_command(name: String) -> Result<()> {
    use colored::Colorize;
    
    println!("{} Initializing new package: {}", "🎉".cyan(), name.bold());
    
    let manifest = models::Manifest {
        package: models::PackageInfo {
            name: name.clone(),
            version: "0.1.0".to_string(),
            authors: vec!["Your Name <you@example.com>".to_string()],
            description: Some("A new package".to_string()),
        },
        dependencies: std::collections::HashMap::new(),
    };
    
    let toml = toml::to_string_pretty(&manifest)?;
    std::fs::write("Package.toml", toml)?;
    
    println!("{}", "✓ Created Package.toml".green());
    println!("{}", "✨ Package initialized!".green().bold());
    Ok(())
}

fn registry_command(subcommand: cli::RegistryCommands) -> Result<()> {
    use colored::Colorize;
    
    match subcommand {
        cli::RegistryCommands::List => {
            println!("{}", "📚 Available packages:".cyan().bold());
            println!();
            
            let registry = registry::Registry::new("registry-data")?;
            let packages = registry.list_packages()?;
            let package_count = packages.len();
            
            for (name, versions) in packages {
                println!("  {} {}", "•".blue(), name.bold());
                println!("    versions: {}", versions.join(", "));
            }
            
            println!();
            println!("{} {} packages available", "✓".green(), package_count);
        }
        cli::RegistryCommands::Search { query } => {
            println!("{} Searching for: {}", "🔍".cyan(), query.bold());
            println!();
            
            let registry = registry::Registry::new("registry-data")?;
            let results = registry.search(&query)?;
            
            for (name, info) in results {
                println!("  {} {} v{}", "•".blue(), name.bold(), info.version);
                if let Some(desc) = info.description {
                    println!("    {}", desc);
                }
            }
        }
        cli::RegistryCommands::Info { package } => {
            println!("{} Package info: {}", "ℹ️".cyan(), package.bold());
            println!();
            
            let registry = registry::Registry::new("registry-data")?;
            let info = registry.get_package_info(&package)?;
            
            println!("  Name:        {}", info.name.bold());
            println!("  Version:     {}", info.version);
            println!("  Authors:     {}", info.authors.join(", "));
            if let Some(desc) = info.description {
                println!("  Description: {}", desc);
            }
            
            if !info.dependencies.is_empty() {
                println!("\n  Dependencies:");
                for (name, version) in info.dependencies {
                    println!("    {} {}", name, version);
                }
            }
        }
    }
    
    Ok(())
}
