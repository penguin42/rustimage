use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops;
use std::fs::File;

use string::*;

// points in this image are indexed using (x: usize, y: usize) tuples
// (0,0) is top left
pub struct Image {
  width: usize,
  height: usize,
  data: Box<[u8]>,
}

#[derive(Debug)]
pub struct Point {
  pub x: usize,
  pub y: usize
}

pub enum Direction {
  Down, // x= 0, y= 1
  Up,   // x= 0, y=-1
  Left, // x=-1, y= 0
  Right,// x= 1, y= 0
}

#[derive(Debug)]
pub enum ImageErr {
  IO(io::Error),
  NumErr,
  BadHeader(String),
  WrongSubtype(String),
}

impl From<io::Error> for ImageErr {
  fn from(err: io::Error) -> ImageErr {
    ImageErr::IO(err)
  }
}

/// Read the pnm header off a file
/// Returns the type and then the (width, height)
pub fn read_pnm_header(f: &mut BufRead) -> Result<(usize, (usize, usize)), ImageErr> {
  // Should start with 'P'
  let mut tmp_byte : [u8; 1] = [0; 1];
  try!(f.read_exact(&mut tmp_byte));

  if tmp_byte[0] != 'P' as u8 {
    return Err(ImageErr::BadHeader(String::from("Missing PGM header P")));
  }

  // Next is a numeric character identifying the pnm subtype
  try!(f.read_exact(&mut tmp_byte));
  if !(tmp_byte[0] as char).is_numeric() {
    return Err(ImageErr::BadHeader(String::from("Bad PGM header type")));
  }
  let subtype = tmp_byte[0] - '0' as u8;

  // Note that PAM format files are different from the other pnm and identify
  // the width/height fields with text not just raw figures, but I'm not dealing
  // with that now

  // White space and the width
  try!(skip_whitespace(f));
  let width = try!(read_integer(f));

  // White space and then height
  try!(skip_whitespace(f));
  let height = try!(read_integer(f));
  
  // White space and then max grey (We only support 255)
  try!(skip_whitespace(f));
  let max_grey = try!(read_integer(f));
  if max_grey != 255 {
    return Err(ImageErr::BadHeader(String::from("Unsupported grey depth")));
  }
  // *1* white space and then data - note the read_integer will have
  // consumed that white space

  Ok((subtype as usize, (width, height)))
}

pub fn load_pgm(file_name: String) -> Result<Image, ImageErr> {
  let mut r = BufReader::new(try!(File::open(file_name)));

  let (pnm_type, (my_width, my_height)) = try!(read_pnm_header(&mut r));

  match pnm_type {
    2 => return Err(ImageErr::WrongSubtype(String::from("Plain PGM not supported"))),
    5 => (), // that's normal binary PGM that we support
    _ => return Err(ImageErr::WrongSubtype(format!("PNM type {} is not a PGM and is not supported", pnm_type))),
  }

  if my_width == 0 || my_height == 0 {
    return Err(ImageErr::BadHeader(String::from("Width/height can't be 0")));
  }

  let mut my_data = vec![0 as u8; my_width * my_height];
  try!(r.read_exact(&mut my_data));
  let result : Image = Image { width: my_width, height: my_height, data: my_data.into_boxed_slice() };

  println!("Got pgm type {} {}x{}", pnm_type, my_width, my_height);

  Ok(result)
}

impl Image {
  pub fn save_pgm(&self, file_name: String) -> Result<(), ImageErr> {
    let mut f = try!(File::create(file_name));
    try!(write!(f, "P5\n{} {}\n255\n", self.width, self.height));
    try!(f.write_all(&self.data));
    Ok(())
  }
}

impl ops::Index<Point> for Image {
  type Output = u8;

  fn index(&self, p: Point) -> &u8 {
    &self.data[p.x + p.y * self.width]
  }
}

impl ops::IndexMut<Point> for Image {
  fn index_mut(&mut self, p: Point) -> &mut u8 {
    &mut self.data[p.x + p.y * self.width]
  }
}

impl Point {
  pub fn step(&mut self, d: Direction, i: &Image) -> bool {
    let &mut tomod;
    let &mut limit;
    let inc : i8;
    match d {
      Direction::Down => {
          tomod = &mut self.y;
          limit = i.height;
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
          limit = i.width;
          inc   = 1
        },
    }
    
    if (inc > 0 && *tomod >= limit) ||
       (inc < 0 && *tomod == 0) {
      return false;
    }
    if inc < 0 {
      *tomod = *tomod - 1;
    } else {
      *tomod = *tomod + 1;
    }

    true
  }
}
