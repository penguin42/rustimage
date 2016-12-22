use std::env;
mod image;
mod string;

fn main() {
  let mut our_args = env::args();

  if our_args.len() != 2 {
    panic!("expected one arg but got {}", our_args.len());
  }
  let file_name = our_args.nth(1).unwrap();
  println!("Filename = {}", file_name);

  let in_image = image::load_pgm(file_name).unwrap();

  let mut here = image::Point { x: 2, y: 0 };

  println!("Top left pixel={}", in_image[image::Point { x: 0, y: 0 }]);
  println!("Here={:?}", here);
  println!("Here step {} down={:?}", here.step(image::Direction::Down, &in_image), here);
  println!("Here step {} left={:?}", here.step(image::Direction::Left, &in_image), here);
  println!("Here step {} left={:?}", here.step(image::Direction::Left, &in_image), here);
  println!("Here step {} left={:?}", here.step(image::Direction::Left, &in_image), here);
  println!("Here step {} right={:?}", here.step(image::Direction::Right, &in_image), here);
  println!("Here step {} up={:?}", here.step(image::Direction::Up, &in_image), here);
}

