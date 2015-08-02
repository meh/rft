use std::f64::consts::PI;

use {Precision, Sample};
use strided::{Strided, MutStrided};

pub fn hamming<S, I>(input: I) -> Vec<Precision>
	where S: Sample,
	      I: Strided<Elem=S>,
{
	let mut output = vec![0.0; input.as_stride().len()];
	hamming_in(input, &mut *output);

	output
}

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
