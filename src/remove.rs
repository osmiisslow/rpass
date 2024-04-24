use std::fs::remove_file;

use crate::{
    append_to_path::append_to_path,
    rpass_dir::rpass_dir,
};

pub fn remove(application: String) {
    let filename = &application as &str;
    let filepath = append_to_path(rpass_dir(), filename);
    remove_file(filepath)
        .expect("cannot remove password as it doesnt exist!");
} 