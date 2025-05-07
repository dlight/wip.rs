use std::collections::HashSet;
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use semver::Version;
use serde::{Serialize, de::DeserializeOwned};
use serde_derive::{Deserialize, Serialize};
use walkdir::WalkDir;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub type WipTomlUnevaluated = WipTomlBase<UnevaluatedVersion>;
pub type WipToml = WipTomlBase<Version>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "V: Serialize + DeserializeOwned")]
pub struct WipTomlBase<V: Serialize + DeserializeOwned + Debug> {
    pub target: Vec<Target<V>>,
    #[serde(skip)]
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "V: Serialize + DeserializeOwned")]
pub struct Target<V: Serialize + DeserializeOwned + Debug> {
    pub name: String,
    pub version: V,
    pub influences_build: InfluencesBuild,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnevaluatedVersion {
    Direct(Version),
    FromToml(VersionFrom),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionFrom {
    from: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfluencesBuild {
    pub include: Vec<PathBuf>,
    pub exclude: Vec<PathBuf>,
}

impl WipTomlUnevaluated {
    fn eval_version(self) -> Result<WipToml> {
        fn read_version(base: &Path, mut cargo_toml_path: PathBuf) -> Result<Version> {
            use std::ffi::OsStr;

            if cargo_toml_path.is_relative() {
                // TODO: make sure this doesn't point outside the repository
                cargo_toml_path = base.join(&cargo_toml_path);
            }

            if cargo_toml_path.file_name() != Some(OsStr::new("Cargo.toml")) {
                Err(format!(
                    "Can only extract version from Cargo.toml: can't read from {}",
                    cargo_toml_path.display()
                ))?;
            }

            let cargo_toml = toml::Value::from_str(&fs::read_to_string(cargo_toml_path)?)?;

            let version = cargo_toml
                .get("package")
                .ok_or("No package section in Cargo.toml")?
                .get("version")
                .ok_or("No version field in Cargo.toml")?
                .as_str()
                .ok_or("Version field is not string in Cargo.toml")?;

            let version = Version::parse(version)?;

            Ok(version)
        }

        fn map_version(base: &Path, version: UnevaluatedVersion) -> Result<Version> {
            match version {
                UnevaluatedVersion::Direct(v) => Ok(v),
                UnevaluatedVersion::FromToml(VersionFrom { from: v }) => read_version(base, v),
            }
        }

        let Self { target, path } = self;

        let base = path
            .parent()
            .ok_or(format!("wip.toml is expected to be a file"))?;

        let new_target = target
            .into_iter()
            .map(
                |Target {
                     name,
                     version,
                     influences_build,
                 }| {
                    Ok(Target {
                        name,
                        version: map_version(base, version)?,
                        influences_build,
                    })
                },
            )
            .collect::<Result<Vec<_>>>()?;

        Ok(WipToml {
            path,
            target: new_target,
        })
    }
}

impl WipToml {
    pub fn read_toml(file_path: &Path) -> Result<Self> {
        let content = fs::read_to_string(file_path)?;
        let mut toml_uneval: WipTomlUnevaluated = toml::from_str(&content)?;
        toml_uneval.path = file_path.to_owned();

        let mut toml = toml_uneval.eval_version()?;

        // Ensure paths are relative
        for target in &mut toml.target {
            for path in &mut target.influences_build.include {
                if path.is_absolute() {
                    Err(format!(
                        "Absolute path found in influences_build.include: {}",
                        path.display()
                    ))?;
                }

                // TODO: also ensure that we can't point outside the current git repository
            }
        }

        Ok(toml)
    }

    pub fn get_target(&self, target_name: &str) -> Result<&Target<Version>> {
        Ok(self
            .target
            .iter()
            .find(|t| t.name == target_name)
            .ok_or(format!("Target {target_name} not found"))?)
    }

    pub fn all_files(&self, target_name: &str) -> Result<HashSet<PathBuf>> {
        let dir = self.path.parent().ok_or("No parent directory")?;
        let mut included = HashSet::new();
        let mut excluded = HashSet::new();

        let target = self.get_target(target_name)?;

        for relative_path in &target.influences_build.exclude {
            for entry in WalkDir::new(dir.join(relative_path)) {
                // TODO: use let chains when it stabilizes
                if let Ok(entry) = entry {
                    let path = fs::canonicalize(entry.path())?;
                    if !entry.file_type().is_dir() {
                        excluded.insert(path);
                    }
                }
            }
        }

        for relative_path in &target.influences_build.include {
            for entry in WalkDir::new(dir.join(relative_path)) {
                // TODO: use let chains when it stabilizes
                if let Ok(entry) = entry {
                    let path = fs::canonicalize(entry.path())?;
                    if !entry.file_type().is_dir() && !excluded.contains(&path) {
                        included.insert(path);
                    }
                }
            }
        }

        Ok(included)
    }
}
