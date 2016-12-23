use std::env;
mod image;
mod string;
mod box_finder;

fn main() {
  let mut our_args = env::args();

  if our_args.len() != 2 {
    panic!("expected one arg but got {}", our_args.len());
  }
  let file_name = our_args.nth(1).unwrap();
  println!("Filename = {}", file_name);

  let in_image = image::load_pgm(file_name).unwrap();
  let image_size = in_image.get_size();
  let left_middle = image::Point { x: 0, y: image_size.y/2 };
  let right_middle = image::Point { x: image_size.x - 1, y: image_size.y/2 };
  let top_middle = image::Point { x: image_size.x/2, y: 0 };
  let bottom_middle = image::Point { x: image_size.x/2, y: image_size.y-1 };
  println!("- > -edge search result: {:?}", box_finder::edge_finder(&in_image, &left_middle, image::Direction::Right));
  println!("- < -edge search result: {:?}", box_finder::edge_finder(&in_image, &right_middle, image::Direction::Left));
  println!("- v -edge search result: {:?}", box_finder::edge_finder(&in_image, &top_middle, image::Direction::Down));
  println!("- ^ -edge search result: {:?}", box_finder::edge_finder(&in_image, &bottom_middle, image::Direction::Up));
}

