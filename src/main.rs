use std::env;
use std::io::prelude::*;
use std::fs::File;

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

  let in_image = image::load_pgm(&file_name).unwrap();
  let image_size = in_image.get_size();
  let left_middle = image::Point { x: 0, y: image_size.y/2 };
  let right_middle = image::Point { x: image_size.x - 1, y: image_size.y/2 };
  let top_middle = image::Point { x: image_size.x/2, y: 0 };
  let bottom_middle = image::Point { x: image_size.x/2, y: image_size.y-1 };

  let (left_edge_corner_top_left, left_edge_corner_bottom_left) = box_finder::edge_finder(&in_image, &left_middle, image::Direction::Right);
  let (right_edge_corner_bottom_right, right_edge_corner_top_right) = box_finder::edge_finder(&in_image, &right_middle, image::Direction::Left);
  let (top_edge_corner_top_right, top_edge_corner_top_left) = box_finder::edge_finder(&in_image, &top_middle, image::Direction::Down);
  let (bottom_edge_corner_bottom_left, bottom_edge_corner_bottom_right) = box_finder::edge_finder(&in_image, &bottom_middle, image::Direction::Up);

  let mut svgf = File::create("debug.svg").unwrap();
  writeln!(svgf, "<svg height=\"{}px\" width=\"{}px\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">", image_size.x, image_size.y).unwrap();
  writeln!(svgf, "  <image xlink:href=\"{}\" x=\"0\" y=\"0\" width=\"{}px\" height=\"{}px\"/>", file_name, image_size.x, image_size.y).unwrap();
  writeln!(svgf, "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"stroke:rgb(255,0,0);stroke-width:1\" />",
           left_edge_corner_top_left.x, left_edge_corner_top_left.y,
           left_edge_corner_bottom_left.x, left_edge_corner_bottom_left.y).unwrap();
  writeln!(svgf, "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"stroke:rgb(255,0,0);stroke-width:1\" />",
           right_edge_corner_top_right.x, right_edge_corner_top_right.y,
           right_edge_corner_bottom_right.x, right_edge_corner_bottom_right.y).unwrap();
  writeln!(svgf, "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"stroke:rgb(255,0,0);stroke-width:1\" />",
           top_edge_corner_top_left.x, top_edge_corner_top_left.y,
           top_edge_corner_top_right.x, top_edge_corner_top_right.y).unwrap();
  writeln!(svgf, "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"stroke:rgb(255,0,0);stroke-width:1\" />",
           bottom_edge_corner_bottom_left.x, bottom_edge_corner_bottom_left.y,
           bottom_edge_corner_bottom_right.x, bottom_edge_corner_bottom_right.y).unwrap();
  writeln!(svgf, "</svg>").unwrap();
}

