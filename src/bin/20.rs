use itertools::*;

fn do_move(v: &mut Vec<(i64, usize)>, i: usize) {
  let n = v[i];
  if n.0 == 0 {
    return;
  }
  let old_index = n.1 as i64;
  let mut new_index = old_index + n.0;

  // when adding to an index it is not possible to get the highest index (v.len()-1)
  // when subtracting from an index it is not possible to get index 0
  if new_index >= v.len() as i64 - 1 {
    new_index = new_index.rem_euclid(v.len() as i64 - 1);
  }
  if new_index <= 0 {
    new_index = (new_index - 1).rem_euclid(v.len() as i64 - 1) + 1;
  }

  v[i].1 = new_index as usize;
  // Shift the index of all affected numbers by 1 or -1
  for (ii, a) in v.iter_mut().enumerate() {
    if ii == i {
      continue;
    }
    if old_index < new_index {
      if old_index <= a.1 as i64 && a.1 as i64 <= new_index {
        a.1 -= 1;
      }
    }
    if new_index < old_index {
      if new_index <= a.1 as i64 && a.1 as i64 <= old_index {
        a.1 += 1;
      }
    }
  }
}

fn print(v: &Vec<(i64, usize)>) {
  println!(
    "{}",
    v.iter().sorted_by_key(|v| v.1).map(|v| v.0.to_string()).join(", ")
  );
}

fn part1(input: &str) -> i64 {
  let mut v = input
    .lines()
    .map(|line| line.parse::<i64>().unwrap())
    .enumerate()
    .map(|(i, v)| (v, i))
    .collect::<Vec<_>>();
  // print(&v);
  for i in 0..v.len() {
    do_move(&mut v, i);
    // print!("Move {i}: ");
    // print(&v);
  }
  let i = v.iter().find(|v| v.0 == 0).unwrap().1;
  let v = v.iter().sorted_by_key(|v| v.1).map(|v| v.0).cycle();
  v.clone().nth(i + 1000).unwrap() + v.clone().nth(i + 2000).unwrap() + v.clone().nth(i + 3000).unwrap()
}

// this part could be made faster by modding/rem_euclid the numbers being used by the length of the vector
fn part2(input: &str) -> i64 {
  let mut v = input
    .lines()
    .map(|line| line.parse::<i64>().unwrap())
    .enumerate()
    .map(|(i, v)| (v * 811589153, i))
    .collect::<Vec<_>>();
  // print(&v);
  for _ in 0..10 {
    for i in 0..v.len() {
      do_move(&mut v, i);
      // print!("Move {i}: ");
      // print(&v);
    }
  }
  // print(&v);
  let i = v.iter().find(|v| v.0 == 0).unwrap().1;
  let v = v.iter().sorted_by_key(|v| v.1).map(|v| v.0).cycle();
  v.clone().nth(i + 1000).unwrap() + v.clone().nth(i + 2000).unwrap() + v.clone().nth(i + 3000).unwrap()
}

fn main() {
  let input = include_str!("../../inputs/20.txt");
  let tinput = "1
2
-3
3
-2
0
4";
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  println!("part2: {}", part2(tinput));
  println!("part2: {}", part2(input));
  // assert_eq!(part1(tinput), 0);
  // assert_eq!(part1(input), 0);
  // assert_eq!(part2(tinput), 0);
  // assert_eq!(part2(input), 0);
}
