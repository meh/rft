extern crate num;

extern crate strided;
use strided::{Strided, MutStrided};

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

pub mod bluestein;

pub fn forward<CI: Complex, CO: ComplexMut, I: Strided<Elem=CI>>(input: I) -> Vec<CO> {
	let     input  = input.as_stride();
	let mut output = vec![CO::zero(); input.len()];

	if input.len().is_power_of_two() {
		cooley_tukey::forward(input, output.as_stride_mut());
	}
	else {
		bluestein::forward(input, output.as_stride_mut());
	}

	output
}

pub fn inverse<CI: Complex, CO: ComplexMut, I: Strided<Elem=CI>>(input: I) -> Vec<CO> {
	let     input  = input.as_stride();
	let mut output = vec![CO::zero(); input.len()];

	if input.len().is_power_of_two() {
		cooley_tukey::inverse(input, output.as_stride_mut());
	}
	else {
		bluestein::inverse(input, output.as_stride_mut());
	}

	// the implementations do no scaling internally
	let length = input.len() as Precision;

	for output in output.iter_mut() {
		output.unscale(length);
	}

	output
}
