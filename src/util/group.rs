use chrono::{DateTime, Local};
use std::collections::HashMap;
use std::fs;

use super::{
    format::format_date,
    types::{GroupBy, NString},
};

pub fn extend_or_insert(
    mut map: HashMap<String, Vec<String>>,
    key: String,
    value: NString,
) -> HashMap<String, Vec<String>> {
    match map.get_mut(&key) {
        Some(values) => match value {
            NString::Single(value) => {
                values.push(value);
            }
            NString::Multiple(value) => {
                values.extend(value);
            }
        },
        None => match value {
            NString::Single(value) => {
                map.insert(key, vec![value]);
            }
            NString::Multiple(value) => {
                map.insert(key, value);
            }
        },
    }

    return map;
}

pub fn group_by(folder: &str, by: &GroupBy) -> std::io::Result<HashMap<String, Vec<String>>> {
    let grouped = fs::read_dir(folder).expect("Can not read folder").fold(
        HashMap::<String, Vec<String>>::new(),
        |mut acc, entry| {
            let path = entry.expect("Can not read entry").path();

            // continue if path is symlink
            if path.is_symlink() {
                return acc;
            }

            let path_str = path.to_str().expect("Can not read path").to_string();
            if path.is_dir() {
                match group_by(&path_str, by) {
                    Ok(groups) => {
                        // Accumulate the results
                        for (key, values) in groups {
                            acc = extend_or_insert(acc, key, NString::Multiple(values));
                        }
                        return acc;
                    }
                    Err(e) => {
                        println!("Can not access {}: {}", path.display(), e);
                        return acc;
                    }
                }
            } else {
                match fs::metadata(&path) {
                    Ok(metadata) => {
                        let key: String = match by {
                            GroupBy::FileExtension => {
                                let ext = path.extension();
                                match ext {
                                    Some(ext) => match ext.to_str() {
                                        Some(ext) => ext.to_owned(),
                                        None => "unknown".to_owned(),
                                    },
                                    None => "unknown".to_owned(),
                                }
                            }
                            GroupBy::Size => {
                                let size = metadata.len();
                                match size {
                                    0..=1024 => "0-1KB".to_owned(),
                                    1025..=1048576 => "1KB-1MB".to_owned(),
                                    1048577..=1073741824 => "1MB-1GB".to_owned(),
                                    _ => "1GB+".to_owned(),
                                }
                            }
                            GroupBy::CreatedAt => {
                                format_date(&DateTime::<Local>::from(metadata.created().unwrap()))
                            }
                            GroupBy::LastModifiedAt => {
                                format_date(&DateTime::<Local>::from(metadata.modified().unwrap()))
                            }
                        };
                        return extend_or_insert(acc, key, NString::Single(path_str));
                    }
                    Err(e) => {
                        println!("Can not access {}: {}", path.display(), e);
                        return acc;
                    }
                }
            }
        },
    );

    Ok(grouped)
}
