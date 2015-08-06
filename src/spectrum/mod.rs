pub mod average;
pub use self::average::Average;
pub use self::average::compute as average;
pub use self::average::compute_in as average_in;

use {Precision, Complex};
use strided::{Strided, MutStrided};

#[inline(always)]
pub fn compute<C, I>(input: I) -> Vec<Precision>
	where C: Complex,
	      I: Strided<Elem=C>
{
	let mut output = vec![0.0; (input.as_stride().len() / 2) + 1];
	compute_in(input, &mut *output);

	output
}

pub fn compute_in<C, I, O>(input: I, mut output: O)
	where C: Complex,
	      I: Strided<Elem=C>,
	      O: MutStrided<Elem=Precision>
{
	let     input  = input.as_stride();
	let mut output = output.as_stride_mut();

	debug_assert_eq!(output.len(), input.len() / 2 + 1);

	for (input, output) in input.iter().zip(output.iter_mut()) {
		*output = (input.real() * input.real() + input.imag() * input.imag()).sqrt();
	}
}

#[inline]
pub fn band<I>(input: I, mut band: usize) -> Precision
	where I: Strided<Elem=Precision>
{
	let input = input.as_stride();

	if band > input.len() - 1 {
		band = input.len() - 1;
	}

	input[band]
}

#[inline(always)]
pub fn bandwidth(size: usize, rate: u32) -> Precision {
	(2.0 / size as Precision) * (rate as Precision / 2.0)
}

pub fn index_for(frequency: u32, size: usize, rate: u32) -> usize {
	let bandwidth = bandwidth(size, rate);

	if frequency < (bandwidth / 2.0) as u32 {
		return 0;
	}

	if frequency > ((rate as Precision / 2.0) - (bandwidth / 2.0)) as u32 {
		return size / 2;
	}

	(size as Precision * (frequency as Precision / rate as Precision)).round() as usize
}

pub fn frequency_for(index: usize, size: usize, rate: u32) -> u32 {
	let bandwidth = bandwidth(size, rate);

	if index == 0 {
		return (bandwidth * 0.25).round() as u32;
	}

	if index >= size / 2 {
		let last = (rate as Precision / 2.0) - (bandwidth / 2.0);
		let half = bandwidth * 0.25;

		return (last + half).round() as u32;
	}

	(index as Precision * bandwidth as Precision) as u32
}

#[cfg(test)]
mod tests {
	#[test]
	fn bandwidth() {
		assert_eq!(super::bandwidth(1024, 44100), 43.066406);
	}

	#[test]
	fn index_for() {
		assert_eq!(super::index_for(23, 1024, 44_100), 1);
		assert_eq!(super::index_for(40_000, 1024, 44_100), 512);
	}

	#[test]
	fn frequency() {
		assert_eq!(super::frequency_for(0,   1024, 44_100),    11);
		assert_eq!(super::frequency_for(256, 1024, 44_100), 11025);
		assert_eq!(super::frequency_for(512, 1024, 44_100), 22039);
	}
}
