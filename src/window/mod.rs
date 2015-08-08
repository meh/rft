mod range;
pub use self::range::Range;

mod window;
pub use self::window::Window;

mod rectangular;
pub use self::rectangular::Rectangular;

mod triangular;
pub use self::triangular::Triangular;

mod bartlett;
pub use self::bartlett::Bartlett;

mod hamming;
pub use self::hamming::Hamming;

mod hann;
pub use self::hann::Hann;

use num::Zero;

use {Precision, Sample, SampleMut};
use strided::{Strided, MutStrided};

pub trait Function {
	fn compute(n: Precision, N: Precision) -> Precision;
}

#[inline(always)]
pub fn compute<F, S>(index: usize, width: usize) -> S
	where F: Function,
	      S: SampleMut
{
	let mut result = S::zero();
	result.set_normalized(F::compute(index as Precision, width as Precision));

	result
}

#[inline(always)]
pub fn apply<F, SO, SI, I, R>(range: R, input: I) -> Vec<SO>
	where F:  Function,
	      SO: SampleMut,
	      SI: Sample,
	      I:  Strided<Elem=SI>,
	      R:  Range
{
	let     input  = input.as_stride();
	let mut output = vec![SO::zero(); input.len()];
	let     length = input.len();

	// Check the range are valid for the window.
	debug_assert!(range.is_valid(length));

	apply_in::<F, SO, SI, _, _, R>(range, input, &mut *output);

	output
}

pub fn apply_in<F, SO, SI, I, O, R>(range: R, input: I, mut output: O)
	where F:  Function,
	      SO: SampleMut,
	      SI: Sample,
	      I:  Strided<Elem=SI>,
	      O:  MutStrided<Elem=SO>,
	      R:  Range
{
	let     input  = input.as_stride();
	let mut output = output.as_stride_mut();
	let     length = input.len();

	// `input` and `output` buffers need to be the same length.
	debug_assert_eq!(input.len(), output.len());

	// Check the range are valid for the window.
	debug_assert!(range.is_valid(length));

	for (index, (input, output)) in input.iter().zip(output.iter_mut()).enumerate() {
		if index >= range.start().unwrap_or(0) as usize &&
		   index <= range.end().unwrap_or(length as u32) as usize
		{
			output.set_normalized(input.normalize()
				* F::compute(index as Precision, range.width(length) as Precision));
		}
	}
}

pub fn apply_on<F, S, IO, R>(range: R, mut data: IO)
	where F:  Function,
	      S:  SampleMut,
	      IO: MutStrided<Elem=S>,
	      R:  Range
{
	let mut data   = data.as_stride_mut();
	let     length = data.len();

	// Check the range are valid for the window.
	debug_assert!(range.is_valid(length));

	for (index, datum) in data.iter_mut().enumerate() {
		if index >= range.start().unwrap_or(0) as usize &&
		   index <= range.end().unwrap_or(length as u32) as usize
		{
			let value = datum.normalize();

			datum.set_normalized(value
				* F::compute(index as Precision, range.width(length) as Precision));
		}
	}
}

pub fn generate<F, S, R>(range: R, size: usize) -> Window<S>
	where F: Function,
	      S: SampleMut,
	      R: Range
{
	let mut output = Window::new(&range, size);

	// Check the range are valid for the window.
	debug_assert!(range.is_valid(size));

	for (index, output) in output.iter_mut().enumerate() {
		if index >= range.start().unwrap_or(0) as usize &&
		   index <= range.end().unwrap_or(size as u32) as usize
		{
			SampleMut::set_normalized(output,
				F::compute(index as Precision, range.width(size) as Precision));
		}
	}

	output
}
