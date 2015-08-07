use super::Function;
use {Precision};

pub struct Rectangular;

impl Function for Rectangular {
	fn compute(_n: Precision, _N: Precision) -> Precision {
		1.0
	}
}
