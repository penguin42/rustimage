use std::f64;

use image::*;

#[derive(Copy, Clone, Debug)]
pub struct Point {
  pub x: usize,
  pub y: usize
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
  Down, // x= 0, y= 1
  Up,   // x= 0, y=-1
  Left, // x=-1, y= 0
  Right,// x= 1, y= 0
}

pub type Line = (Point, Point);

impl Point {
  pub fn step(&mut self, d: Direction, i: &Image, amount: usize) -> bool {
    let &mut tomod;
    let &mut limit;
    let inc : i8;
    match d {
      Direction::Down => {
          tomod = &mut self.y;
          limit = i.get_size().y;
          inc   = 1
        },
      Direction::Up => {
          tomod = &mut self.y;
          limit = 0;
          inc   = -1
        },
      Direction::Left => {
          tomod = &mut self.x;
          limit = 0;
          inc   = -1
        },
      Direction::Right => {
          tomod = &mut self.x;
          limit = i.get_size().x;
          inc   = 1
        },
    }
    
    if (inc > 0 && (amount >= limit || *tomod >= limit-amount)) ||
       (inc < 0 && *tomod < amount) {
      return false;
    }
    if inc < 0 {
      *tomod = *tomod - amount;
    } else {
      *tomod = *tomod + amount;
    }

    true
  }

  pub fn distance(&self, other: &Point) -> f64 {
    let xdiff = self.x as f64 - other.x as f64;
    let ydiff = self.y as f64 - other.y as f64;

    (xdiff*xdiff + ydiff*ydiff).sqrt()
  }

  pub fn line_distance(&self, line: &(Point, Point)) -> f64 {
    let (linep1, linep2) = *line;
    let l1x = linep1.x as f64;
    let l1y = linep1.y as f64;
    let l2x = linep2.x as f64;
    let l2y = linep2.y as f64;
    // from https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line
    let numer = ((l2y - l1y) * (self.x as f64) - (l2x - l1x) * (self.y as f64) +
                    l2x * l1y - l2y * l1x).abs();
    let denom = ((l2y - l1y).powi(2) + (l2x - l1x).powi(2)).sqrt();
    numer/denom
  }
}

impl Direction {
  pub fn clockwise(self) -> Direction {
    match self {
      Direction::Down => Direction::Left,
      Direction::Left => Direction::Up,
      Direction::Up => Direction::Right,
      Direction::Right => Direction::Down,
    }
  }
  pub fn cntr_clockwise(self) -> Direction {
    match self {
      Direction::Down => Direction::Right,
      Direction::Right => Direction::Up,
      Direction::Up => Direction::Left,
      Direction::Left => Direction::Down,
    }
  }
} 
