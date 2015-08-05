use {Precision};
use strided::{Strided, MutStrided, Stride, MutStride};

pub trait Average {
	type Arguments;

	fn size(args: &Self::Arguments) -> usize;
	fn compute(args: &Self::Arguments, input: Stride<Precision>, mut output: MutStride<Precision>);
}

pub struct Linear;

impl Average for Linear {
	type Arguments = usize;

	#[inline]
	fn size(&amount: &Self::Arguments) -> usize {
		amount
	}

	fn compute(&amount: &Self::Arguments, input: Stride<Precision>, mut output: MutStride<Precision>) {
		debug_assert!(amount <= input.len() / 2);

		let width = input.len() / amount;

		for i in 0 .. amount {
			let mut average = 0.0;
			let mut j       = 0;

			while j < width {
				let offset = j + i * width;

				if offset >= output.len() {
					break;
				}

				average += input[offset];
				j       += 1;
			}

			output[i] = average / j as Precision;
		}
	}
}

pub struct Logarithmic;

impl Average for Logarithmic {
	type Arguments = (u32, u32, u32);

	fn size(&(rate, min, bands): &Self::Arguments) -> usize {
		let mut nyquist = rate / 2;
		let mut octaves = 1;

		while nyquist / 2 > min {
			octaves += 1;
			nyquist /= 2;
		}

		(octaves * bands) as usize
	}

	fn compute(&(rate, min, bands): &Self::Arguments, input: Stride<Precision>, mut output: MutStride<Precision>) {
		let mut nyquist = rate / 2;
		let mut octaves = 1u32;

		while nyquist / 2 < min {
			octaves += 1;
			nyquist /= 2;
		}

		for i in 0 .. octaves {
			let low = if i == 0 {
				0
			}
			else {
				(rate / 2) / (octaves - i).pow(2)
			};

			let hig  = (rate / 2) / (octaves - i - 1).pow(2);
			let step = (hig - low) / bands;

			let mut f = low;

			for j in 0 .. bands {
				let offset = j + i * bands;
				let low    = super::index_for(f as u32,        input.len(), rate);
				let hig    = super::index_for(f as u32 + step, input.len(), rate);

				let mut average = 0.0;

				for i in low .. hig {
					average += input[i];
				}

				output[offset as usize]  = average / (hig - low + 1) as Precision;
				f                       += step;
			}
		}
	}
}

#[inline]
pub fn size<A>(args: &A::Arguments) -> usize
	where A: Average
{
	A::size(args)
}

#[inline]
pub fn compute<A, I>(args: &A::Arguments, input: I) -> Vec<Precision>
	where A: Average,
	      I: Strided<Elem=Precision>
{
	let mut output = vec![0.0; A::size(args)];
	A::compute(args, input.as_stride(), output.as_stride_mut());

	output
}

#[inline]
pub fn compute_in<A, I, O>(args: &A::Arguments, input: I, mut output: O)
	where A: Average,
	      I: Strided<Elem=Precision>,
	      O: MutStrided<Elem=Precision>
{
	A::compute(args, input.as_stride(), output.as_stride_mut());
}

#[cfg(test)]
mod tests {
	use super::{Linear, Logarithmic};

	#[test]
	fn size() {
		assert_eq!(super::size::<Linear>(&3), 3);
		assert_eq!(super::size::<Logarithmic>(&(44100, 11, 1)), 11);
	}
}
