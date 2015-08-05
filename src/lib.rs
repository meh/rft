extern crate num;

extern crate strided;
use strided::{Strided, MutStrided};

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

pub mod discrete;

pub mod cooley_tukey;

pub mod bluestein;

pub mod window;

pub mod spectrum;

#[inline]
pub fn forward<CI, CO, I>(input: I) -> Vec<CO>
	where CI: Complex,
	      CO: ComplexMut,
	      I:  Strided<Elem=CI>
{
	let mut output = vec![CO::zero(); input.as_stride().len()];
	forward_in(input, &mut *output);

	output
}

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

#[inline]
pub fn inverse<CI, CO, I>(input: I) -> Vec<CO>
	where CI: Complex,
	      CO: ComplexMut,
	      I:  Strided<Elem=CI>
{
	let mut output = vec![CO::zero(); input.as_stride().len()];
	inverse_in(input, &mut *output);

	output
}

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
