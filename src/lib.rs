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

pub fn forward<C: Complex>(input: &[C]) -> Vec<num::Complex<Precision>> {
	let mut output = vec![num::Complex::new(0.0, 0.0); input.len()];

	if input.len().is_power_of_two() {
		cooley_tukey::forward(input, &mut output);
	}
	else {
		unimplemented!();
	}

	output
}

pub fn inverse<C: Complex>(input: &[C]) -> Vec<num::Complex<Precision>> {
	let mut output = vec![num::Complex::new(0.0, 0.0); input.len()];

	if input.len().is_power_of_two() {
		cooley_tukey::inverse(input, &mut output);
	}
	else {
		unimplemented!();
	}

	output
}
