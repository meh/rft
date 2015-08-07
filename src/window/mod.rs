mod limits;
use self::limits::Limits;

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
pub fn apply<F, SO, SI, I, L>(limits: L, input: I) -> Vec<SO>
	where F:  Function,
	      SO: SampleMut,
	      SI: Sample,
	      I:  Strided<Elem=SI>,
	      L:  Limits
{
	let     input  = input.as_stride();
	let mut output = vec![SO::zero(); input.len()];
	let     length = input.len();

	// Check the limits are valid for the window.
	debug_assert!(limits.is_valid(length));

	apply_in::<F, SO, SI, _, _, L>(limits, input, &mut *output);

	output
}

pub fn apply_in<F, SO, SI, I, O, L>(limits: L, input: I, mut output: O)
	where F:  Function,
	      SO: SampleMut,
	      SI: Sample,
	      I:  Strided<Elem=SI>,
	      O:  MutStrided<Elem=SO>,
	      L:  Limits
{
	let     input  = input.as_stride();
	let mut output = output.as_stride_mut();
	let     length = input.len();

	// `input` and `output` buffers need to be the same length.
	debug_assert_eq!(input.len(), output.len());

	// Check the limits are valid for the window.
	debug_assert!(limits.is_valid(length));

	for (index, (input, output)) in input.iter().zip(output.iter_mut()).enumerate() {
		if index >= limits.start().unwrap_or(0) as usize &&
		   index <= limits.end().unwrap_or(length as u32) as usize
		{
			output.set_normalized(input.normalize()
				* F::compute(index as Precision, limits.width(length) as Precision));
		}
	}
}

pub fn apply_on<F, S, IO, L>(limits: L, mut data: IO)
	where F:  Function,
	      S:  SampleMut,
	      IO: MutStrided<Elem=S>,
	      L:  Limits
{
	let mut data   = data.as_stride_mut();
	let     length = data.len();

	// Check the limits are valid for the window.
	debug_assert!(limits.is_valid(length));

	for (index, datum) in data.iter_mut().enumerate() {
		if index >= limits.start().unwrap_or(0) as usize &&
		   index <= limits.end().unwrap_or(length as u32) as usize
		{
			let value = datum.normalize();

			datum.set_normalized(value
				* F::compute(index as Precision, limits.width(length) as Precision));
		}
	}
}

pub fn generate<F, S, L>(limits: L, size: usize) -> Window<S>
	where F: Function,
	      S: SampleMut,
	      L: Limits
{
	let mut output = Window::new(&limits, size);

	// Check the limits are valid for the window.
	debug_assert!(limits.is_valid(size));

	for (index, output) in output.iter_mut().enumerate() {
		if index >= limits.start().unwrap_or(0) as usize &&
		   index <= limits.end().unwrap_or(size as u32) as usize
		{
			SampleMut::set_normalized(output,
				F::compute(index as Precision, limits.width(size) as Precision));
		}
	}

	output
}
