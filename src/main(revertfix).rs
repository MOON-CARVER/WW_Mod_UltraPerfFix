use std::fs;
use std::path::PathBuf;
use std::env;
use std::io;
use rfd::FileDialog;

const BACKUP_SUFFIX: &str = "ini_bakcup_from_ultraperf_fixer_tool";

fn select_folder() -> Option<PathBuf> {
    FileDialog::new().pick_folder()
}

fn find_ultraperffixed_ini_files(root_folder: &PathBuf) -> Vec<PathBuf> {
    let mut ini_files = Vec::new();
    if let Ok(entries) = fs::read_dir(root_folder) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                ini_files.extend(find_ultraperffixed_ini_files(&path));
            } else if let Some(extension) = path.extension() {
                if extension == "ini" {
                    ini_files.push(path);
                }
            }
        }
    }
    ini_files
}

fn revert_ini_file(file_path: &PathBuf) {
    let mut backup_path = file_path.clone();
    backup_path.set_extension(format!("{}", BACKUP_SUFFIX));
    
    if backup_path.exists() {
        if let Err(e) = fs::copy(&backup_path, &file_path) {
            eprintln!("Failed to restore {}: {}", file_path.display(), e);
        } else {
            if let Err(e) = fs::remove_file(&backup_path) {
                eprintln!("Failed to remove backup {}: {}", backup_path.display(), e);
            } else {
                println!("Reverted changes for {}", file_path.display());
            }
        }
    } else {
        println!("Backup not found for {}, skipping.", file_path.display());
    }
}

fn main() {
    if let Some(root_folder) = select_folder() {
        let ini_files = find_ultraperffixed_ini_files(&root_folder);
        for ini_file in ini_files {
            revert_ini_file(&ini_file);
        }
        println!("Reversion complete.");
    } else {
        println!("No folder selected.");
    }

    println!("Press Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
