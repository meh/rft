extern crate num;

extern crate strided;

#[cfg(feature = "f32")]
pub type Precision = f32;
#[cfg(feature = "f64")]
pub type Precision = f64;

mod real;
pub use real::{Real, RealMut};

mod complex;
pub use complex::{Complex, ComplexMut};

pub mod discrete;

pub mod cooley_tukey;

pub fn forward<CI: Complex, CO: ComplexMut>(input: &[CI]) -> Vec<CO> {
	let mut output = vec![CO::zero(); input.len()];

	if input.len().is_power_of_two() {
		cooley_tukey::forward(input, &mut output);
	}
	else {
		unimplemented!();
	}

	output
}

pub fn inverse<CI: Complex, CO: ComplexMut>(input: &[CI]) -> Vec<CO> {
	let mut output = vec![CO::zero(); input.len()];

	if input.len().is_power_of_two() {
		cooley_tukey::inverse(input, &mut output);
	}
	else {
		unimplemented!();
	}

	// the implementations do no scaling internally
	let length = input.len() as Precision;

	for output in output.iter_mut() {
		output.unscale(length);
	}

	output
}
