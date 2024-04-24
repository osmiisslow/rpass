extern crate dirs;

use crate::append_to_path::append_to_path;
use std::path::PathBuf;

fn rpass_dir_windows() -> PathBuf {
    append_to_path(
        dirs::data_dir().unwrap(),
        r"\rpass\")
}

fn rpass_dir_linux() -> PathBuf {
    append_to_path(
        dirs::data_dir().unwrap(),
        r"/rpass/")
}

pub fn rpass_dir() -> PathBuf {
    if cfg!(windows) {
        rpass_dir_windows()
    } else if cfg!(unix) {
        rpass_dir_linux()
    } else {
        panic!("not using windows or unix")
    }
}