/**
 * name: File and Folder Traversing
 * author: iamajraj
 */

use std::{env::args, fs};

/**
 * Takes argument of folder name
 * 
 * e.g
 * cargo run -- {folder name}
 */
fn main(){
    if args().len() < 2 {
        panic!("Please provide folder name");
    }
    let folder = args().nth(1).unwrap();
    traverse_folder(&folder, 0);
}

/**
 * Traverse the files and folders
 */
fn traverse_folder(folder: &str, indent: usize){
    if let Ok(entries) = fs::read_dir(folder) {
        for entry in entries {
            if let Ok(dir) = entry {
               if dir.path().is_dir() {
                println!("{:}🗂️: {:}", "-".repeat(indent), dir.file_name().to_str().unwrap());
                traverse_folder(dir.path().to_str().unwrap(), indent+1);
               }else{
                println!("{:}📁 {:}", "-".repeat(indent), dir.file_name().to_str().unwrap());
               }
            }
        }
    }
}