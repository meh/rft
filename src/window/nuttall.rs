use std::f64::consts::PI;

use super::Function;
use {Precision};

pub struct Nuttall;

const ALPHA0: Precision = 0.355768;
const ALPHA1: Precision = 0.487396;
const ALPHA2: Precision = 0.144232;
const ALPAH3: Precision = 0.012604;

const PI2: Precision = (PI * 2.0) as Precision;
const PI4: Precision = (PI * 4.0) as Precision;
const PI6: Precision = (PI * 6.0) as Precision;

impl Function for Nuttall {
	fn compute(n: Precision, N: Precision) -> Precision {
		ALPHA0 -
		ALPHA1 * ((PI2 * n) / (N - 1.0)).cos() +
		ALPHA2 * ((PI4 * n) / (N - 1.0)).cos() -
		ALPHA3 * ((PI6 * n) / (N - 1.0)).cos()
	}
}
