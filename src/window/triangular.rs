use super::Function;
use {Precision};

pub struct Triangular;

impl Function for Triangular {
	fn compute(n: Precision, N: Precision) -> Precision {
		1.0 - ((n - (N - 1.0) / 2.0) / (N / 2.0))
	}
}
