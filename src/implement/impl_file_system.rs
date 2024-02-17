use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

pub fn print_dir_tree(
    path: &str,
    max_depth: Option<usize>,
    current_depth: usize,
    prefix: String,
    excludes: &[String], // Add parameter for excluded directories
) -> io::Result<()> {
    if let Some(max_depth) = max_depth {
        if current_depth > max_depth {
            return Ok(());
        }
    }

    let path = Path::new(path);
    if path.is_dir() {
        let entries = fs::read_dir(path)?
            .filter_map(Result::ok)
            .collect::<Vec<DirEntry>>();
        for (i, entry) in entries.iter().enumerate() {
            let last = i == entries.len() - 1;
            let entry_path = entry.path();
            let metadata = fs::metadata(&entry_path)?;

            // Skip excluded directories
            if excludes.contains(&entry.file_name().to_string_lossy().to_string()) {
                continue;
            }

            let new_prefix = if last { "└─ " } else { "├─ " };
            let continuation_prefix = if last { "    " } else { "│   " };

            println!("{}{}{}", prefix, new_prefix, entry.file_name().to_string_lossy());

            if metadata.is_dir() {
                let new_prefix = format!("{}{}", prefix, continuation_prefix);
                print_dir_tree(
                    &entry_path.to_string_lossy(),
                    max_depth,
                    current_depth + 1,
                    new_prefix,
                    excludes, // Pass excludes to recursive calls
                )?;
            }
        }
    }
    Ok(())
}
