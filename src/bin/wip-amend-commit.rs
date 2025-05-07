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
use serde_with::{DisplayFromStr, serde_as};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// TODO: change to struct Sha1([u8; 20]);
type Sha1 = String;

// Currently same type as Sha1, which is unfortunate
type ShortHash = String;

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
struct BuildInfo {
    target_name: String,
    #[serde_as(as = "DisplayFromStr")]
    version: Version,
    short_hash: ShortHash,
    build_warnings: Option<String>,
}

#[derive(Debug)]
struct CommitMessage {
    message: String,
    metadata: Vec<BuildInfo>,
}

impl std::fmt::Display for BuildInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "successful build: {}-{} ({})",
            self.target_name, self.version, self.short_hash
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

        if num_args < 4 {
            eprintln!(
                "Usage: wip-amend-commit <target wip.toml> <target name> <version> [build warnings]"
            );
            std::process::exit(1);
        }

        let (own_binary, wip_toml, target_name, version) = args.next_tuple().unwrap();
        let build_warnings = args.next();

        let own_binary = Path::new(&own_binary);

        let wip_toml = &Path::new(&wip_toml);

        let version = Version::parse(&version)?;

        let bin_dir = find_bin_dir(own_binary);

        let subset_tree = get_subset_tree(own_binary, wip_toml)?;

        let short_hash = short_hash(subset_tree)?;

        Ok(Self {
            target_name,
            version,
            build_warnings,
            short_hash,
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

fn get_subset_tree(own_binary: &Path, wip_toml: &Path) -> Result<Sha1> {
    let bin_dir = find_bin_dir(own_binary)?;

    let bin_path = bin_dir.join("wip-subset-tree");

    let mut wip_subset_tree = Command::new(bin_path);
    wip_subset_tree.arg(wip_toml);
    let output = wip_subset_tree.output()?;
    if output.status.success() {
        let hash = String::from_utf8_lossy(&output.stdout);
        Ok(Sha1::from(hash.trim().to_string()))
    } else {
        return Err("wip-subset-tree failed")?;
    }
}

fn short_hash(hash: Sha1) -> Result<ShortHash> {
    let mut git = Command::new("git");
    git.arg("rev-parse");
    git.arg("--short");
    git.arg(hash);

    let output = git.output()?;
    if output.status.success() {
        let hash = String::from_utf8_lossy(&output.stdout);
        Ok(hash.trim().to_string())
    } else {
        return Err("git rev-parse failed")?;
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
