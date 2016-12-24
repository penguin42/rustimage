use image::*;

const LIGHT_TO_DARK_THRESHOLD : u8 = 65;
const DARK_TO_LIGHT_THRESHOLD : u8 = 105;

// Hmm this might be tricky - my top edge brightness is so bridgt I'm seeing speckling in the line
// the other edges we're good down to about 25 as black  - same problem on bottom edge
// Contrast stretch the entire thing first?
// Idea: Get slope based on point at 15% and 5% from the end ,then do intersect of matching corners

// Moves until we hit a light point; returns the last dark point
// (or the original if it was light)
fn step_to_light(i: &Image, start: &Point, d: Direction) -> Point {
  let mut cur = *start;
  let mut res = cur;

  'find_light: loop {
    if i[cur] > DARK_TO_LIGHT_THRESHOLD { break 'find_light; }
    res = cur;
    if !cur.step(d, i, 1) { panic!("Ran off edge finding light"); }
  }

  res
}

// We're given the bounds and middle of a line and expected to find where the end of it is in
// direction 'd'.  Note the 'd' is a compass direction since we don't know the slope of the line
fn find_corner(i: &Image, d: Direction, line_width: f64,
              inner_start: &Point, mid_start: &Point, outer_start: &Point) -> Point {

  let mut cur_inner = *inner_start;
  let mut cur_mid =   *mid_start;
  let mut cur_outer = *outer_start;

  let mut found_point = cur_mid;
  let mut found;

  println!("find_corner: {:?}/{:?}/{:?} going {:?}", inner_start, mid_start, outer_start, d);
  loop {
    found = false;
    if !cur_inner.step(d, i, 1) ||
       !cur_mid.step(d, i, 1) ||
       !cur_outer.step(d, i, 1) {
      panic!("Fell off edge {:?}/{:?}/{:?}", cur_inner, cur_mid, cur_outer);
    };

    if i[cur_mid] <= DARK_TO_LIGHT_THRESHOLD {
      found_point = cur_mid;
      found = true;
    }
    if !found && i[cur_outer] <= DARK_TO_LIGHT_THRESHOLD {
      found_point = cur_outer;
      found = true;
    }
    if !found && i[cur_inner] <= DARK_TO_LIGHT_THRESHOLD {
      found_point = cur_inner;
      found = true;
    }
    if !found {
      println!("find_corner: Hit blank at {:?}/{:?}/{:?}", cur_outer,cur_mid,cur_inner);
      break;
    }
    // Find the middle of our current line
    let proto_outer = step_to_light(i, &found_point, d.cntr_clockwise());
    let proto_inner = step_to_light(i, &found_point, d.clockwise());
    let distance = proto_outer.distance(&proto_inner);

    if distance > line_width * 3.0 {
      // Looks like we've hit the corner because we've started running along a dark edge
      println!("find_corner: Hit other edge at {:?}", found_point);
      break;
    }

    cur_outer = proto_outer;
    cur_mid = proto_outer;
    cur_mid.step(d.clockwise(), i, (distance/2.0) as usize);
    cur_inner = proto_inner;
  }
  cur_mid
}

pub fn edge_finder(i: &Image, start: &Point, d: Direction) -> (Point,Point) {
  let mut cur = *start;

  println!("edge_finder: {:?} going {:?}", start, d);
  // Step1: Find a white area in case the edge is in shadow
  'find_init_white: loop {
    if i[cur] > DARK_TO_LIGHT_THRESHOLD { break 'find_init_white; }
    if !cur.step(d, i, 1) { panic!("Ran off edge finding white"); }
  }
  println!("Have white at {:?}", cur);

  // Step2: Find the edge of the line
  'find_outer_edge: loop {
    if i[cur] < LIGHT_TO_DARK_THRESHOLD { break 'find_outer_edge; }
    if !cur.step(d, i, 1) { panic!("Ran off edge finding outer edge"); }
  }
  let outer_edge_marker = cur;

  // Step3: Find the inner edge of the line
  let inner_edge_marker = step_to_light(i, &cur, d);

  let line_width = inner_edge_marker.distance(&outer_edge_marker);
  println!("Line width={}", line_width); 

  let mut mid_point = outer_edge_marker;
  mid_point.step(d, i, (line_width / 2.0) as usize);

  let corner1 = find_corner(i, d.cntr_clockwise(), line_width, &inner_edge_marker, &mid_point, &outer_edge_marker);
  let corner2 = find_corner(i, d.clockwise(), line_width, &inner_edge_marker, &mid_point, &outer_edge_marker);
  (corner1, corner2)
}

