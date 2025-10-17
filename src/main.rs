use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use walkdir::WalkDir;
use std::collections::HashSet;

fn main() {
    println!("=== PhotoRec File Organizer ===\n");
    println!("1. Remove files smaller than 10KB");
    println!("2. Organize files by extension");
    println!("3. Both (remove small files, then organize)");
    println!("\nChoose an option (1-3): ");

    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => remove_small_files(),
        "2" => organize_files(),
        "3" => {
            remove_small_files();
            println!("\n");
            organize_files();
        }
        _ => println!("Invalid option!"),
    }
}

fn remove_small_files() {
    let start_path = std::env::current_dir().expect("Failed to get current directory");
    let min_size: u64 = 20 * 1024; // 10 KB

    println!("\n=== Removing files smaller than 10KB ===");
    println!("Scanning directory: {}\n", start_path.display());

    let mut removed_count = 0;
    let mut removed_size: u64 = 0;
    let mut error_count = 0;

    // Lista de diretórios e arquivos a ignorar
    let ignore_dirs = ["target", "src", ".git", "node_modules"];
    let ignore_files = ["Cargo.toml", "Cargo.lock"];

    for entry in WalkDir::new(&start_path)
        .into_iter()
        .filter_entry(|e| {
            // Ignora diretórios específicos
            if e.path().is_dir() {
                if let Some(name) = e.path().file_name() {
                    return !ignore_dirs.contains(&name.to_string_lossy().as_ref());
                }
            }
            true
        })
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        // Ignora arquivos específicos
        if let Some(name) = path.file_name() {
            if ignore_files.contains(&name.to_string_lossy().as_ref()) {
                continue;
            }
        }

        match fs::metadata(path) {
            Ok(metadata) => {
                let file_size = metadata.len();

                if file_size < min_size {
                    match fs::remove_file(path) {
                        Ok(_) => {
                            println!("Removed: {} ({} bytes)", path.display(), file_size);
                            removed_count += 1;
                            removed_size += file_size;
                        }
                        Err(e) => {
                            eprintln!("Error removing {}: {}", path.display(), e);
                            error_count += 1;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading metadata for {}: {}", path.display(), e);
                error_count += 1;
            }
        }
    }

    println!("\n=== Summary ===");
    println!("Files removed: {}", removed_count);
    println!("Total size freed: {} bytes ({:.2} KB)", removed_size, removed_size as f64 / 1024.0);
    println!("Errors encountered: {}", error_count);
}

fn organize_files() {
    let start_path = std::env::current_dir().expect("Failed to get current directory");

    let known_extensions = vec!["png", "jpg", "jpeg", "zip", "mov", "gif", "mp3", "mp4", "mkv"];
    let mut all_folders = known_extensions.clone();
    all_folders.push("others");

    // Diretórios a ignorar
    let ignore_dirs = ["target", "src", ".git", "node_modules"];

    println!("\n=== Organizing files by extension ===");
    println!("Working in: {}\n", start_path.display());

    // Step 1: Create target directories
    println!("Creating target directories...");
    for ext in &all_folders {
        let dir_path = start_path.join(ext);
        match fs::create_dir_all(&dir_path) {
            Ok(_) => println!("  [OK] {}", ext),
            Err(e) => eprintln!("  [ERROR] {}: {}", ext, e),
        }
    }
    println!();

    // Step 2: Collect all files to move
    println!("Scanning files...");
    let mut files_to_move: Vec<(PathBuf, String)> = Vec::new();

    for entry in WalkDir::new(&start_path)
        .min_depth(1)
        .into_iter()
        .filter_entry(|e| {
            // Ignora diretórios específicos
            if e.path().is_dir() {
                if let Some(name) = e.path().file_name() {
                    let name_str = name.to_string_lossy();
                    return !ignore_dirs.contains(&name_str.as_ref())
                        && !all_folders.contains(&name_str.as_ref());
                }
            }
            true
        })
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        // Skip Cargo files
        if let Some(name) = path.file_name() {
            let name_str = name.to_string_lossy();
            if name_str == "Cargo.toml" || name_str == "Cargo.lock" {
                continue;
            }
        }

        // Determine target folder
        let target_folder = if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            if known_extensions.contains(&ext_str.as_str()) {
                ext_str.to_string()
            } else {
                "others".to_string()
            }
        } else {
            "others".to_string()
        };

        files_to_move.push((path.to_path_buf(), target_folder));
    }

    println!("Found {} files to organize\n", files_to_move.len());

    // Step 3: Move files
    println!("Moving files...");
    let mut moved_count = 0;
    let mut skipped_count = 0;

    for (source_path, target_folder) in files_to_move {
        let file_name = source_path.file_name().unwrap();
        let mut dest_path = start_path.join(&target_folder).join(file_name);

        // Handle duplicate filenames
        let mut counter = 1;
        while dest_path.exists() {
            let stem = source_path.file_stem().unwrap().to_string_lossy();
            let ext = source_path.extension()
                .map(|e| format!(".{}", e.to_string_lossy()))
                .unwrap_or_default();
            let new_name = format!("{}_{}{}", stem, counter, ext);
            dest_path = start_path.join(&target_folder).join(new_name);
            counter += 1;
        }

        match fs::rename(&source_path, &dest_path) {
            Ok(_) => {
                println!("  {} -> {}/", file_name.to_string_lossy(), target_folder);
                moved_count += 1;
            }
            Err(e) => {
                eprintln!("  [ERROR] {}: {}", source_path.display(), e);
                skipped_count += 1;
            }
        }
    }

    println!();

    // Step 4: Remove empty directories
    println!("Removing empty directories...");
    let mut removed_dirs = 0;

    let mut protected_dirs: HashSet<PathBuf> = all_folders
        .iter()
        .map(|ext| start_path.join(ext))
        .collect();

    // Adiciona diretórios do projeto a lista de proteção
    for dir in &ignore_dirs {
        protected_dirs.insert(start_path.join(dir));
    }

    let mut all_dirs: Vec<PathBuf> = WalkDir::new(&start_path)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.path().to_path_buf())
        .collect();

    // Sort by depth (deepest first)
    all_dirs.sort_by(|a, b| {
        let depth_a = a.components().count();
        let depth_b = b.components().count();
        depth_b.cmp(&depth_a)
    });

    for dir_path in all_dirs {
        if protected_dirs.contains(&dir_path) {
            continue;
        }

        match fs::read_dir(&dir_path) {
            Ok(mut entries) => {
                if entries.next().is_none() {
                    match fs::remove_dir(&dir_path) {
                        Ok(_) => {
                            println!("  Removed: {}", dir_path.file_name().unwrap().to_string_lossy());
                            removed_dirs += 1;
                        }
                        Err(e) => {
                            eprintln!("  [ERROR] removing {}: {}", dir_path.display(), e);
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }

    println!();
    println!("=== Summary ===");
    println!("Files moved: {}", moved_count);
    println!("Files skipped: {}", skipped_count);
    println!("Empty directories removed: {}", removed_dirs);
}
