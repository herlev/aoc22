use std::fmt;

use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(Add, AddAssign, Sub, SubAssign, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Point2<T> {
  pub x: T,
  pub y: T,
}

pub enum Direction {
  Up,
  Down,
  Right,
  Left,
}

pub enum DirectionDiag {
  Upleft,
  Up,
  Upright,
  Right,
  DownRight,
  Down,
  Downleft,
  Left,
}

impl Direction {
  pub fn to_point(&self) -> Point2<i32> {
    match self {
      Self::Up => Point2::new(0, 1),
      Self::Down => Point2::new(0, -1),
      Self::Right => Point2::new(1, 0),
      Self::Left => Point2::new(-1, 0),
    }
  }
}

impl<T> Point2<T> {
  pub fn from(p: (T, T)) -> Self {
    Self { x: p.0, y: p.1 }
  }
  pub fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl Point2<i32> {
  pub fn neighbors(self) -> impl Iterator<Item = Point2<i32>> {
    [Direction::Up, Direction::Down, Direction::Right, Direction::Left]
      .into_iter()
      .map(move |d| self + d.to_point())
  }
  pub fn neighbors_grid(self, w: usize, h: usize) -> impl Iterator<Item = Point2<i32>> {
    self
      .neighbors()
      .filter(move |n| (0 as i32..w as i32).contains(&n.x) && (0 as i32..h as i32).contains(&n.y))
  }
}

impl<T: num::traits::Signed> Point2<T> {
  pub fn signum(&self) -> Point2<T> {
    Point2::new(self.x.signum(), self.y.signum())
  }
}

impl<T: std::fmt::Display> fmt::Display for Point2<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

// TODO
// struct Grid {}
