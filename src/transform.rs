/// (c) David Alan Gilbert <dave@treblig.org> 2016
/// Licensed under GPLv3, see the LICENSE file for a full copy

use curve;
use image;
use point_line::Point;

pub fn transform(i: &image::Image, o: &mut image::Image,
                 lcurve: &curve::Bezierq,
                 mcurve: &curve::Bezierq,
                 rcurve: &curve::Bezierq) {
  let out_size = o.get_size();
  for y in 0..out_size.y {
    let t_y = y as f64 / (out_size.y as f64);
    let left_point = lcurve.interp(t_y);
    let mid_point  = mcurve.interp(t_y);
    let right_point = rcurve.interp(t_y);

    // For each horizontal line we create a bezier through the source
    let line_bez = curve::Bezierq::through(left_point, mid_point, right_point, 0.5);

    println!("transform: y={} t_y={} {:?}/{:?}/{:?}", y, t_y, left_point, mid_point, right_point);
    for x in 0..out_size.x {
      let t_x = x as f64 / (out_size.x as f64);
      let cur_point = Point::from(line_bez.interp(t_x));

      // TODO: Use some type of interpolation based on the fractional location
      // to get an average from neighbours
      o[Point { x: x, y: y }] = i[cur_point];
    }
  }
}
