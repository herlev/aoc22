use itertools::*;
use std::collections::HashMap;

fn part1(input: &str) -> u32 {
  let mut fs = HashMap::<Vec<&str>, u32>::new();
  let mut path = vec![];
  for line in input.lines() {
    match line.split(" ").collect::<Vec<_>>()[..] {
      ["$", "ls"] => (),
      ["$", "cd", "/"] => path = vec![],
      ["$", "cd", ".."] => {
        path.pop().unwrap();
        ()
      }
      ["$", "cd", dir] => path.push(dir),
      ["dir", _] => (),
      [size, _] => {
        let mut tmp_path = path.clone();
        while tmp_path.len() > 0 {
          *fs.entry(tmp_path.clone()).or_default() += size.parse::<u32>().unwrap();
          tmp_path.pop().unwrap();
        }
      }
      _ => panic!(),
    }
  }
  fs.values().filter(|&&v| v <= 100000).sum()
}

fn part2(input: &str) -> u32 {
  let mut fs = HashMap::<Vec<&str>, u32>::new();
  let mut path = vec!["/"];
  for line in input.lines() {
    match line.split(" ").collect::<Vec<_>>()[..] {
      ["$", "ls"] => (),
      ["$", "cd", "/"] => path = vec!["/"],
      ["$", "cd", ".."] => {
        path.pop().unwrap();
        ()
      }
      ["$", "cd", dir] => path.push(dir),
      ["dir", _] => (),
      [size, _] => {
        let mut tmp_path = path.clone();
        while tmp_path.len() > 0 {
          *fs.entry(tmp_path.clone()).or_default() += size.parse::<u32>().unwrap();
          tmp_path.pop().unwrap();
        }
      }
      _ => panic!(),
    }
  }
  let total_disk_space = 70_000_000;
  let required_unused_space = 30_000_000;
  let current_unused_space = total_disk_space - *fs.entry(["/"].into()).or_default();
  *fs
    .values()
    .sorted()
    .find(|&&v| current_unused_space + v >= required_unused_space)
    .unwrap()
}

fn main() {
  let input = include_str!("../../inputs/07.txt");
  let tinput = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  println!("part2: {}", part2(tinput));
  println!("part2: {}", part2(input));
}
