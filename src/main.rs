#![allow(unused_imports)]

use std::env;
use std::io;
use std::option;
use std::path::{PathBuf, Path};
extern crate dirs;

//Get home dir path
fn get_home() -> Result<PathBuf, &'static str> {
    let home = dirs::home_dir();
    match home {
        Some(x) => Ok(x),
        None => Err("No home directory"),
    }
}
//Get current dir path
fn get_dots() -> Result<PathBuf, io::Error> {
    let dot = env::current_dir();
    match dot {
        Ok(x) => Ok(x),
        Err(x) => Err(x),
    }
}

fn symlink(dots_path: PathBuf) -> PathBuf {
    let home_path = get_home().unwrap();
    let mut file_path = dots_path.strip_prefix(home_path).unwrap().to_path_buf();
    file_path
}

fn main() {
    println!("{:?}", get_home().unwrap());
    println!("{:?}", get_dots().unwrap());
    let test = Path::new("/home/regul4r/code/symdot/src").to_path_buf();
    println!("{:?}", symlink(test)); 
}
