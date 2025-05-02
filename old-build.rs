// Just for reference

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]

use std::fs::write;
use std::path::{Path, PathBuf};
use std::{env, process::exit};

use git2::build::TreeUpdateBuilder;
use git2::{FileMode, ObjectType, Oid, Repository};
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};

use cargo_toml::{Manifest, Value};

use serde::Deserialize;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

macro_rules! debug {
    ($($x:tt)*) => { write("/dev/tty", { let mut x = format!($($x)*); x += "\n"; x }).unwrap() }
}

enum FileTree<T> {
    File {
        name: String,
        contents: T,
    },
    Directory {
        name: String,
        contents: Box<FileTree<T>>,
    },
}

#[derive(Deserialize, Debug, Clone)]
struct MyMetadata {
    subtree: SubtreeMetadata,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
struct SubtreeMetadata {
    default_include: bool,
    include: Vec<PathBuf>,
}

struct Program {
    this_crate_dir: PathBuf,
    top_level_dir: PathBuf,
    git_repository: Repository,
    my_metadata: MyMetadata,
}

impl Program {
    fn new() -> Self {
        let this_crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

        let manifest =
            Manifest::from_path_with_metadata(this_crate_dir.join("Cargo.toml")).unwrap();

        let git_repository = Repository::discover(&this_crate_dir).unwrap();

        let top_level_dir = git_repository.workdir().unwrap().to_str().unwrap().into();

        let my_metadata = MyMetadata::from_manifest(&manifest);

        //let /*top_level_dir*/ a = manifest.workspace.unwrap().members;

        //println!("{a:#?}");

        //panic!();

        Self {
            this_crate_dir,
            top_level_dir,
            git_repository,
            my_metadata,
        }
    }

    fn cargo_rerun(&self) {
        let paths: Vec<_> = self
            .my_metadata
            .subtree
            .include
            .iter()
            .inspect(|x| assert!(x.is_relative())) // forbid absolute paths; it would break things
            .map(|x| self.top_level_dir.join(x))
            .map(|x| pathdiff::diff_paths(x, &self.this_crate_dir).unwrap())
            .map(|x| x.to_str().unwrap().to_owned())
            .collect();

        for i in paths {
            println!("cargo:rerun-if-changed={i}");
        }
    }

    fn build_subtree(&self) {
        let mut builder = TreeUpdateBuilder::new();

        for relative_path in &self.my_metadata.subtree.include {
            let absolute_path = self.top_level_dir.join(relative_path);

            let oid = Oid::hash_file(ObjectType::Blob, absolute_path).unwrap();

            builder.upsert(relative_path, oid, FileMode::Blob);
        }

        //builder.create_updated(repo, baseline)
    }
}

impl MyMetadata {
    fn from_manifest(manifest: &Manifest<Self>) -> Self {
        let metadata: MyMetadata = manifest
            .package
            .as_ref()
            .map(|x| x.metadata.clone())
            .unwrap()
            .unwrap();

        assert!(metadata.subtree.default_include == false);

        metadata
    }
}

impl FileTree<()> {
    fn from_paths() {}
}

fn do_manifest() -> Result {
    let m: Manifest<MyMetadata> = Manifest::from_path_with_metadata(format!(
        "{}/Cargo.toml",
        env::var("CARGO_MANIFEST_DIR")?
    ))?;

    Ok(())
}

fn main() -> Result {
    debug!("aaaaaaaaaaaa");

    let prog = Program::new();

    // rerun if you change any files specified in include = [..] in
    // [package.metadata.subtree]
    prog.cargo_rerun();

    // note: cargo post doesn't set this variable yet
    let is_post = env::var("CARGO_POST").is_ok();

    if !is_post {
        println!("cargo:warning=You need to build this program using cargo post");
        println!("cargo:warning=See https://crates.io/crates/cargo-post");

        let rand_string = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect::<String>();

        // always rerun next time (so if one rerun with cargo post, this build.rs will work)
        // workaround found in https://stackoverflow.com/a/76743504
        println!("cargo:rerun-if-changed=./{}", rand_string);

        exit(1);
    }

    do_manifest()?;

    Ok(())
}
