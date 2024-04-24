use std::path::Path;
use std::fs;
use crate::rpass_dir::rpass_dir;


pub fn checkifran() {
    let exists: bool = Path::new(&rpass_dir()).exists();
    if !exists {
        fs::create_dir(rpass_dir()).unwrap();
    }
}