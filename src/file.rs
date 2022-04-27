use crate::DeeDoo;
use crc::{crc32, Hasher32};
use std::fs::{rename, DirBuilder, File};
use std::io::{BufReader, Read};
use std::path::Path;
use walkdir::DirEntry;

pub fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with('.'))
        .unwrap_or(false)
}

pub fn handle_file<'a>(dir_entry: &'a DirEntry, ds: &DeeDoo<'_>) {
    let path = dir_entry.path();

    if dir_entry.file_type().is_dir() {
        return;
    }

    let mut digest = crc32::Digest::new(crc32::IEEE);
    let current_file_content = get_file_content(path);
    digest.write(&current_file_content);
    let crc = digest.sum32();

    if ds.verbose {
        println!("The crc is: {} for file {}", crc, path.display());
    }

    if ds.hm.borrow().contains_key(&crc) {
        let mut ok_to_reject = true;
        let ds_borrow = ds.hm.borrow();
        let previous_file_path = ds_borrow.get(&crc).unwrap();

        if ds.ensure {
            let previous_file_content = get_file_content(previous_file_path);

            if previous_file_content != current_file_content {
                ok_to_reject = false;
            }
        }

        if ok_to_reject {
            println!(
                "{} is duplicate of {}. Moving it to rejects.",
                path.display(),
                previous_file_path.display()
            );
            match move_file(path, ds.reject_dir) {
                Ok(()) => {}
                Err(e) => eprintln!("Error moving file: {}", e),
            }
        }
    } else {
        ds.hm.borrow_mut().insert(crc, path.to_path_buf());
    }
}

pub fn move_file(file: &Path, dst_dir: &Path) -> Result<(), std::io::Error> {
    let mut duplicate = file;
    if file.starts_with(".") {
        /* Should be safe to unwrap() */
        duplicate = file.strip_prefix(".").unwrap();
    }

    /* we have a file in i.e. /home/user/document/file
    we need to move it to /home/user/document/rejects/home/user/document/file
    */

    let rejects_duplicate_path = {
        let mut non_abs_duplicate = duplicate;
        if duplicate.starts_with("/") {
            non_abs_duplicate = duplicate.strip_prefix("/").unwrap();
        }
        dst_dir.join(non_abs_duplicate)
    };
    let rejects_duplicate_dir = { rejects_duplicate_path.parent().unwrap() };

    match DirBuilder::new()
        .recursive(true)
        .create(rejects_duplicate_dir)
    {
        Ok(_) => {}
        Err(e) => eprintln!(
            "Failed to create directory: [{}]. Error: {}",
            rejects_duplicate_dir.display(),
            e
        ),
    }

    rename(duplicate, rejects_duplicate_path)
}

fn get_file_content(path: &Path) -> Vec<u8> {
    let mut buffer = BufReader::new(File::open(path).unwrap());
    let mut file_content = Vec::new();
    let _ = buffer.read_to_end(&mut file_content);
    file_content
}
