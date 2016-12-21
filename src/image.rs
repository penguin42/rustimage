use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

pub struct Image {
}

#[derive(Debug)]
pub enum ImageErr {
  IO(io::Error),
  NumErr,
  BadHeader(String),
}

impl From<io::Error> for ImageErr {
  fn from(err: io::Error) -> ImageErr {
    ImageErr::IO(err)
  }
}

// TODO: Split these stirng parsers out into a separate file
/// Skip whitespace
pub fn skip_whitespace(f: &mut BufRead) -> Result<(), io::Error> {
  'search: loop {
    {
      let buf = try!(f.fill_buf());
      if !((buf[0] as char).is_whitespace()) {
        break 'search;
      }
    }
    f.consume(1);
  }
 
  Ok(())
}

/// Reads an integer from the stream
/// Note: Consumes the following white space
pub fn read_integer(f: &mut BufRead) -> Result<usize, ImageErr> {
  // TODO: Use peekable to avoid consuming the terminating ws
  let mut it = f.bytes();
  let mut result : usize = 0;
  let mut have_digit : bool = false;

  loop {
    match it.next() {
      Some(r) => {
        let b = try!(r);
        if (b as char).is_digit(10) {
          have_digit = true;
          result = result * 10 + (b - ('0' as u8)) as usize;
        } else {
          if !(b as char).is_whitespace() {
            return Err(ImageErr::NumErr);
          }
          break;
        }
      }
      None => { break }
    }
  }

  if have_digit {
    Ok(result)
  } else {
    Err(ImageErr::NumErr)
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
  let mut tmp_byte : [u8; 1] = [0; 1];
  let mut r = BufReader::new(try!(File::open(file_name)));

  let (pnm_type, (width, height)) = try!(read_pnm_header(&mut r));

  // Dummy read - more later
  try!(r.read_exact(&mut tmp_byte));
  println!("Got pgm type {} {}x{}", pnm_type, width, height);
  Ok(Image {})
}

