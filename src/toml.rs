use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};

use crate::config::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FygToml {
    pub project: ProjectConfig,
    #[serde(default)]
    pub build: Option<BuildConfig>,
    #[serde(default)]
    pub targets: Option<TargetsConfig>,
    #[serde(default)]
    pub dependencies: Option<DependenciesConfig>,
    #[serde(default)]
    pub test: Option<TestConfig>,
    #[serde(default)]
    pub repositories: Option<HashMap<String, RepositoryConfig>>,
}

impl FygToml {
    pub fn new(name: &str, group: &str) -> Self {
        Self {
            project: ProjectConfig {
                name: name.to_string(),
                group: group.to_string(),
                version: "1.0.0-SNAPSHOT".to_string(),
                authors: None,
                description: None,
            },
            build: None,
            targets: None,
            dependencies: None,
            test: None,
            repositories: None,
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_str(&content)
    }

    pub fn from_str(content: &str) -> anyhow::Result<Self> {
        let config: FygToml = toml::from_str(content)?;
        Ok(config)
    }

    pub fn to_toml_string(&self) -> anyhow::Result<String> {
        let content = toml::to_string_pretty(self)?;
        Ok(content)
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let content = self.to_toml_string()?;
        std::fs::write(path, content)?;
        Ok(())
    }
}