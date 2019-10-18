mod args;
mod file;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::create_dir;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use args::get_args;
use file::{handle_file, is_not_hidden};

pub struct DeeDoo<'a> {
    hm: RefCell<HashMap<u32, PathBuf>>,
    reject_dir: &'a Path,
    ensure: bool,
    verbose: bool,
}

fn main() {
    let matches = get_args();

    let root_directory = matches.value_of("directory").unwrap_or(".");
    let root_directory = Path::new(root_directory);
    let default_rejects_dir =
        matches.value_of("out_directory").unwrap_or("rejects");
    let default_rejects_dir = Path::new(default_rejects_dir);
    let rejects = root_directory.join(default_rejects_dir);

    let ensure = matches.is_present("ensure");
    let verbose = matches.is_present("verbose");

    if !rejects.exists() {
        match create_dir(&rejects) {
            Ok(_) => {
                println!("Rejects directory [{}] created", rejects.display())
            }
            Err(e) => {
                eprintln!("Could not create directory for rejects: {}", e);
            }
        }
    }

    let ds = DeeDoo {
        hm: RefCell::new(HashMap::new()),
        reject_dir: &rejects,
        ensure,
        verbose,
    };

    WalkDir::new(&root_directory)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .filter(|e| !e.path().starts_with(&rejects))
        .for_each(|d| handle_file(&d, &ds));
}
