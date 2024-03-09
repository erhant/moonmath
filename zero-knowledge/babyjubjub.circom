pragma circom 2.1.0;

/// Asserts that a given point (x, y) is a valid point on the
/// BabyJubJub curve. The curve has equation:
///
/// a * x^2 + y^2 = 1 + d * x^2 * y^2
///
/// where a = 168700 and d = 168696.
///
/// Inputs:
/// - x: x-coordinate
/// - y: y coordinate
template BabyJubJub() {
  signal input x;
  signal input y;

  var a = 168700;
  var d = 168696;

  signal x2 <== x*x;
  signal y2 <== y*y;

  a*x2 + y2 === 1 + d*x2*y2;
}

// for some reason Groth16 verification failed when there were no public signals
// so I added (x, y) as a public signal
component main {public [x, y]} = BabyJubJub();
 