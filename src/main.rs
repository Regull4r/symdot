#![allow(unused_imports)]

use std::env;
use std::io;
use std::option;
use std::path::{Path, PathBuf};
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
//Get the symlink file path
fn get_symlink_path(dots_path: PathBuf) -> PathBuf {
    let home_path = get_home().unwrap();
    let dot_name = PathBuf::from(get_dots().unwrap().file_name().unwrap());
    let sym_file = dots_path
        .strip_prefix(&home_path)
        .unwrap()
        .strip_prefix(dot_name)
        .unwrap();
    sym_file.to_path_buf()
}

//Create the symlink
fn symlink(git_path: PathBuf){
    let symlink_path = get_symlink_path(git_path);
    //algo si no existe
    let symlink_path_info = symlink_path.metadata().unwrap().file_type();
    if 
    
}
fn main() {
    println!("{:?}", get_home().unwrap());
    println!("{:?}", get_dots().unwrap());
    let test = Path::new("/home/regul4r/code/symdot/src").to_path_buf();
    println!("{:?}", symlink(test));
}
