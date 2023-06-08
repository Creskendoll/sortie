use std::fs;

// Recursively list all files in a folder
fn size_and_count(folder: &str, size: u64, count: u64) -> std::io::Result<(u64, u64)> {
    let sum = fs::read_dir(folder)?.fold((size, count), |acc, entry| {
        let path = entry.expect("Can not read entry").path();

        // continue if path is symlink
        if path.is_symlink() {
            return acc;
        }

        if path.is_dir() {
            let path_str = path.to_str().expect("Can not read path").to_string();
            match size_and_count(&path_str, size, count) {
                Ok(sc) => return (acc.0 + sc.0, acc.1 + sc.1),
                Err(e) => {
                    println!("Can not access {}: {}", path.display(), e);
                    return acc;
                }
            }
        } else {
            match fs::metadata(&path) {
                Ok(metadata) => return (acc.0 + metadata.len(), acc.1 + 1),
                Err(e) => {
                    println!("Can not access {}: {}", path.display(), e);
                    return acc;
                }
            }
        }
    });

    Ok(sum)
}
