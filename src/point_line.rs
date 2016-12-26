use std::f64;

use image::*;

#[derive(Copy, Clone, Debug)]
pub struct Point {
  pub x: usize,
  pub y: usize
}

#[derive(Copy, Clone, Debug)]
pub struct Pointf {
  pub x: f64,
  pub y: f64
}

impl From<Point> for Pointf {
  fn from(p : Point) -> Pointf {
    Pointf { x: p.x as f64, y: p.y as f64 }
  }
}

impl From<Pointf> for Point {
  fn from(p : Pointf) -> Point {
    Point { x: p.x as usize, y: p.y as usize }
  }
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

pub fn line_intersection(l1 : &Line, l2 : &Line) -> Point {
  let (l1p1, l1p2) = *l1;
  let (l2p1, l2p2) = *l2;
  let l1p1x = l1p1.x as f64;
  let l1p1y = l1p1.y as f64;
  let l1p2x = l1p2.x as f64;
  let l1p2y = l1p2.y as f64;
  let l2p1x = l2p1.x as f64;
  let l2p1y = l2p1.y as f64;
  let l2p2x = l2p2.x as f64;
  let l2p2y = l2p2.y as f64;

  // From https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line
  let xnumer = (l1p1x * l1p2y - l1p1y * l1p2x) * (l2p1x - l2p2x) -
               (l1p1x - l1p2x) * (l2p1x * l2p2y - l2p1y * l2p2x);
  let denom = (l1p1x - l1p2x) * (l2p1y - l2p2y) -
              (l1p1y - l1p2y) * (l2p1x - l2p2x);

  let ynumer = (l1p1x * l1p2y - l1p1y * l1p2x) * (l2p1y - l2p2y) -
               (l1p1y - l1p2y) * (l2p1x * l2p2y - l2p1y * l2p2x);

  let x = xnumer / denom;
  let y = ynumer / denom;

  // TODO: We should return this as an f64 point
  Point { x: x as usize, y: y as usize }
}
