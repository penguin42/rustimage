use std::env;
use std::io::prelude::*;
use std::fs::File;

mod box_finder;
mod image;
mod point_line;
mod string;

fn plot_svg_line(f: &mut File, l: &point_line::Line, style: &str) -> Result<(), std::io::Error> {
  let (p1, p2) = *l;
  writeln!(f, "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"{}\" />",
           p1.x, p1.y, p2.x, p2.y, style)
}

fn main() {
  let mut our_args = env::args();

  if our_args.len() != 2 {
    panic!("expected one arg but got {}", our_args.len());
  }
  let file_name = our_args.nth(1).unwrap();
  println!("Filename = {}", file_name);

  let in_image = image::load_pgm(&file_name).unwrap();
  let image_size = in_image.get_size();
  let left_middle = point_line::Point { x: 0, y: image_size.y/2 };
  let right_middle = point_line::Point { x: image_size.x - 1, y: image_size.y/2 };
  let top_middle = point_line::Point { x: image_size.x/2, y: 0 };
  let bottom_middle = point_line::Point { x: image_size.x/2, y: image_size.y-1 };

  let (left_edge_line_top, left_edge_line_bottom) = box_finder::edge_finder(&in_image, &left_middle, point_line::Direction::Right);
  let (right_edge_line_bottom, right_edge_line_top) = box_finder::edge_finder(&in_image, &right_middle, point_line::Direction::Left);
  let (top_edge_line_right, top_edge_line_left) = box_finder::edge_finder(&in_image, &top_middle, point_line::Direction::Down);
  let (bottom_edge_line_left, bottom_edge_line_right) = box_finder::edge_finder(&in_image, &bottom_middle, point_line::Direction::Up);

  let mut svgf = File::create("debug.svg").unwrap();
  writeln!(svgf, "<svg height=\"{}px\" width=\"{}px\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">", image_size.x, image_size.y).unwrap();
  writeln!(svgf, "  <image xlink:href=\"{}\" x=\"0\" y=\"0\" width=\"{}px\" height=\"{}px\"/>", file_name, image_size.x, image_size.y).unwrap();
  let green_1_style="stroke:rgb(0,255,0);stroke-width:1";
  plot_svg_line(&mut svgf, &left_edge_line_top, green_1_style).unwrap();
  plot_svg_line(&mut svgf, &left_edge_line_bottom, green_1_style).unwrap();

  plot_svg_line(&mut svgf, &right_edge_line_top, green_1_style).unwrap();
  plot_svg_line(&mut svgf, &right_edge_line_bottom, green_1_style).unwrap();

  plot_svg_line(&mut svgf, &top_edge_line_left, green_1_style).unwrap();
  plot_svg_line(&mut svgf, &top_edge_line_right, green_1_style).unwrap();

  plot_svg_line(&mut svgf, &bottom_edge_line_left, green_1_style).unwrap();
  plot_svg_line(&mut svgf, &bottom_edge_line_right, green_1_style).unwrap();

  writeln!(svgf, "</svg>").unwrap();
}

