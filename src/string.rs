use std::io;
use std::io::prelude::*;

use image::ImageErr;

/// (c) David Alan Gilbert <dave@treblig.org> 2016

/// Skip whitespace
pub fn skip_whitespace(f: &mut BufRead) -> Result<(), io::Error> {
  let mut in_comment = false;

  // TODO: This really needs to be using the iterator - fill_buf isn't safe with interruptibles etc
  'search: loop {
    {
      let buf = try!(f.fill_buf());
      let cur = buf[0] as char;
      if cur=='#' {
        in_comment = true;
      }
      if !in_comment && !(cur.is_whitespace()) {
        break 'search;
      }
      if in_comment && cur=='\n' {
        in_comment = false;
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
  let mut result : usize = 0;
  let mut have_digit : bool = false;

  for r in f.bytes() {
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

  if have_digit {
    Ok(result)
  } else {
    Err(ImageErr::NumErr)
  }
}
