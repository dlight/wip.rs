use std::collections::HashSet;
use std::fs::canonicalize;
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};
use walkdir::WalkDir;

struct Config {
    wip: WipToml,
    path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct WipToml {
    depends_on: WipDependency,
}

#[derive(Debug, Serialize, Deserialize)]
struct WipDependency {
    include: Vec<PathBuf>,
    exclude: Vec<PathBuf>,
}
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Config {
    fn read_toml(file_path: &Path) -> Result<Config> {
        let content = std::fs::read_to_string(file_path)?;
        let mut toml: WipToml = toml::from_str(&content)?;

        // Ensure paths are relative
        for path in &mut toml.depends_on.include {
            if path.is_absolute() {
                Err(format!(
                    "Absolute path found in depends_on.include: {}",
                    path.display()
                ))?;
            }

            // TODO: also ensure that we can't point outside the current git repository
        }

        Ok(Config {
            wip: toml,
            path: file_path.to_path_buf(),
        })
    }
    fn all_files(&self) -> Result<HashSet<PathBuf>> {
        let dir = self.path.parent().ok_or("No parent directory")?;
        let mut included = HashSet::new();
        let mut excluded = HashSet::new();

        for relative_path in &self.wip.depends_on.exclude {
            for entry in WalkDir::new(dir.join(relative_path)) {
                // TODO: use let chains when it stabilizes
                if let Ok(entry) = entry {
                    let path = canonicalize(entry.path())?;
                    if !entry.file_type().is_dir() {
                        excluded.insert(path);
                    }
                }
            }
        }

        for relative_path in &self.wip.depends_on.include {
            for entry in WalkDir::new(dir.join(relative_path)) {
                // TODO: use let chains when it stabilizes
                if let Ok(entry) = entry {
                    let path = canonicalize(entry.path())?;
                    if !entry.file_type().is_dir() && !excluded.contains(&path) {
                        included.insert(path);
                    }
                }
            }
        }

        Ok(included)
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: wip-read-toml <file>");
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);

    let config = Config::read_toml(file_path)?;

    let files = config.all_files()?;

    for file in files {
        println!("{}", file.display());
    }

    Ok(())
}
