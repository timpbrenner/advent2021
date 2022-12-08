use crate::util::open_file;
use std::collections::{HashMap, VecDeque};
use std::iter::Peekable;
use std::str::Lines;

use itertools::Itertools;

pub fn traverse() {
  let contents = open_file("data/day7.txt");
  let mut cwd: VecDeque<String> = VecDeque::new();
  let mut lines = contents.lines().peekable();
  let mut fs: HashMap<String, Vec<String>> = HashMap::new();

  while let Some(line) = lines.next() {
    if line.starts_with("$ ls") {
      process_ls(&mut lines, &mut cwd, &mut fs);
    } else if line.starts_with("$ cd") {
      process_cd(&mut cwd, line);
    }
  }

  let mut sizes = HashMap::new();
  get_directory_sizes(&fs, &mut sizes, "/");

  let sum_total = sizes.values().filter(|s| **s <= 100_000).sum::<i32>();
  println!("Total: {}", sum_total);

  let needed_space: i32 = 30_000_000;
  let free_space: i32 = 70_000_000 - sizes.get("/").unwrap();
  let need_to_free: i32 = needed_space - free_space;

  let size_to_delete = sizes.values().sorted().find(|s| **s >= need_to_free).unwrap();
  println!("Delete: {}", size_to_delete);
}

fn process_cd(cwd: &mut VecDeque<String>, line: &str) {
  if line == "$ cd .." {
    cwd.pop_back();
  } else if line == "$ cd /" {
    cwd.clear();
  } else {
    let mut parts = line.split(" ").skip(2);
    cwd.push_back(parts.next().unwrap().to_string());
  }
}

fn process_ls(input: &mut Peekable<Lines>, cwd: &VecDeque<String>, fs: &mut HashMap<String, Vec<String>>) {
  let base_path = if cwd.len() == 0 {
    String::from("/")
  } else {
      format!("/{}/", cwd.iter().join("/"))
  };

  let mut contents: Vec<String> = Vec::new();
  loop {
    contents.push(input.next().unwrap().to_string()); 
    let ahead = input.peek();
    if ahead.is_none() || ahead.unwrap().starts_with("$ ") {
        break;
    }
  }

  fs.insert(base_path, contents);
}

fn get_directory_sizes(
  fs: &HashMap<String, Vec<String>>,
  sizes: &mut HashMap<String, i32>,
  starting: &str,
) -> i32 {
  let mut total_size = 0;
  if let Some(contents) = fs.get(starting) {
    for content in contents {
      if content.starts_with("dir") {
        let name = content.split(' ').skip(1).next().unwrap();
        let dir_name = &(starting.to_string() + name + "/");
        total_size += get_directory_sizes(fs, sizes, dir_name);
      } else {
        let size = content.split(' ').next().unwrap().parse::<i32>().unwrap();
        total_size += size;
      }
    }
  }

  sizes.insert(starting.to_string(), total_size);
  total_size
}