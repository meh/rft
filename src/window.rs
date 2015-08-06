use std::f64::consts::PI;

use {Precision, Sample, SampleMut};
use strided::{Strided, MutStrided};

/// Creates a Hamming window from the given input.
#[inline(always)]
pub fn hamming<S, I>(input: I) -> Vec<Precision>
	where S: Sample,
	      I: Strided<Elem=S>,
{
	let mut output = vec![0.0; input.as_stride().len()];
	hamming_in(input, &mut *output);

	output
}

/// Sets a Hamming window on the given output from the given input.
pub fn hamming_in<S, I, O>(input: I, mut output: O)
	where S: Sample,
	      I: Strided<Elem=S>,
	      O: MutStrided<Elem=Precision>
{
	let     input  = input.as_stride();
	let mut output = output.as_stride_mut();

	// input and output buffers need to be the same length
	debug_assert_eq!(input.len(), output.len());

	let length = input.len() as Precision;

	for (i, (input, output)) in input.iter().zip(output.iter_mut()).enumerate() {
		*output = input.normalize()
			* (0.54 - 0.46)
			* (((PI * 2.0) as Precision * i as Precision) / (length - 1.0));
	}
}

/// Sets a Hamming window in place with the given data.
pub fn hamming_on<S, IO>(mut data: IO)
	where S:  SampleMut,
	      IO: MutStrided<Elem=S>
{
	let mut data   = data.as_stride_mut();
	let     length = data.len() as Precision;

	for (i, datum) in data.iter_mut().enumerate() {
		let new = datum.normalize()
			* (0.54 - 0.46)
			* (((PI * 2.0) as Precision * i as Precision) / (length - 1.0));

		datum.set_normalized(new);
	}
}
