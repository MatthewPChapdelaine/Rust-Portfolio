use std::path::Path;
use anyhow::{Context, Result};
use colored::Colorize;

use crate::models::ResolvedPackage;

pub fn install_packages(packages: &[ResolvedPackage]) -> Result<()> {
    let target_dir = Path::new("pkg_modules");
    
    if !target_dir.exists() {
        std::fs::create_dir(target_dir)?;
    }

    for package in packages {
        install_package(package, target_dir)?;
    }

    Ok(())
}

fn install_package(package: &ResolvedPackage, target_dir: &Path) -> Result<()> {
    let package_dir = target_dir.join(&package.name);
    
    if package_dir.exists() {
        std::fs::remove_dir_all(&package_dir)?;
    }
    
    std::fs::create_dir(&package_dir)?;

    let version_file = package_dir.join("VERSION");
    std::fs::write(version_file, package.version.to_string())?;

    let readme = format!(
        "# {} v{}\n\n\
        This is a simulated package installation.\n\n\
        ## Dependencies\n\n\
        {}\n\n\
        Installed by: Package Manager (pkgmgr)\n",
        package.name,
        package.version,
        if package.dependencies.is_empty() {
            "No dependencies".to_string()
        } else {
            package.dependencies
                .iter()
                .map(|d| format!("- {}", d))
                .collect::<Vec<_>>()
                .join("\n")
        }
    );
    
    std::fs::write(package_dir.join("README.md"), readme)?;

    let manifest = format!(
        "[package]\n\
        name = \"{}\"\n\
        version = \"{}\"\n\n\
        [dependencies]\n\
        {}",
        package.name,
        package.version,
        package.dependencies
            .iter()
            .map(|d| format!("{} = \"*\"", d))
            .collect::<Vec<_>>()
            .join("\n")
    );
    
    std::fs::write(package_dir.join("Package.toml"), manifest)?;

    println!("  {} {} v{}", "✓".green(), package.name.bold(), package.version.to_string().cyan());

    Ok(())
}

pub fn verify_installation(packages: &[ResolvedPackage]) -> Result<bool> {
    let target_dir = Path::new("pkg_modules");
    
    if !target_dir.exists() {
        return Ok(false);
    }

    for package in packages {
        let package_dir = target_dir.join(&package.name);
        let version_file = package_dir.join("VERSION");
        
        if !version_file.exists() {
            return Ok(false);
        }

        let installed_version = std::fs::read_to_string(version_file)?;
        if installed_version.trim() != package.version.to_string() {
            return Ok(false);
        }
    }

    Ok(true)
}
