use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};
use crate::models::ResolvedPackage;

#[derive(Debug, Serialize, Deserialize)]
pub struct Lockfile {
    pub version: String,
    pub packages: Vec<LockfilePackage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockfilePackage {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub checksum: String,
}

impl Lockfile {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .context(format!("Failed to read lockfile: {}", path))?;
        let lockfile: Lockfile = toml::from_str(&content)
            .context("Failed to parse lockfile")?;
        Ok(lockfile)
    }
}

pub fn generate_lockfile(packages: &[ResolvedPackage], path: &str) -> Result<()> {
    let mut lockfile_packages = Vec::new();

    for package in packages {
        let checksum = calculate_checksum(&package.name, &package.version.to_string());
        
        lockfile_packages.push(LockfilePackage {
            name: package.name.clone(),
            version: package.version.to_string(),
            dependencies: package.dependencies.clone(),
            checksum,
        });
    }

    lockfile_packages.sort_by(|a, b| a.name.cmp(&b.name));

    let lockfile = Lockfile {
        version: "1.0".to_string(),
        packages: lockfile_packages,
    };

    let toml = toml::to_string_pretty(&lockfile)?;
    std::fs::write(path, toml)?;

    Ok(())
}

fn calculate_checksum(name: &str, version: &str) -> String {
    use sha2::{Sha256, Digest};
    
    let mut hasher = Sha256::new();
    hasher.update(name.as_bytes());
    hasher.update(version.as_bytes());
    let result = hasher.finalize();
    
    format!("{:x}", result)
}

pub fn verify_lockfile(packages: &[ResolvedPackage], path: &str) -> Result<bool> {
    if !std::path::Path::new(path).exists() {
        return Ok(false);
    }

    let lockfile = Lockfile::from_file(path)?;

    if lockfile.packages.len() != packages.len() {
        return Ok(false);
    }

    for package in packages {
        let found = lockfile.packages.iter().any(|lp| {
            lp.name == package.name && lp.version == package.version.to_string()
        });

        if !found {
            return Ok(false);
        }
    }

    Ok(true)
}
