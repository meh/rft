use num;
use strided::{Stride, MutStride};
use std::f64::consts::PI;

use {Precision, Complex, ComplexMut};

fn fft<CI: Complex, CO: ComplexMut>(direction: Precision, input: Stride<CI>, mut output: MutStride<CO>) {
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
	let (mut left, mut right) = output.split_at_mut(length >> 1);

	// recursively perform two FFTs on alternating elements of the input, writing
	// the results into the first and second half of the output array
	// respectively
	fft(direction, evens, left.reborrow());
	fft(direction, odds, right.reborrow());

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

#[inline(always)]
pub fn forward<CI: Complex, CO: ComplexMut>(input: Stride<CI>, output: MutStride<CO>) {
	// input and output buffers need to be the same length
	debug_assert_eq!(input.len(), output.len());

	// the length has to be a power of two
	debug_assert!(input.len().is_power_of_two(), "length is not a power of two");

	fft(-2.0, input, output);
}

#[inline(always)]
pub fn inverse<CI: Complex, CO: ComplexMut>(input: Stride<CI>, output: MutStride<CO>) {
	// input and output buffers need to be the same length
	debug_assert_eq!(input.len(), output.len());

	// the length has to be a power of two
	debug_assert!(input.len().is_power_of_two(), "length is not a power of two");

	fft(2.0, input, output);
}

#[cfg(test)]
mod tests {
	use num::Complex;
	use strided::{Stride, MutStrided};
	use ::ComplexMut;

	macro_rules! assert_approx_eq {
		($a:expr, $b:expr, $p:expr) => (
			assert_eq!(format!("{:.1$}", $a.re, $p), format!("{:.1$}", $b.re, $p));
			assert_eq!(format!("{:.1$}", $a.im, $p), format!("{:.1$}", $b.im, $p));
		)
	}

	#[test]
	fn forward() {
		let mut output = vec![Complex::new(0.0, 0.0); 4];
		super::forward(Stride::new(&[1.0, 1.0, 0.0, 0.0]), output.as_stride_mut());

		assert_approx_eq!(output[0], Complex::new(2.00,  0.00), 2);
		assert_approx_eq!(output[1], Complex::new(1.00, -1.00), 2);
		assert_approx_eq!(output[2], Complex::new(0.00,  0.00), 2);
		assert_approx_eq!(output[3], Complex::new(1.00,  1.00), 2);
	}

	#[test]
	fn inverse() {
		let mut output = vec![Complex::new(0.0, 0.0); 4];
		super::inverse(Stride::new(&[1.0, 1.0, 0.0, 0.0]), output.as_stride_mut());

		for output in output.iter_mut() {
			ComplexMut::unscale(output, 4.0);
		}

		assert_approx_eq!(output[0], Complex::new(0.50,  0.00), 2);
		assert_approx_eq!(output[1], Complex::new(0.25,  0.25), 2);
		assert_approx_eq!(output[2], Complex::new(0.00,  0.00), 2);
		assert_approx_eq!(output[3], Complex::new(0.25, -0.25), 2);
	}
}
