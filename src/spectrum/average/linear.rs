use strided::{Stride, MutStride};

use {Precision};
use super::Average;

/// Linear average.
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
