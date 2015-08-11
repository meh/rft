//! Rust Fourier Transform.
//!
//! The length of the input must not be a power of two, it picks the proper
//! algorithm automatically.
//!
//! Cooley-Tukey is used for power of two sizes, it has an O(N log N) time complexity
//! and O(N) space complexity.
//!
//! Bluestein is used for non-power of two sizes, it has an O(5N log N) time complexity
//! and O(5N) space complexity.
//!
//! If it isn't already obvious, try to use power of two sizes.

#![allow(non_snake_case)]
#![warn(missing_docs)]

extern crate num;

extern crate strided;
use strided::{Strided, MutStrided};

/// The float precision all operations use.
#[cfg(all(not(feature = "f64"), not(feature = "f32")))]
pub type Precision = f32;
#[cfg(feature = "f32")]
pub type Precision = f32;
#[cfg(feature = "f64")]
pub type Precision = f64;

mod sample;
pub use sample::{Sample, SampleMut};

mod complex;
pub use complex::{Complex, ComplexMut};

/// Various algorithms to compute the fourier transform.
pub mod transform;
use transform::{cooley_tukey, bluestein};

/// Window function application.
pub mod window;
pub use window::Window;

/// Spectrum computations.
pub mod spectrum;

/// Applies a forward fourier transform to the given input and returns a vector
/// of complex numbers.
#[inline(always)]
pub fn forward<CI, CO, I>(input: I) -> Vec<CO>
	where CI: Complex,
	      CO: ComplexMut,
	      I:  Strided<Elem=CI>
{
	let mut output = vec![CO::zero(); input.as_stride().len()];
	forward_in(input, &mut *output);

	output
}

/// Applies a forward fourier transform to the given input and puts it into the
/// given output.
#[inline]
pub fn forward_in<CI, CO, I, O>(input: I, mut output: O)
	where CI: Complex,
	      CO: ComplexMut,
	      I:  Strided<Elem=CI>,
	      O:  MutStrided<Elem=CO>
{
	let input  = input.as_stride();
	let output = output.as_stride_mut();

	if input.len().is_power_of_two() {
		cooley_tukey::forward(input, output);
	}
	else {
		bluestein::forward(input, output);
	}
}

/// Applies an inverse fourier transform to the given input and returns a
/// vector of complex numbers.
#[inline(always)]
pub fn inverse<CI, CO, I>(input: I) -> Vec<CO>
	where CI: Complex,
	      CO: ComplexMut,
	      I:  Strided<Elem=CI>
{
	let mut output = vec![CO::zero(); input.as_stride().len()];
	inverse_in(input, &mut *output);

	output
}

/// Applies an inverse fourier transform to the given input and puts it into
/// the given output.
#[inline]
pub fn inverse_in<CI, CO, I, O>(input: I, mut output: O)
	where CI: Complex,
	      CO: ComplexMut,
	      I:  Strided<Elem=CI>,
	      O:  MutStrided<Elem=CO>
{
	let     input  = input.as_stride();
	let mut output = output.as_stride_mut();

	if input.len().is_power_of_two() {
		cooley_tukey::inverse(input, output.reborrow());
	}
	else {
		bluestein::inverse(input, output.reborrow());
	}

	// the implementations do no scaling internally
	let length = input.len() as Precision;

	for output in output.iter_mut() {
		output.unscale(length);
	}
}
