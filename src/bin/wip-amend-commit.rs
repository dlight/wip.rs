#![allow(unused)]

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

use boolinator::Boolinator;
use execute::Execute;
use itertools::Itertools;
use semver::Version;
use serde_derive::{Deserialize, Serialize};

use toml::Value;
use wip::{Error, Result, Sha1, WipToml};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct BuildInfo {
    #[serde(flatten)]
    target: VersionedTarget,
    subset_tree: Sha1,
    build_warnings: Option<String>,
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

#[derive(Debug, Clone)]
struct CommitMessage {
    message: String,
    metadata: Vec<BuildInfo>,
}

impl CommitMessage {
    const SEPARATOR: &str = "\n\n--- something\n\n";
}

impl fmt::Display for CommitMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)?;

        write!(f, "{}", CommitMessage::SEPARATOR);

        let mut metadata: HashMap<&str, &[BuildInfo]> = HashMap::new();

        metadata.insert("build", &self.metadata);

        let metadata = toml::to_string(&metadata).unwrap();

        write!(f, "{}", metadata);

        Ok(())
    }
}

impl FromStr for CommitMessage {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some(sep_offset) = s.rfind(CommitMessage::SEPARATOR) else {
            return Ok(CommitMessage {
                message: s.to_string(),
                metadata: vec![],
            });
        };

        let message = s[0..sep_offset].to_string();

        let sep_end = sep_offset + CommitMessage::SEPARATOR.len();

        let toml_str = &s[sep_end..s.len()];

        // the toml crate can serialize a `Vec<BuildInfo>` as [[build]] .. but
        // can't deserialize it without wrapping it in a struct with a build field
        #[derive(Deserialize)]
        struct BuildRead {
            build: Vec<BuildInfo>,
        }

        let BuildRead { build }: BuildRead = toml::from_str(toml_str)?;

        Ok(CommitMessage {
            message,
            metadata: build,
        })
    }
}

#[derive(Debug, Clone)]
enum WorkingTreeCommit {
    Wip(Sha1),
    Head(Sha1),
}

#[derive(Debug, Clone)]
struct Amend {
    commit: WorkingTreeCommit,
    commit_message: CommitMessage,
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
    let bin_path = find_bin_dir(own_binary)?.join("wip-subset-tree");

    let output = Command::new(bin_path.clone())
        .arg(wip_toml)
        .arg(target_name)
        .output()?;

    output.status.success().ok_or("wip-subset-tree failed")?;

    let hash = String::from_utf8(output.stdout)?;

    Ok(Sha1::from(hash.trim().to_string()))
}

fn working_tree_dirty() -> Result<bool> {
    let output = Command::new("git")
        .args(&["status", "--porcelain"])
        .output()?;

    output.status.success().ok_or("git status failed")?;

    let no_changes = String::from_utf8(output.stdout)?.trim().is_empty();

    Ok(!no_changes)
}

fn working_tree_commit(own_binary: &Path, git_repo: &Path) -> Result<WorkingTreeCommit> {
    let bin_path = find_bin_dir(own_binary)?.join("git-working-tree");

    let output = Command::new(bin_path)
        .arg("--directory")
        .arg(git_repo)
        .output()?;

    output.status.success().ok_or("git-working-tree failed")?;

    let stdout = String::from_utf8(output.stdout)?;

    let (kind, hash) = stdout.trim().split_whitespace().next_tuple().unwrap();

    match kind {
        "wip" => Ok(WorkingTreeCommit::Wip(hash.to_string())),
        "head" => Ok(WorkingTreeCommit::Head(hash.to_string())),
        _ => Err(format!("commit is neither wip nor head: {kind}"))?,
    }
}

fn resolve_git_tag(tag: &str) -> Result<Option<Sha1>> {
    use std::ops::Not;
    use std::str::from_utf8;

    let output = Command::new("git")
        .arg("rev-parse")
        .arg(format!("refs/tags/{}", tag))
        .output()?;

    output.status.success().ok_or("git rev-parse failed")?;

    let sha1 = from_utf8(&output.stdout)?.trim();
    let not_empty = !sha1.is_empty();

    let sha1 = not_empty.then(|| sha1.to_string());

    Ok(sha1)
}

fn okay(s: &str) -> Result<(String, Option<Value>)> {
    let Some(sep_offset) = s.rfind(CommitMessage::SEPARATOR) else {
        return Ok((s.to_string(), None));
    };

    let message = s[0..sep_offset].to_string();

    let sep_end = sep_offset + CommitMessage::SEPARATOR.len();

    let toml_str = &s[sep_end..s.len()];
    let metadata = toml::from_str(toml_str)?;

    Ok((message, metadata))
}

fn main() -> Result<()> {
    let build_info = BuildInfo::from_cmdline_args(std::env::args())?;

    let commit = CommitMessage {
        message: "abc".to_owned(),
        metadata: vec![build_info.clone(), build_info.clone()],
    };

    println!("{:#?}", commit);

    println!("\n=========\n");

    let s = format!("{}", commit);

    println!("{}", s);

    println!("\n=========\n");

    let (a, b) = okay(&s)?;

    println!("{:#?}", (a, b.unwrap()));

    println!("\n=========\n");

    let a = s.parse::<CommitMessage>()?;

    println!("{:#?}", a);

    Ok(())
}
