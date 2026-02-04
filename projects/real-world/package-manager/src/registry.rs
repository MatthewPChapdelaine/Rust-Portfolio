use std::collections::HashMap;
use std::path::Path;
use anyhow::{Context, Result, anyhow};
use crate::models::RegistryPackage;

pub struct Registry {
    path: String,
    packages: HashMap<String, Vec<RegistryPackage>>,
}

impl Registry {
    pub fn new(path: &str) -> Result<Self> {
        let mut registry = Self {
            path: path.to_string(),
            packages: HashMap::new(),
        };
        
        registry.load_packages()?;
        Ok(registry)
    }

    fn load_packages(&mut self) -> Result<()> {
        let registry_path = Path::new(&self.path);
        
        if !registry_path.exists() {
            return Ok(());
        }

        for entry in std::fs::read_dir(registry_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("toml") {
                let content = std::fs::read_to_string(&path)?;
                let package: RegistryPackage = toml::from_str(&content)?;
                
                self.packages
                    .entry(package.name.clone())
                    .or_insert_with(Vec::new)
                    .push(package);
            }
        }

        for versions in self.packages.values_mut() {
            versions.sort_by(|a, b| {
                let v_a = semver::Version::parse(&a.version).unwrap();
                let v_b = semver::Version::parse(&b.version).unwrap();
                v_b.cmp(&v_a)
            });
        }

        Ok(())
    }

    pub fn get_package(&self, name: &str, version_req: &str) -> Result<RegistryPackage> {
        let versions = self.packages
            .get(name)
            .ok_or_else(|| anyhow!("Package not found: {}", name))?;

        let req = semver::VersionReq::parse(version_req)
            .context("Invalid version requirement")?;

        for package in versions {
            let version = semver::Version::parse(&package.version)?;
            if req.matches(&version) {
                return Ok(package.clone());
            }
        }

        Err(anyhow!("No matching version found for {} {}", name, version_req))
    }

    pub fn list_packages(&self) -> Result<HashMap<String, Vec<String>>> {
        let mut result = HashMap::new();
        
        for (name, versions) in &self.packages {
            let version_strings: Vec<String> = versions
                .iter()
                .map(|p| p.version.clone())
                .collect();
            result.insert(name.clone(), version_strings);
        }
        
        Ok(result)
    }

    pub fn search(&self, query: &str) -> Result<HashMap<String, RegistryPackage>> {
        let mut results = HashMap::new();
        let query_lower = query.to_lowercase();
        
        for (name, versions) in &self.packages {
            if name.to_lowercase().contains(&query_lower) {
                if let Some(latest) = versions.first() {
                    results.insert(name.clone(), latest.clone());
                }
            } else if let Some(latest) = versions.first() {
                if let Some(desc) = &latest.description {
                    if desc.to_lowercase().contains(&query_lower) {
                        results.insert(name.clone(), latest.clone());
                    }
                }
            }
        }
        
        Ok(results)
    }

    pub fn get_package_info(&self, name: &str) -> Result<RegistryPackage> {
        let versions = self.packages
            .get(name)
            .ok_or_else(|| anyhow!("Package not found: {}", name))?;

        versions.first()
            .cloned()
            .ok_or_else(|| anyhow!("No versions available for {}", name))
    }
}
