// finds the real root of the equation ax^2 + bx + c = 0
// if none exists returns None
// the roots returned should always be ordered in ascending order
pub fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<[f32; 2]> {
	let delta = b * b - 4.0 * a * c;
	if delta < 0.0 {
		return None;
	}

	let delta_sqrt = delta.sqrt();

	Some([
		(-b - delta_sqrt) / (2.0 * a), //
		(-b + delta_sqrt) / (2.0 * a),
	])
}
