use clap::{self, Parser};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Arguments {
    /// directory source path
    source: String,
    /// directory destination path
    destination: String,
}

fn copy_directories(source_path: &Path, dest_path: &Path) -> Result<(), std::io::Error> {
    if !source_path.is_dir() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Source path is not a directory"));
    }

    fs::create_dir_all(dest_path)?;
    for entry in fs::read_dir(source_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let new_path = dest_path.join(path.file_name().unwrap());
            fs::create_dir(new_path.clone())?;
            // Recursively copy subdirectories
            copy_directories(&path, &new_path)?;
        }
    }
    Ok(())
}

fn main() {
    let args = Arguments::parse();
    let source_path = PathBuf::from(args.source);
    let dest_path = PathBuf::from(args.destination);
    match copy_directories(&source_path, &dest_path) {
        Ok(_) => println!("Directories copied successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
