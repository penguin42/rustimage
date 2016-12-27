use image::*;
use point_line::*;
use std::f64;

const LIGHT_TO_DARK_THRESHOLD : u8 = 65;
const DARK_TO_LIGHT_THRESHOLD : u8 = 105;
const PATH_LINE_BASE_FRAC : f64 = 0.95;
const PATH_LINE_END_FRAC : f64 = 0.90;
const PATH_LINE_SAMPLES : usize = 10;

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
fn follow_edge(i: &Image, d: Direction, line_width: f64,
              inner_start: &Point,
              mid_start: &Point,
              outer_start: &Point) -> Vec<Point> {
  // A history of our mid points, we'll use it to synthesize a line at the end
  let mut history = Vec::new();

  let mut cur_inner = *inner_start;
  let mut cur_mid =   *mid_start;
  let mut cur_outer = *outer_start;

  let mut found_point = cur_mid;
  let mut found;

  println!("find_corner: {:?}/{:?}/{:?} going {:?}", inner_start, mid_start, outer_start, d);
  loop {
    history.push(cur_mid);
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
    //println!("FSVG: <circle cx=\"{}\" cy=\"{}\" r=\"2px\" style=\"stroke:rgb(0,255,0);stroke-width=1\"",
    //         cur_mid.x, cur_mid.y);
  }

  history
}

// Given a vector of points along an edge, generate a line pointing in the direction of the end point
fn gen_edge_vector(history: &Vec<Point>) -> Line {
  // Synthesise a vector from the set of midpoints we've followed; the line is curved
  // so we use some near the end, but not right at the end because we tend to swing
  // off a bit as we approach the new edge (because we always recentre the midpoint on the middle of the
  // black range, and we also tend not to have clean corners)

  // So we try PATH_LINE_SAMPLES out of each of two sets PATH_LINE_BASE/END_FRAC along
  // and build a line from each pair; then figure out which is the best match to the points
  let path_len = history.len();

  let outer_base = (path_len as f64 * PATH_LINE_BASE_FRAC) as usize;
  let inner_base = (path_len as f64 * PATH_LINE_END_FRAC) as usize;

  println!("path_len={} outer_base={} inner_base={}", path_len, outer_base, inner_base);

  let mut best_score = f64::MAX;
  let mut best_line = (Point { x:0, y:0 }, Point { x:0, y:0 });
  for outer_index in 0..PATH_LINE_SAMPLES {
    let outer_point = history[outer_base + outer_index];

    for inner_index in 0..PATH_LINE_SAMPLES {
      let inner_point = history[inner_base + inner_index];
      
      let line = (outer_point, inner_point);

      // Build a score for this line by looping over all the
      // (other) points and measuring their distance from the line
      // so loop over both inner and outer points and get distance
      let mut score = 0.0;

      for score_index in 0..PATH_LINE_SAMPLES {
        if score_index != outer_index {
          score += history[outer_base + score_index].line_distance(&line);
        }
      } 
      for score_index in 0..PATH_LINE_SAMPLES {
        if score_index != inner_index {
          score += history[inner_base + score_index].line_distance(&line);
        }
      } 

      if score < best_score {
        best_line = line;
        best_score = score;
      }
    }
  } 
  //println!("find_corner for {:?} point={:?} best_score={} best_line={:?}",
  //         d, cur_mid, best_score, best_line);
  best_line
}

fn edge_finder(i: &Image, start: &Point, d: Direction) -> (Line,Line,Point) {
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

  // Step4: Find the midpoint of the edge
  let line_width = inner_edge_marker.distance(&outer_edge_marker);
  println!("Line width={}", line_width); 

  let mut mid_point = outer_edge_marker;
  mid_point.step(d, i, (line_width / 2.0) as usize);

  // Step5: Follow the edges to the end/corners
  let mut vec1 = follow_edge(i, d.cntr_clockwise(), line_width, &inner_edge_marker, &mid_point, &outer_edge_marker);
  let mut vec2 = follow_edge(i, d.clockwise(), line_width, &inner_edge_marker, &mid_point, &outer_edge_marker);

  // Step6: Generate vectors pointing along the edge near the corner
  // the caller then combines the vectors from edges that touch to find
  // the corner
  let corner1 = gen_edge_vector(&vec1);
  let corner2 = gen_edge_vector(&vec2);

  // Step7: Find a midpoint on the edge to find as a curve point
  vec1.reverse();
  vec1.append(&mut vec2); // Hmm this dupes the start point?

  let middle_point = vec1[vec1.len()/2];
  (corner1, corner2, middle_point)
}

// Result is points going clockwise from top left with one midpoint on each line
pub fn box_finder(i: &Image) -> (Point, Point, Point, Point, Point, Point, Point, Point) {
  let image_size = i.get_size();
  let left_middle = Point { x: 0, y: image_size.y/2 };
  let right_middle = Point { x: image_size.x - 1, y: image_size.y/2 };
  let top_middle = Point { x: image_size.x/2, y: 0 };
  let bottom_middle = Point { x: image_size.x/2, y: image_size.y-1 };

  let (left_edge_line_top, left_edge_line_bottom, left_edge_mid) =
    edge_finder(i, &left_middle, Direction::Right);
  let (right_edge_line_bottom, right_edge_line_top, right_edge_mid) =
    edge_finder(i, &right_middle, Direction::Left);
  let (top_edge_line_right, top_edge_line_left, top_edge_mid) =
    edge_finder(i, &top_middle, Direction::Down);
  let (bottom_edge_line_left, bottom_edge_line_right, bottom_edge_mid) =
    edge_finder(i, &bottom_middle, Direction::Up);

  let top_left = line_intersection(&left_edge_line_top, &top_edge_line_left);
  let top_right = line_intersection(&right_edge_line_top, &top_edge_line_right);
  let bottom_left = line_intersection(&left_edge_line_bottom, &bottom_edge_line_left);
  let bottom_right = line_intersection(&right_edge_line_bottom, &bottom_edge_line_right);

  // Going clockwise from top left
  (top_left, top_edge_mid, top_right,
             right_edge_mid, bottom_right,
             bottom_edge_mid, bottom_left,
   left_edge_mid)
}

