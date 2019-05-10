use clap::{App, Arg};
use crc::{crc32, Hasher32};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{create_dir, rename, DirBuilder, File};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn handle_file<'a, 'b>(dir_entry: &'a DirEntry, ds: &DeeDoo<'b>) {
    let path = dir_entry.path();

    if dir_entry.file_type().is_dir() {
        return;
    }

    let mut file_content = Vec::new();
    let mut buffer = BufReader::new(File::open(path).unwrap());
    let mut digest = crc32::Digest::new(crc32::IEEE);
    /* We have path here so lets try to read the file and run CRC on it */
    let _ = buffer.read_to_end(&mut file_content);
    digest.write(&file_content);
    let crc = digest.sum32();

    println!("The crc is: {} for file {}", crc, path.display());
    if ds.hm.borrow().contains_key(&crc) {
        /* Move the file to reject directory. */
        /* TODO: Ensure the files are exactly the same by comparing the bytes? */
        println!(
            "{} is duplicate. Moving it to {}",
            path.display(),
            ds.reject_dir.display()
        );
        match move_file(&path.to_owned(), &ds.reject_dir) {
            Ok(()) => println!("Moved!"),
            Err(e) => println!("Error moving file: {}", e),
        }
    } else {
        ds.hm.borrow_mut().insert(crc, path.to_path_buf());
    }
}

fn move_file(file: &Path, dst_dir: &Path) -> Result<(), std::io::Error> {
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
        Ok(_) => println!("Created rejects dir."),
        Err(e) => println!("{}", e),
    }

    rename(duplicate, rejects_duplicate_path)
}

struct DeeDoo<'a> {
    hm: RefCell<HashMap<u32, PathBuf>>,
    reject_dir: &'a Path,
    ensure: bool,
}

fn main() {
    let matches = App::new("deedoo")
        .version("0.1")
        .author("versbinarii <versbinarii@gmail.com>")
        .about("File deduplicator")
        .arg(
            Arg::with_name("directory")
                .required(true)
                .help("Directory to scan.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("out_directory")
                .help("Directory for duplicated files.")
                .short("o")
                .long("output")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ensure")
                .help("Runs additional check to verify duplicate.")
                .short("E")
                .long("ensure"),
        )
        .arg(
            Arg::with_name("show-only")
                .help("Dont move, just display duplicate location.")
                .short("s")
                .long("show-only"),
        )
        .get_matches();

    let root_directory = matches.value_of("directory").unwrap_or(".");
    let root_directory = Path::new(root_directory);
    let default_rejects_dir = matches.value_of("out_directory").unwrap_or("rejects");
    let default_rejects_dir = Path::new(default_rejects_dir);
    let rejects = root_directory.join(default_rejects_dir);

    let ensure = matches.is_present("ensure");

    if !rejects.exists() {
        match create_dir(&rejects) {
            Ok(_) => println!("Rejects directory [{}] created", rejects.display()),
            Err(e) => {
                println!("Could not create directory for rejects: {}", e);
            }
        }
    }

    let ds = DeeDoo {
        hm: RefCell::new(HashMap::new()),
        reject_dir: &rejects,
        ensure,
    };

    WalkDir::new(&root_directory)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .filter(|e| !e.path().starts_with(&rejects))
        .for_each(|d| handle_file(&d, &ds));
}
