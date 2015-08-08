use std::f64::consts::PI;

use super::Function;
use {Precision};

pub struct Welch;

const ALPHA: Precision = 0.16;
const BETA:  Precision = 1.73;

const ALPHA0: Precision = (1.0 - ALPHA) / 2.0;
const ALPHA1: Precision = 1.0 / 2.0;
const ALPHA2: Precision = ALPHA / 2.0;

const PI2: Precision = (PI * 2.0) as Precision;
const PI4: Precision = (PI * 4.0) as Precision;

impl Function for Blackman {
	fn compute(n: Precision, N: Precision) -> Precision {
		ALPHA0 -
		ALPHA1 * ((PI2 * n) / (N - 1.0)).cos() +
		ALPHA2 * ((PI4 * n) / (N - 1.0)).cos()
	}
}
