use std::path::Path;

use wip::{Result, WipToml};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: wip-read-toml <file> <target>");
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);

    let target = &args[2];

    let toml = WipToml::read_toml(file_path)?;

    let files = toml.all_files(target)?;

    for file in files {
        println!("{}", file.display());
    }

    Ok(())
}
