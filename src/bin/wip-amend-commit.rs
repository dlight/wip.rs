#![allow(unused)]

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use execute::Execute;
use itertools::Itertools;
use semver::Version;
use serde_derive::{Deserialize, Serialize};

use wip::{Result, WipToml};

// TODO: change to struct Sha1([u8; 20]);
type Sha1 = String;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct BuildInfo {
    #[serde(flatten)]
    target: VersionedTarget,
    subset_tree: Sha1,
    build_warnings: Option<String>,
}

#[derive(Debug, Clone)]
struct CommitMessage {
    message: String,
    metadata: Vec<BuildInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct VersionedTarget {
    name: String,
    version: Version,
}

impl fmt::Display for VersionedTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.name, self.version)
    }
}

impl VersionedTarget {
    fn from_wip_toml(wip_toml_target: &wip::Target<Version>) -> Self {
        Self {
            name: wip_toml_target.name.clone(),
            version: wip_toml_target.version.clone(),
        }
    }
}

impl std::fmt::Display for BuildInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "successful build: {}-{} ({})",
            self.target.name, self.target.version, self.subset_tree
        )?;

        if let Some(warnings) = &self.build_warnings {
            writeln!(f, "\nbuild warnings:\n{}", warnings)?;
        }
        Ok(())
    }
}

impl CommitMessage {
    const SEPARATOR: &str = "--- ";
}

impl fmt::Display for CommitMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)?;

        write!(f, "\n\n");
        write!(f, "{}", CommitMessage::SEPARATOR);
        write!(f, "\n\n");

        let mut metadata: HashMap<&str, &[BuildInfo]> = HashMap::new();

        metadata.insert("build", &self.metadata);

        let metadata = toml::to_string(&metadata).unwrap();

        write!(f, "{}", metadata);

        Ok(())
    }
}

impl BuildInfo {
    fn from_cmdline_args(mut args: impl ExactSizeIterator<Item = String>) -> Result<Self> {
        let num_args = args.len();

        if num_args < 3 {
            eprintln!("Usage: wip-amend-commit <target wip.toml> <target name> [build warnings]");
            std::process::exit(1);
        }

        let (own_binary, wip_toml_path, target_name) = args.next_tuple().unwrap();
        let build_warnings = args.next();

        let own_binary = Path::new(&own_binary);
        let wip_toml_path = &Path::new(&wip_toml_path);
        let bin_dir = find_bin_dir(own_binary);
        let subset_tree = get_subset_tree(own_binary, wip_toml_path, &target_name)?;

        let wip_toml = WipToml::read_toml(wip_toml_path)?;
        let target = wip_toml.get_target(&target_name)?;
        let target = VersionedTarget::from_wip_toml(target);

        Ok(Self {
            target,
            build_warnings,
            subset_tree,
        })
    }
}

fn find_bin_dir(own_binary: &Path) -> Result<PathBuf> {
    let mut path = fs::canonicalize(own_binary)?;
    path.pop(); // wip-amend-commit/..
    path.pop(); // debug/..
    path.pop(); // target/..
    path.push("bin");
    Ok(path)
}

fn get_subset_tree(own_binary: &Path, wip_toml: &Path, target_name: &str) -> Result<Sha1> {
    let bin_dir = find_bin_dir(own_binary)?;

    let bin_path = bin_dir.join("wip-subset-tree");

    let mut wip_subset_tree = Command::new(bin_path);
    wip_subset_tree.arg(wip_toml);
    wip_subset_tree.arg(target_name);

    let output = wip_subset_tree.output()?;
    if output.status.success() {
        let hash = String::from_utf8_lossy(&output.stdout);
        Ok(Sha1::from(hash.trim().to_string()))
    } else {
        return Err("wip-subset-tree failed")?;
    }
}

//fn working_tree_commit()

fn main() -> Result<()> {
    let build_info = BuildInfo::from_cmdline_args(std::env::args())?;

    let commit = CommitMessage {
        message: "abc".to_owned(),
        metadata: vec![build_info],
    };

    println!("{}", commit);

    Ok(())
}
