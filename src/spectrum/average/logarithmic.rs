use strided::{Stride, MutStride};

use {Precision};
use spectrum::average::Average;
use spectrum::index_for;

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
				let low    = index_for(f as u32,        input.len(), rate);
				let hig    = index_for(f as u32 + step, input.len(), rate);

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
