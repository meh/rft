use super::Function;
use {Precision};

/// https://en.wikipedia.org/wiki/Window_function#Rectangular_window
pub struct Rectangular;

impl Function for Rectangular {
	fn compute(_n: Precision, _N: Precision) -> Precision {
		1.0
	}
}
