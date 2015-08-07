use std::f64::consts::PI;

use super::Function;
use Precision;

pub struct Hann;

const PI2: Precision = (PI * 2.0) as Precision;

impl Function for Hann {
	fn compute(n: Precision, N: Precision) -> Precision {
		0.5 * (1.0 - ((PI2 * n) / (N - 1.0)).cos())
	}
}
