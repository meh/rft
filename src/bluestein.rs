use std::cmp;
use std::f64::consts::PI;
use num::{self, Zero};
use strided::{Strided, MutStrided, Stride, MutStride};

use {Precision, Complex, ComplexMut};
use cooley_tukey as ct;

fn fft<CI: Complex, CO: ComplexMut>(direction: Precision, input: Stride<CI>, mut output: MutStride<CO>) {
	let     length = input.len();
	let mut next   = 1;

	// find a power-of-2 convultion such as next >= length * 2 + 1
	while next < length * 2 + 1 {
		next <<= 1;
	}

	// exponent table
	let mut t = Vec::with_capacity(length);
	for i in 0 .. length {
		t.push(num::Complex::from_polar(&1.0,
			&(direction * PI as Precision * (i * i % (length * 2)) as Precision / length as Precision)));
	}

	let mut a = Vec::with_capacity(next);
	let mut b = Vec::with_capacity(next);

	for i in 0 .. next {
		if i < length {
			a.push(input[i].to_num() * t[i]);
		}
		else {
			a.push(num::Complex::new(0.0, 0.0));
		}

		if i < length || next - i < length {
			b.push(t[cmp::min(i, next - i)].conj());
		}
		else {
			b.push(num::Complex::new(0.0, 0.0));
		}
	}

	// do the convultion
	convolve(&mut a, &mut b, output.reborrow());

	// postprocessing
	for (output, exp) in output.iter_mut().zip(t.iter()) {
		output.mul(exp);
	}
}

fn convolve<CO: ComplexMut>(x: &mut [num::Complex<Precision>], y: &mut [num::Complex<Precision>], mut output: MutStride<CO>) {
	// cache the length
	let length = x.len();
	
	// temporary buffer for the juggling
	let mut tmp = vec![num::Complex::<Precision>::zero(); length];

	// forward FFT on y (tmp)
	ct::forward(y.as_stride(), tmp.as_stride_mut());

	// forward FFT on x (y)
	ct::forward(x.as_stride(), y.as_stride_mut());

	// multiply x (y) with y (tmp)
	for i in 0 .. length {
		y[i].mul(&tmp[i]);
	}

	// inverse FFT on x (y)
	ct::inverse(y.as_stride(), x.as_stride_mut());

	// scale and set the output
	for (i, output) in output.iter_mut().enumerate() {
		output.set(&x[i].unscale(length as Precision));
	}
}

pub fn forward<CI: Complex, CO: ComplexMut>(input: Stride<CI>, output: MutStride<CO>) {
	// input and output buffers need to be the same length
	assert_eq!(input.len(), output.len());

	fft(-1.0, input, output);
}

pub fn inverse<CI: Complex, CO: ComplexMut>(input: Stride<CI>, output: MutStride<CO>) {
	// input and output buffers need to be the same length
	assert_eq!(input.len(), output.len());

	fft(1.0, input, output);
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
		let mut output = vec![Complex::new(0.0, 0.0); 5];
		super::forward(Stride::new(&[1.0, 1.0, 0.0, 0.0, 0.5]), output.as_stride_mut());

		assert_approx_eq!(output[0], Complex::new( 2.50, -0.001), 2);
		assert_approx_eq!(output[1], Complex::new( 1.46, -0.48 ), 2);
		assert_approx_eq!(output[2], Complex::new(-0.21, -0.29 ), 2);
		assert_approx_eq!(output[3], Complex::new(-0.21,  0.29 ), 2);
		assert_approx_eq!(output[4], Complex::new( 1.46,  0.48 ), 2);
	}

	#[test]
	fn inverse() {
		let mut output = vec![Complex::new(0.0, 0.0); 5];
		super::inverse(Stride::new(&[1.0, 1.0, 0.0, 0.0, 0.5]), output.as_stride_mut());

		for output in output.iter_mut() {
			ComplexMut::unscale(output, 5.0);
		}

		assert_approx_eq!(output[0], Complex::new( 0.50, -0.001), 2);
		assert_approx_eq!(output[1], Complex::new( 0.29,  0.10 ), 2);
		assert_approx_eq!(output[2], Complex::new(-0.04,  0.06 ), 2);
		assert_approx_eq!(output[3], Complex::new(-0.04, -0.06 ), 2);
		assert_approx_eq!(output[4], Complex::new( 0.29, -0.10 ), 2);
	}
}
