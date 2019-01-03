#![allow(unused_imports)]

use std::env;
use std::fs::remove_file;
use std::io;
use std::option;
use std::os::unix::fs;
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
fn create_symlink(git_path: PathBuf) -> Result<String, io::Error> { 
    //TODO better string returns
    let symlink_path = get_symlink_path(git_path.to_path_buf());
    if !symlink_path.exists() {
        fs::symlink(git_path, symlink_path.to_path_buf())?;
    } else {
        let symlink_file_type = symlink_path.metadata().unwrap().file_type();
        if symlink_file_type.is_symlink() {
            if symlink_path.read_link().unwrap() != git_path {
                remove_file(symlink_path.to_path_buf())?;
                fs::symlink(git_path, symlink_path.to_path_buf())?;
            }
        } else if symlink_file_type.is_file() {
            println!("Regular File: {}", symlink_path.to_str().unwrap());
        }
    }
    Ok(symlink_path.to_path_buf().to_str().unwrap().to_string())
}

fn iterator(path : &PathBuf) -> std::io::Result<()>{
    // for entry in &path.read_dir()?{
    //     let file_type = entry?.file_type()?;
    //     if file_type.is_dir(){
    //         iterator(entry?.path()); 
    //     } else if file_type.is_file(){
    //         create_symlink(entry?.path());
    //     }
    // }
    path.read_dir()?.for_each(|x|{
        if x.file_type()?.is_file(){
            
        }
    })
    //Ok(())
}


fn main() {
    println!("{:?}", get_home().unwrap());
    println!("{:?}", get_dots().unwrap());
    let test = Path::new("/home/regul4r/code/symdot/src").to_path_buf();
    println!("{:?}", create_symlink(test));
}
