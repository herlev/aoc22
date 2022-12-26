pub mod direction;
pub mod grid2;
pub mod grid3;
pub mod point2;
pub mod point3;
pub mod priority_queue;

pub use direction::*;
pub use grid2::*;
pub use grid3::*;
pub use point2::*;
pub use point3::*;
pub use priority_queue::*;

fn get_nums(s: &str) -> Vec<i32> {
  let mut i = s.chars();
  let mut nums = Vec::new();

  loop {
    let s: String = i
      .by_ref()
      .skip_while(|c| !c.is_ascii_digit())
      .take_while(|c| c.is_ascii_digit())
      .collect();
    if s.len() > 0 {
      nums.push(s.parse::<i32>().unwrap());
    } else {
      break;
    }
  }
  nums
}
