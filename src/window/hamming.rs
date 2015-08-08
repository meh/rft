use std::f64::consts::PI;

use super::Function;
use Precision;

pub struct Hamming;

const ALPHA: Precision = 0.53836;
const BETA:  Precision = 0.46164;

const PI2: Precision = (PI * 2.0) as Precision;

impl Function for Hamming {
	fn compute(n: Precision, N: Precision) -> Precision {
		ALPHA - BETA * ((PI2 * n) / (N - 1.0)).cos()
	}
}
