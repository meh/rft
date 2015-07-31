use std::f64::consts::PI;

use {Precision, Complex, ComplexMut};

pub fn dft<CI: Complex, CO: ComplexMut>(direction: Precision, input: &[CI], output: &mut [CO]) {
	assert_eq!(input.len(), output.len());

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

pub fn forward<CI: Complex, CO: ComplexMut>(input: &[CI], output: &mut [CO]) {
	dft(2.0, input, output);
}

pub fn inverse<CI: Complex, CO: ComplexMut>(input: &[CI], output: &mut [CO]) {
	dft(-2.0, input, output);
}
