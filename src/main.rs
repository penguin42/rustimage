use std::env;
use std::io::prelude::*;
use std::fs::File;

mod box_finder;
mod curve;
mod image;
mod point_line;
mod string;

use point_line::*;

fn plot_svg_line(f: &mut File, l: &point_line::Line, style: &str) -> Result<(), std::io::Error> {
  let (p1, p2) = *l;
  writeln!(f, "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"{}\" />",
           p1.x, p1.y, p2.x, p2.y, style)
}

fn plot_svg_our_bez(f: &mut File, bez: &curve::Bezierq, steps: usize, style: &str) -> Result<(), std::io::Error> {
  let mut prev = Point::from(bez.interp(0.0));

  for p in 1..(steps+1) {
    let t = p as f64 / steps as f64;
    let cur = Point::from(bez.interp(t));

    try!(plot_svg_line(f, &(prev, cur), style));
    prev = cur;
  }
  Ok(())
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

  let mut svgf = File::create("debug.svg").unwrap();
  writeln!(svgf, "<svg height=\"{}px\" width=\"{}px\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">", image_size.x, image_size.y).unwrap();
  writeln!(svgf, "  <image xlink:href=\"{}\" x=\"0\" y=\"0\" width=\"{}px\" height=\"{}px\"/>", file_name, image_size.x, image_size.y).unwrap();

  let (top_left, top_edge_mid, top_right,
                 right_edge_mid, bottom_right,
                 bottom_edge_mid, bottom_left,
                 left_edge_mid) = box_finder::box_finder(&in_image);

  let green_1_style="stroke:rgb(0,255,0);stroke-width:1";
  let cyan_1_style="stroke:rgb(0,255,255);stroke-width:1";
  plot_svg_line(&mut svgf, &(left_edge_mid, right_edge_mid), cyan_1_style).unwrap();
  plot_svg_line(&mut svgf, &(top_edge_mid, bottom_edge_mid), cyan_1_style).unwrap();

  let top_bez = curve::Bezierq::through(Pointf::from(top_left),
                                        Pointf::from(top_edge_mid),
                                        Pointf::from(top_right),
                                        0.5);
  let bottom_bez = curve::Bezierq::through(Pointf::from(bottom_left),
                                        Pointf::from(bottom_edge_mid),
                                        Pointf::from(bottom_right),
                                        0.5);
  let left_bez = curve::Bezierq::through(Pointf::from(bottom_left),
                                        Pointf::from(left_edge_mid),
                                        Pointf::from(top_left),
                                        0.5);
  let right_bez = curve::Bezierq::through(Pointf::from(bottom_right),
                                        Pointf::from(right_edge_mid),
                                        Pointf::from(top_right),
                                        0.5);
  let orange_1_style="stroke:rgb(255,138,0);stroke-width:1";
  plot_svg_our_bez(&mut svgf, &top_bez, 10, orange_1_style).unwrap();
  plot_svg_our_bez(&mut svgf, &bottom_bez, 10, orange_1_style).unwrap();
  plot_svg_our_bez(&mut svgf, &left_bez, 10, orange_1_style).unwrap();
  plot_svg_our_bez(&mut svgf, &right_bez, 10, orange_1_style).unwrap();
  writeln!(svgf, "</svg>").unwrap();
}

