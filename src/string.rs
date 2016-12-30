/// (c) David Alan Gilbert <dave@treblig.org> 2016
/// Licensed under GPLv3, see the LICENSE file for a full copy

use std::io;
use std::iter;

use image::ImageErr;


/// Skip whitespace
pub fn skip_whitespace(it: &mut iter::Peekable<io::Bytes<&mut io::BufRead>>) -> Result<(), io::Error> {
  let mut in_comment = false;

  loop {
    {
      let cur = it.peek();
      match cur {
        None => { break ; },
        Some(&Ok(ch)) => {
          let ch = ch as char;
          if ch=='#' {
            in_comment = true;
          }
          if !in_comment && !(ch.is_whitespace()) {
            break;
          }
          if in_comment && ch=='\n' {
            in_comment = false;
          }
        },
        // On io::error, drop through and the it.next() try!
        // will propagate the err
        Some(&Err(_)) => { },
      }
    }
    try!(it.next().expect("skip_whitespace consume"));
  }
 
  Ok(())
}

/// Reads an integer from the stream
pub fn read_integer(it: &mut iter::Peekable<io::Bytes<&mut io::BufRead>>) -> Result<usize, ImageErr> {
  let mut result : usize = 0;
  let mut have_digit : bool = false;

  loop {
    {
      let cur = it.peek();
      match cur {
        None => { break ; },
        Some(&Ok(b)) => {
          let ch = b as char;
          if ch.is_digit(10) {
            have_digit = true;
            result = result * 10 + (b - ('0' as u8)) as usize;
          } else {
            if ch.is_whitespace() {
              break;
            }
            // A number not terminated by a whitespace is an error
            return Err(ImageErr::NumErr);
          }
        },
        // On io::error, drop through and the it.next() try!
        // will propagate the err
        Some(&Err(_)) => { },
      }
    }
    try!(it.next().expect("read_integer consume"));
  }

  if have_digit {
    Ok(result)
  } else {
    Err(ImageErr::NumErr)
  }
}
