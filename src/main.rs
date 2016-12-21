use std::env;
mod image;

fn main() {
  let mut our_args = env::args();

  if our_args.len() != 2 {
    panic!("expected one arg but got {}", our_args.len());
  }
  let file_name = our_args.nth(1).unwrap();
  println!("Filename = {}", file_name);

  let in_image = image::load_pgm(file_name).unwrap();
}

