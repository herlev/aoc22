use aoc22::{Direction, Point2, PriorityQueue};
use itertools::*;
use std::collections::HashSet;

type Point = Point2<i32>;

#[derive(Clone)]
struct CircularVec<T> {
  data: Vec<T>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct State {
  pos: Point,
  time: u32,
  going_to_idx: usize,
}

type Mover = ();

struct Movers {
  data: Vec<CircularVec<Option<Mover>>>,
  move_dir: Direction,
}

impl<T: Clone + Default> CircularVec<T> {
  fn new(size: usize) -> Self {
    Self {
      data: vec![T::default(); size],
    }
  }
  fn at_mut(&mut self, i: usize, offset: i32) -> &mut T {
    if i >= self.data.len() {
      panic!()
    }
    let idx = (offset + i as i32).rem_euclid(self.data.len() as i32);
    &mut self.data[idx as usize]
  }
  fn at(&self, i: usize, offset: i32) -> &T {
    if i >= self.data.len() {
      panic!()
    }
    let idx = (offset + i as i32).rem_euclid(self.data.len() as i32);
    &self.data[idx as usize]
  }
  fn len(&self) -> usize {
    self.data.len()
  }
}

impl Movers {
  fn new(width: usize, height: usize, dir: Direction) -> Self {
    let (inner_size, outer_size) = match dir {
      Direction::Left | Direction::Right => (width, height),
      Direction::Up | Direction::Down => (height, width),
    };
    Self {
      data: vec![CircularVec::new(inner_size); outer_size],
      move_dir: dir,
    }
  }
  fn width(&self) -> usize {
    match self.move_dir {
      Direction::Left | Direction::Right => self.data[0].len(),
      Direction::Up | Direction::Down => self.data.len(),
    }
  }
  fn height(&self) -> usize {
    match self.move_dir {
      Direction::Left | Direction::Right => self.data.len(),
      Direction::Up | Direction::Down => self.data[0].len(),
    }
  }
  fn at_mut(&mut self, p: Point, time: u32) -> Option<&mut Option<Mover>> {
    if !((0i32..self.width() as i32).contains(&p.x) && (0i32..self.height() as i32).contains(&p.y)) {
      return None;
    }
    let offset = match self.move_dir {
      Direction::Right | Direction::Up => time as i32,
      Direction::Left | Direction::Down => -(time as i32),
    };
    match self.move_dir {
      Direction::Left | Direction::Right => Some(self.data[p.y as usize].at_mut(p.x as usize, offset)),
      Direction::Up | Direction::Down => Some(self.data[p.x as usize].at_mut(p.y as usize, offset)),
    }
  }
  fn at(&self, p: Point, time: u32) -> Option<&Option<Mover>> {
    if !((0i32..self.width() as i32).contains(&p.x) && (0i32..self.height() as i32).contains(&p.y)) {
      return None;
    }
    let offset = match self.move_dir {
      Direction::Right | Direction::Up => -(time as i32),
      Direction::Left | Direction::Down => time as i32,
    };
    match self.move_dir {
      Direction::Left | Direction::Right => Some(self.data[p.y as usize].at(p.x as usize, offset)),
      Direction::Up | Direction::Down => Some(self.data[p.x as usize].at(p.y as usize, offset)),
    }
  }
}

fn parse(input: &str) -> (usize, usize, [Movers; 4]) {
  let i = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
  let width = i[0].len() - 2;
  let height = i.len() - 2;
  let mut up_movers = Movers::new(width, height, Direction::Up);
  let mut down_movers = Movers::new(width, height, Direction::Down);
  let mut left_movers = Movers::new(width, height, Direction::Left);
  let mut right_movers = Movers::new(width, height, Direction::Right);
  for (x, y) in iproduct!(0..width, 0..height) {
    let p = Point::new(x as i32, height as i32 - 1 - y as i32);
    match i[y + 1][x + 1] {
      '>' => *right_movers.at_mut(p, 0).unwrap() = Some(()),
      '<' => *left_movers.at_mut(p, 0).unwrap() = Some(()),
      'v' => *down_movers.at_mut(p, 0).unwrap() = Some(()),
      '^' => *up_movers.at_mut(p, 0).unwrap() = Some(()),
      '.' => (),
      _ => panic!(),
    }
  }
  (width, height, [up_movers, right_movers, down_movers, left_movers])
}

fn manhattan_dist(a: &Point, b: &Point) -> usize {
  let d = (*a - *b).abs();
  (d.x + d.y) as usize
}

fn print_state(s: &State, movers: &[Movers; 4]) {
  let (w, h) = (movers[0].width(), movers[0].height());
  let mut g = vec![vec![vec![]; w]; h];
  for (x, y) in iproduct!(0..w, 0..h) {
    let p = Point::new(x as i32, y as i32);
    for mover in movers {
      if matches!(mover.at(p, s.time), Some(Some(_))) {
        g[y][x].push(&mover.move_dir);
      }
    }
  }
  let s = g
    .iter()
    .map(|row| {
      row
        .iter()
        .map(|v| match v.len() {
          0 => '.',
          1 => match v[0] {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
          },
          n => n.to_string().chars().next().unwrap(),
        })
        .collect::<String>()
    })
    .rev()
    .join("\n");
  println!("{s}\n");
}

fn solve(movers: [Movers; 4], route: Vec<Point>) -> u32 {
  let mut pq = PriorityQueue::new();
  let mut visited = HashSet::new();
  let minimum_remaining_dist = |state: &State| {
    let s = Some(state.pos);
    let i = s.iter().chain(route.iter().skip(state.going_to_idx));
    i.tuple_windows().map(|(a, b)| manhattan_dist(a, b)).sum::<usize>()
  };
  let is_free = |pos, time| route.iter().contains(&pos) || movers.iter().all(|m| matches!(m.at(pos, time), Some(None)));
  let priority = |state: &State| state.time as usize + minimum_remaining_dist(state);
  let pqe = |state: State| (priority(&state), state);
  let initial_state = State {
    pos: route[0],
    time: 0,
    going_to_idx: 1,
  };
  pq.push(pqe(initial_state));
  visited.insert(initial_state);

  // Perform A* search
  while let Some(State {
    pos,
    time,
    going_to_idx,
  }) = pq.pop()
  {
    for p in pos.neighbors().chain(Some(pos)) {
      let s = State {
        pos: p,
        time: time + 1,
        going_to_idx: going_to_idx + if p == route[going_to_idx] { 1 } else { 0 },
      };
      if is_free(p, time + 1) && !visited.contains(&s) {
        if going_to_idx == route.len() - 1 && p == route[route.len() - 1] {
          return time + 1;
        }
        pq.push(pqe(s));
        visited.insert(s);
      }
    }
  }
  panic!()
}

fn part1(input: &str) -> u32 {
  let (w, h, movers) = parse(input);
  let start_point = Point::new(0, h as i32);
  let end_point = Point::new(w as i32 - 1, -1);
  solve(movers, vec![start_point, end_point])
}

fn part2(input: &str) -> u32 {
  let (w, h, movers) = parse(input);
  let start_point = Point::new(0, h as i32);
  let end_point = Point::new(w as i32 - 1, -1);
  solve(movers, vec![start_point, end_point, start_point, end_point])
}

fn main() {
  let input = include_str!("../../inputs/24.txt");
  let tinput = "#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#";
  let tinput2 = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
  println!("part1: {}", part1(tinput2));
  println!("part1: {}", part1(input));
  println!("part2: {}", part2(tinput2));
  println!("part2: {}", part2(input));
  assert_eq!(part1(tinput2), 18);
  assert_eq!(part1(input), 281);
  assert_eq!(part2(tinput2), 54);
  assert_eq!(part2(input), 807);
}
