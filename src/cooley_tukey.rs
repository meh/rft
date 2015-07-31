use num;
use strided::Stride;
use std::f64::consts::PI;

use {Precision, Complex, ComplexMut};

fn fft<CI: Complex, CO: ComplexMut>(direction: Precision, input: Stride<CI>, output: &mut [CO]) {
	// cache the length
	let length = input.len();

	// base case: the DFT of a single element is itself.
	if length == 1 {
		output[0].set(&input[0]);

		return;
	}

	// split the input into two arrays of alternating elements ("decimate in
	// time")
	let (evens, odds) = input.substrides2();

	// break the output into two halves (front and back, not alternating)
	let (left, right) = output.split_at_mut(length >> 1);

	// recursively perform two FFTs on alternating elements of the input, writing
	// the results into the first and second half of the output array
	// respectively
	fft(direction, evens, left);
	fft(direction, odds, right);

	// exp(-2πi/N)
	let twiddle = num::Complex::from_polar(&1.0,
		&(direction * PI as Precision / length as Precision));

	let mut factor = num::Complex::new(1.0, 0.0);

	// combine the subFFTs with the relations:
	//   X_k       = E_k + exp(-2πki/N) * O_k
	//   X_{k+N/2} = E_k - exp(-2πki/N) * O_k
	for (even, odd) in left.iter_mut().zip(right.iter_mut()) {
		let twiddled = factor * odd.to_num();
		let e        = even.to_num();

		even.set(&(e + twiddled));
		odd.set(&(e - twiddled));

		factor = factor * twiddle;
	}
}

pub fn forward<CI: Complex, CO: ComplexMut>(input: &[CI], output: &mut [CO]) {
	// input and output buffers need to be the same length
	assert_eq!(input.len(), output.len());

	// the length has to be a power of two
	assert!(input.len().is_power_of_two(), "length is not a power of two");

	fft(-2.0, Stride::new(input), output);
}

pub fn inverse<CI: Complex, CO: ComplexMut>(input: &[CI], output: &mut [CO]) {
	// input and output buffers need to be the same length
	assert_eq!(input.len(), output.len());

	// the length has to be a power of two
	assert!(input.len().is_power_of_two(), "length is not a power of two");

	fft(2.0, Stride::new(input), output);

	let length = input.len() as Precision;

	// finish the inversion by unscaling by the length
	for output in output.iter_mut() {
		let real = output.real() / length;
		let imag = output.imag() / length;

		output.set_real(real);
		output.set_imag(imag);
	}
}
