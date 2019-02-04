#![allow(unused_imports)]

use std::env;
use std::fs::remove_file;
use std::io;
use std::option;
use std::os::unix::fs;
use std::path::{Path, PathBuf};
extern crate dirs;

// //Get home dir path
// fn get_home() -> Result<PathBuf, &'static str> {
//     let home = dirs::home_dir();
//     match home {
//         Some(x) => Ok(x),
//         None => Err("No home directory"),
//     }
// }
// //Get current dir path
// fn get_dots() -> Result<PathBuf, io::Error> {
//     let dot = env::current_dir();
//     match dot {
//         Ok(x) => Ok(x),
//         Err(x) => Err(x),
//     }
// }
//Get the symlink file path
fn get_symlink_path(home_path: PathBuf, dot_path: PathBuf, file_path: PathBuf) -> PathBuf {
    //let home_path = get_home().unwrap();
    let dot_name = PathBuf::from(dot_path.file_name().unwrap());
    let sym_file = file_path
        .strip_prefix(&home_path)
        .unwrap()
        .strip_prefix(dot_name)
        .unwrap();
    home_path.join(sym_file)
}

//Create the symlink
fn create_symlink(
    home_path: PathBuf,
    dot_path: PathBuf,
    file_path: PathBuf,
) -> Result<String, io::Error> {
    //TODO better string returns
    let symlink_path = get_symlink_path(home_path, dot_path, file_path.to_path_buf());
    if !symlink_path.exists() {
        fs::symlink(file_path.to_path_buf(), symlink_path.to_path_buf())?;
    } else {
        let symlink_file_type = symlink_path.metadata().unwrap().file_type();
        if symlink_file_type.is_symlink() {
            if symlink_path.read_link().unwrap() != file_path {
                remove_file(symlink_path.to_path_buf())?;
                fs::symlink(file_path.to_path_buf(), symlink_path.to_path_buf())?;
            }
        } else if symlink_file_type.is_file() {
            println!("Regular File: {}", symlink_path.to_str().unwrap());
        }
    }
    Ok(symlink_path.to_path_buf().to_str().unwrap().to_string())
}

fn iterator(home_path: PathBuf, dot_path: PathBuf, name: Option<String>) -> std::io::Result<()> {
    if let Some(name) = name {
        for entry in dot_path.read_dir()? {
            let path = entry?.path();
            let is_match = matches(path.to_str().unwrap(), name.as_ref());
            if is_match {
                if path.is_dir() {
                    iterator(home_path.to_path_buf(), path.to_path_buf(), None)?;
                } else if path.is_file() {
                    create_symlink(home_path.to_path_buf(), dot_path.to_path_buf(), path.to_path_buf())?;
                }
            }
        }
    } else {
        for entry in dot_path.read_dir()? {
            let path = entry?.path();
            if path.is_dir() {
                iterator(home_path.to_path_buf(), path.to_path_buf(), None)?;
            } else if path.is_file() {
                create_symlink(home_path.to_path_buf(), dot_path.to_path_buf() , path.to_path_buf())?;
            }
        }
    }
    Ok(())
}

fn matches(filename: &str, input: &str) -> bool {
    input == filename
}

fn main() -> Result<(), io::Error> {
    let home_path = dirs::home_dir().expect("Na home directory found");
    //let dotfiles_dir_path = env::current_dir().expect("Error obtaining current directory");
    let dotfiles_dir_path = PathBuf::from("~/dots");
    println!(
        "{:?}",
        get_symlink_path(
            home_path,
            dotfiles_dir_path,
            PathBuf::from("/home/regull4r/dots/superlinker.py")
        )
    );
    // let test = Path::new("/home/regul4r/code/symdot/src").to_path_buf();
    // println!("{:?}", create_symlink(test));
    matches("o", "oa");
    Ok(())
}
