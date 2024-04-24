use std::fs;

use crate::rpass_dir::rpass_dir;

pub fn list() {
    println!("Passwords exist for the following applications:");
    match fs::read_dir(rpass_dir()) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        match entry.file_name().to_str() {
                            Some(file_name) => println!("{}", file_name),
                            None => panic!("unable to get file name"),
                        }
                    }
                    Err(_) => panic!("error reading entry"),
                }
            }
        }
        Err(_) => panic!("error reading directory"),
    }
}