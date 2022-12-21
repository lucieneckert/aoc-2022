// Tried using an actual tree earlier and rust made me want to kill myself
// I at least survived during this implementation attempt

use std::{collections::HashMap, i32};

type DirectoryId = String;

#[derive(Debug)]
struct FileSystem {
  directories: HashMap<DirectoryId, Directory> 
}

impl FileSystem {
  // get mutable reference to a directory with provided id
  fn get_dir(&mut self, id: &DirectoryId) -> &mut Directory {
    return match self.directories.get_mut(id) {
      Some(dir) => dir,
      _ => panic!("Directory with id {:?} not found,", id)
    };
  }
  fn add_dir(&mut self, id: DirectoryId) {
    self.directories.insert(id, Directory { local_size: 0, children: vec![] });
  }
  fn get_dir_size(&self, id: &DirectoryId) -> i32 {
    let dir = self.directories.get(id).unwrap();
    return 
      dir.children
        .iter()
        .fold(dir.local_size, |acc, dir| 
          return acc + self.get_dir_size(dir));
  }
}

#[derive(Debug)]
struct Directory {
  local_size: i32, // sum of all file sizes
  children: Vec<DirectoryId>, // ids of child directories
}

fn parse_filesystem(input: Vec<String>) -> FileSystem {
  let mut fs = FileSystem { directories: HashMap::new() }; 
  // initialize the root directory
  let root_id = String::from("/");
  fs.add_dir(root_id.clone());
  // keep track of our path
  let mut path : Vec<DirectoryId> = vec![root_id.clone()];
  // and if we want to add a new dir after mutably borrowing current dir
  let mut add_dir : Option<String> = None;
  for line in input {
    println!("Current Path: {:?}", &path);
    let mut current_dir : &mut Directory = fs.get_dir(path.last_mut().unwrap());
    println!("- Current Dir: {:?}", current_dir);
    println!("- Parsing: {:?}", line);
    let mut tokens = line.split(" ");
    match tokens.next().unwrap() {
      "$" => { // parse command
        match tokens.next().unwrap() {
          "cd" => { // change to new directory
            match tokens.next().unwrap() {
                "/" => {path.clear(); path.push(root_id.clone())} // return to root
                ".." => {path.pop();} // go up one
                dir_id => {path.push(dir_id.to_string())} // move to dir_id
            }
          },
          _ => {} // ignore ls and everything else
        }
      },
      "dir" => { // add dir to current dir
        // note that we need to add dir to filesystem
        let new_dir_id = tokens.next().unwrap().to_string();
        add_dir = Some(new_dir_id.clone());
        // track dir as child of current dir
        current_dir.children.push(new_dir_id.clone());
      },
      file_size => { // add filesize to current dir
        current_dir.local_size += file_size.parse::<i32>().unwrap();
      }
    }
    // see if we need to add any directories
    match add_dir {
      Some(dir_id) => {
        fs.add_dir(dir_id);
        add_dir = None;
      },
      None => (),
    }
  }
  return fs;
}


const MAX_SIZE : i32 = 100_000;

pub fn solve(input: Vec<String>) -> String {
  // parse a filesystem from the input
  let fs = parse_filesystem(input);
  // what even was the rest of the puzzle
  // oh right check all the dirs and see which ones satisfy condition
  let puzzle_1_solution : i32 = fs.directories
    .keys()
    .map(|dir| fs.get_dir_size(dir))
    .filter(|size| *size <= MAX_SIZE)
    .sum();
  return format!("1: {:?}", puzzle_1_solution);
}