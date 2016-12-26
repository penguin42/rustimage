// A curve to interpolate between points
// This is currently a Quadratic Bezier; pretty simple.

use point_line::Pointf;

pub struct Bezierq {
  pub start : Pointf,
  pub control : Pointf,
  pub end : Pointf,
}

// From https://en.wikipedia.org/wiki/B%C3%A9zier_curve#Quadratic_B.C3.A9zier_curves
fn quad_interp(t: f64, s: f64, c: f64, e: f64) -> f64 {
  (1.0-t)*(1.0-t)*s + 2.0*(1.0-t)*t*c + t*t*e
}

// Reworking the above to give c as the answer and specifying a 
// t and result
fn find_control(s: f64, m: f64, e: f64, mid_t: f64) -> f64 {
  (m-mid_t*mid_t*e-(1.0-mid_t)*(1.0-mid_t)*s) / 
    (2.0 * (1.0 - mid_t) * mid_t)
}

impl Bezierq {
  // Return a point on the curve; t is 0 (start) -> 1 (end)
  pub fn interp(&self, t: f64) -> Pointf {
    Pointf { x: quad_interp(t, self.start.x, self.control.x, self.end.x),
             y: quad_interp(t, self.start.y, self.control.y, self.end.y) }
  }

  // Return a curve that passes through the given points
  // the 'mid' point happens at the specified 't' interpolation point
  pub fn through(s: Pointf, m: Pointf, e: Pointf, mid_t: f64) -> Bezierq {
    Bezierq { start: s, end: e,
              control: Pointf {
                x: find_control(s.x, m.x, e.x, mid_t),
                y: find_control(s.y, m.y, e.y, mid_t) } }
  }
}
