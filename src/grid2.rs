use std::ops::RangeInclusive;

use itertools::iproduct;

use crate::Point2;

#[derive(Debug)]
pub struct Grid2<T> {
  data: Vec<Vec<T>>,
  pub x_range: RangeInclusive<i32>,
  pub y_range: RangeInclusive<i32>,
}

impl<T: Default + Clone> Grid2<T> {
  pub fn new(x_range: RangeInclusive<i32>, y_range: RangeInclusive<i32>) -> Self {
    let xd = (x_range.end() - x_range.start()) as usize;
    let yd = (y_range.end() - y_range.start()) as usize;
    let g = vec![vec![T::default(); yd + 1]; xd + 1];
    Self {
      data: g,
      x_range,
      y_range,
    }
  }

  pub fn iter(&self) -> impl Iterator<Item = Point2<i32>> {
    iproduct!(self.x_range.clone(), self.y_range.clone()).map(Point2::from)
  }

  pub fn at_point(&mut self, p: &Point2<i32>) -> Option<&mut T> {
    self.at_mut(&p.x, &p.y)
  }

  pub fn at(&self, x: &i32, y: &i32) -> Option<&T> {
    if self.x_range.contains(&x) && self.y_range.contains(&y) {
      let xn = (x - self.x_range.start()) as usize;
      let yn = (y - self.y_range.start()) as usize;
      return Some(&self.data[xn][yn]);
    }
    None
  }

  pub fn at_mut(&mut self, x: &i32, y: &i32) -> Option<&mut T> {
    if self.x_range.contains(&x) && self.y_range.contains(&y) {
      let xn = (x - self.x_range.start()) as usize;
      let yn = (y - self.y_range.start()) as usize;
      return Some(&mut self.data[xn][yn]);
    }
    None
  }
}
