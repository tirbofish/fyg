use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::FygBinaryType;

/// Project metadata configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub group: String,
    pub version: String,
    #[serde(default)]
    pub authors: Option<Vec<String>>,
    #[serde(default)]
    pub description: Option<String>,
}

/// Build configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuildConfig {
    #[serde(default)]
    pub multiplatform: Option<bool>,
    #[serde(default)]
    pub languages: Option<Vec<String>>,
}

/// Targets configuration for multiplatform builds
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TargetsConfig {
    #[serde(default)]
    pub jvm: Option<JvmTarget>,
    #[serde(default, rename = "ios-arm64")]
    pub ios_arm64: Option<TargetEnabled>,
    #[serde(default, rename = "ios-x64")]
    pub ios_x64: Option<TargetEnabled>,
    #[serde(default, rename = "ios-simulator-arm64")]
    pub ios_simulator_arm64: Option<TargetEnabled>,
    #[serde(default, rename = "linux-x64")]
    pub linux_x64: Option<TargetEnabled>,
    #[serde(default, rename = "macos-arm64")]
    pub macos_arm64: Option<TargetEnabled>,
    #[serde(default, rename = "windows-x64")]
    pub windows_x64: Option<TargetEnabled>,
    #[serde(default)]
    pub native: Option<NativeConfig>,
}

/// JVM target configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JvmTarget {
    pub enabled: bool,
    #[serde(default)]
    pub target: Option<String>,
}

/// Simple enabled flag for targets
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TargetEnabled {
    pub enabled: bool,
}

/// Native binary configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NativeConfig {
    #[serde(default)]
    pub binary: Option<NativeBinaryConfig>,
}

/// Native binary type and base name
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NativeBinaryConfig {
    #[serde(rename = "type")]
    pub binary_type: FygBinaryType,
    #[serde(default, rename = "base-name")]
    pub base_name: Option<String>,
}

/// Dependencies configuration supporting common, jvm, and test scopes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DependenciesConfig {
    #[serde(default)]
    pub common: Option<HashMap<String, DependencyValue>>,
    #[serde(default)]
    pub jvm: Option<HashMap<String, DependencyValue>>,
    #[serde(default)]
    pub test: Option<HashMap<String, DependencyValue>>,
}

/// A dependency can be a version string or a complex object
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DependencyValue {
    Version(String),
    Complex(DependencyConfig),
}

/// Complex dependency configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DependencyConfig {
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub workspace: Option<bool>,
    #[serde(default)]
    pub version: Option<String>,
}

/// Test configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestConfig {
    #[serde(default)]
    pub framework: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryConfig {
    Enabled(bool),
    Custom(CustomRepository),
}

/// Custom repository configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomRepository {
    #[serde(rename = "type")]
    pub repo_type: String,
    pub url: String,
}