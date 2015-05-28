use std::io;
use std::fs::{self, DirEntry};
use std::env;
use std::path::Path;

extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;


fn md5_of_file(filepath: &Path) -> bool {
    let mut hasher = Md5::new();
    hasher.input_str("");
    let out = hasher.result_str();
    println!("hash of {} -> {}", filepath.to_str().unwrap(), out);
    return true;
}

fn visit_dirs(dir: &Path) -> io::Result<()> {
    for entry in try!(fs::read_dir(dir)) {
        let entry: fs::DirEntry = try!(entry);
        println!("{:?}", entry.path());
        let metadata = try!(fs::metadata(&entry.path()));
        if metadata.is_dir() {
            try!(visit_dirs(&entry.path()));
        } else {
            md5_of_file(&entry.path());
        }
    }
    Ok(())
}

fn main() {
    // list files under `./` if no param, else list files under param 1
    let path: String = match env::args().nth(1) {
        None => ".".to_string(),
        Some(arg) => arg,
    };
    let path: &Path = Path::new(&path);

    println!("{:?}", path);
    // check if path is a dir
    let metadata = match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                let _ = visit_dirs(path);
            }
        }
        Err(_) => {
            println!("{} is not a Valid path!", path.display());
            return
        }
    };
}
