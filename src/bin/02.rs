const SCORES: [i32; 3] = [0, 3, 6];

fn part1(input: &str) -> u32 {
  input
    .lines()
    .map(|line| {
      let oplay = line.chars().nth(0).unwrap() as i32 - 'A' as i32;
      let splay = line.chars().nth(2).unwrap() as i32 - 'X' as i32;
      (splay + 1 + SCORES[(splay - oplay + 1).rem_euclid(3) as usize]) as u32
    })
    .sum()
}

fn part2(input: &str) -> u32 {
  input
    .lines()
    .map(|line| {
      let oplay = line.chars().nth(0).unwrap() as i32 - 'A' as i32;
      let rec = line.chars().nth(2).unwrap() as i32 - 'X' as i32;
      (1 + (oplay + (rec - 1)).rem_euclid(3) + SCORES[rec as usize]) as u32
    })
    .sum()
}

fn main() {
  let tinput = "A Y
B X
C Z";
  let input = include_str!("../../inputs/02.txt");
  println!("part1: {}\n", part1(tinput));
  println!("part1: {}\n", part1(input));
  println!("part2: {}\n", part2(tinput));
  println!("part2: {}\n", part2(input));
}
