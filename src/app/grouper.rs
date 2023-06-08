use crate::util;

use util::group::{extend_or_insert, group_by};
use util::types::{GroupBy, NString};

use std::collections::HashMap;
use std::fs;
use std::thread;

pub fn grouper(folder: &String) -> HashMap<String, Vec<String>> {
    println!("Scanning {}", folder);

    let jobs = fs::read_dir(folder)
        .expect("Can not read dir")
        .filter_map(|entry| {
            let path = entry.expect("Can not read entry").path();
            let sub_folder = path.to_str().expect("Can not read").to_string();
            if path.is_dir() {
                return Some(thread::spawn(move || {
                    // match size_and_count(&sub_folder, 0, 0) {
                    //     Ok(sc) => sc,
                    //     Err(e) => {
                    //         println!("{}: {}", path.display(), e);
                    //         (0, 0)
                    //     }
                    // }

                    match group_by(&sub_folder, &GroupBy::LastModifiedAt) {
                        Ok(groups) => groups,
                        Err(e) => {
                            println!("{}: {}", path.display(), e);
                            HashMap::new()
                        }
                    }
                }));
            }
            None
        });

    // let sum = jobs.fold((0, 0), |acc, job| {
    //     let (size, count) = job.join().unwrap();
    //     (acc.0 + size, acc.1 + count)
    // });
    // println!("{} files, {} bytes", sum.1, sum.0);

    let groups = jobs.fold(HashMap::<String, Vec<String>>::new(), |mut acc, job| {
        let groups = job.join().unwrap();
        // Accumulate the results
        for (key, values) in groups {
            acc = extend_or_insert(acc, key, NString::Multiple(values));
        }
        acc
    });

    groups
}
