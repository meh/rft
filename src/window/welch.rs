use super::Function;
use {Precision};

/// https://en.wikipedia.org/wiki/Window_function#Welch_window
pub struct Welch;

impl Function for Welch {
	fn compute(n: Precision, N: Precision) -> Precision {
		1.0 - ((n - (N - 1.0) / 2.0) / ((N - 1.0) / 2.0))
	}
}
