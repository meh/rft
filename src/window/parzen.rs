use super::Function;
use {Precision};

pub struct Parzen;

impl Function for Parzen {
	fn compute(n: Precision, N: Precision) -> Precision {
		if n >= 0.0 && n <= N / 4.0 {
			1.0 - 6.0 * (n / (N / 2.0)).powf(2.0) * (1.0 - (n.abs() / (N / 2.0)))
		}
		else if n >= N / 4.0 && n <= N / 2.0 {
			2.0 * (1.0 - (n.abs() / (N / 2.0)))
		}
		else {
			0.0
		}
	}
}
