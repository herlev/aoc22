use crate::Point2;

#[derive(Clone)]
pub enum Direction {
  Up,
  Right,
  Down,
  Left,
}

pub enum DirectionDiag {
  Upleft,
  Up,
  Upright,
  Right,
  Downright,
  Down,
  Downleft,
  Left,
}

impl DirectionDiag {
  pub fn to_point(&self) -> Point2<i32> {
    match self {
      Self::Up => Point2::new(0, 1),
      Self::Down => Point2::new(0, -1),
      Self::Right => Point2::new(1, 0),
      Self::Left => Point2::new(-1, 0),
      Self::Upleft => Point2::new(-1, 1),
      Self::Upright => Point2::new(1, 1),
      Self::Downleft => Point2::new(-1, -1),
      Self::Downright => Point2::new(1, -1),
    }
  }
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
