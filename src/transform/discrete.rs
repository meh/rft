use std::f64::consts::PI;
use strided::{Stride, MutStride};

use {Precision, Complex, ComplexMut};

fn dft<CI: Complex, CO: ComplexMut>(direction: Precision, input: Stride<CI>, mut output: MutStride<CO>) {
	debug_assert_eq!(input.len(), output.len());

	let length = input.len() as Precision;

	for (i_out, output) in output.iter_mut().enumerate() {
		let mut real = 0.0;
		let mut imag = 0.0;

		for (i_in, input) in input.iter().enumerate() {
			let angle = direction
				* PI as Precision
				* i_in as Precision
				* i_out as Precision
				/ length;

			real +=  input.real() * angle.cos() + input.imag() * angle.sin();
			imag += -input.real() * angle.sin() + input.imag() * angle.cos();
		}

		output.set_real(real);
		output.set_imag(imag);
	}
}

/// Applies a forward discrete Fourier transform on the given input and puts
/// the result in the given output.
#[inline(always)]
pub fn forward<CI: Complex, CO: ComplexMut>(input: Stride<CI>, output: MutStride<CO>) {
	// input and output buffers need to be the same length
	debug_assert_eq!(input.len(), output.len());

	dft(2.0, input, output);
}

/// Applies an inverse discrete Fourier transform on the given input and puts
/// the result in the given output.
///
/// Note the result is not scaled.
#[inline(always)]
pub fn inverse<CI: Complex, CO: ComplexMut>(input: Stride<CI>, output: MutStride<CO>) {
	// input and output buffers need to be the same length
	debug_assert_eq!(input.len(), output.len());

	dft(-2.0, input, output);
}
