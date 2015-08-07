use super::Function;
use {Precision};

pub struct Bartlett;

impl Function for Bartlett {
	fn compute(n: Precision, N: Precision) -> Precision {
		1.0 - ((n - (N - 1.0) / 2.0) / ((N - 1.0) / 2.0))
	}
}
