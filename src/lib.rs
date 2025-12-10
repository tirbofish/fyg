pub mod toml;
pub mod config;

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::toml::FygToml;

/// Fyg is the build tool used for building JVM based apps. It serves as a simpler replacement 
/// to other build tools such as Gradle and Maven. 
pub struct Fyg;

impl Fyg {
    /// Creates a new folder and initialises the new folder with a new fyg project.
    /// 
    /// This will create a new directory with the project name inside the given folder path,
    /// then initialise it with the fyg project structure.
    pub fn new<P: AsRef<Path>>(folder_path: P, fyg_toml: FygToml) -> anyhow::Result<()> {
        let new_path = folder_path.as_ref().join(&fyg_toml.project.name);
        std::fs::create_dir_all(&new_path)?;
        Self::init(&new_path, fyg_toml)?;
        Ok(())
    }

    /// Initialises an existing folder to be ready for Fyg compilation. 
    /// 
    /// This requires you to pass in the [`Path`] of the folder and a [`FygToml`] configuration.
    /// It will populate the folder with the necessary project structure.
    pub fn init<P: AsRef<Path>>(folder_path: P, fyg_toml: FygToml) -> anyhow::Result<()> {
        let path = folder_path.as_ref().to_path_buf();

        fyg_toml.write_to_file(path.join("fyg.toml"))?;

        let src_path = path
            .join("src")
            .join("kotlin")
            .join(fyg_toml.project.group.replace('.', "/"));
        std::fs::create_dir_all(&src_path)?;

        Ok(())
    }

    pub fn build(_config_file_path: impl AsRef<Path>) -> anyhow::Result<()> {
        todo!("Not implemented yet :(")
    }
}

/// The type of project that will be exported when the project is built. 
/// 
/// This is shown at https://kotlinlang.org/docs/multiplatform/multiplatform-build-native-binaries.html#declare-binaries
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FygBinaryType {
    /// Factory method: `executable`
    Executable,

    /// Factory method: `test`
    Test,

    /// Factory method: `sharedLib`
    SharedLib,

    /// Factory method: `staticLib`
    StaticLib,

    /// Factory method: `framework`
    Framework,
}

impl std::fmt::Display for FygBinaryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FygBinaryType::Executable => "executable",
            FygBinaryType::Test => "test",
            FygBinaryType::SharedLib => "sharedLib",
            FygBinaryType::StaticLib => "staticLib",
            FygBinaryType::Framework => "framework",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use crate::toml::FygToml;

    #[test]
    fn test_parse_fyg_toml() {
        let config = FygToml::from_file("fyg.toml").expect("Failed to parse fyg.toml");
        
        assert_eq!(config.project.name, "dropbear");
        assert_eq!(config.project.group, "com.dropbear");
        assert_eq!(config.project.version, "1.0.0-SNAPSHOT");
        
        let build = config.build.expect("build section should exist");
        assert_eq!(build.multiplatform, Some(true));
        assert_eq!(build.languages, Some(vec!["kotlin".to_string(), "java".to_string()]));
        
        let targets = config.targets.expect("targets section should exist");
        let jvm = targets.jvm.expect("jvm target should exist");
        assert!(jvm.enabled);
        assert_eq!(jvm.target, Some("17".to_string()));
    }

    #[test]
    fn test_create_new_fyg_toml() {
        let config = FygToml::new("my-app", "com.example");
        
        assert_eq!(config.project.name, "my-app");
        assert_eq!(config.project.group, "com.example");
        assert_eq!(config.project.version, "1.0.0-SNAPSHOT");
    }

    #[test]
    fn test_roundtrip_serialization() {
        let config = FygToml::new("test-app", "org.test");
        let toml_str = config.to_toml_string().expect("Failed to serialize");
        let parsed = FygToml::from_str(&toml_str).expect("Failed to parse back");
        
        assert_eq!(parsed.project.name, config.project.name);
        assert_eq!(parsed.project.group, config.project.group);
    }
}