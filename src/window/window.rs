use std::ops::{self, Deref, DerefMut};
use num::Zero;
use strided::{Strided, MutStrided};

use super::range::Range;
use {Sample, SampleMut};

/// Represents a cached window function on a range and window size.
#[derive(PartialEq, Clone, Debug)]
pub struct Window<S: SampleMut> {
	range:  ops::Range<u32>,
	buffer: Vec<S>,
}

impl<S0: SampleMut> Window<S0> {
	#[doc(hidden)]
	pub fn new<R: Range>(range: &R, size: usize) -> Self {
		Window {
			range: ops::Range {
				start: range.start().unwrap_or(0),
				end:   range.end().unwrap_or(size as u32),
			},

			buffer: vec![S0::zero(); size],
		}
	}

	/// Applies the cached window on the given input and return a vector with the
	/// applied result.
	#[inline(always)]
	pub fn apply<SO, SI, I>(&self, input: I) -> Vec<SO>
		where SO: SampleMut,
		      SI: Sample,
		      I:  Strided<Elem=SI>,
	{
		let     input  = input.as_stride();
		let mut output = vec![SO::zero(); input.len()];
		let     length = input.len();
	
		// Check the range are valid for the window.
		debug_assert!(self.range.is_valid(length));

		self.apply_in(input, &mut *output);
	
		output
	}

	/// Applies the cached window on the given input putting the result in the
	/// given output.
	pub fn apply_in<SO, SI, I, O>(&self, input: I, mut output: O)
		where SO: SampleMut,
		      SI: Sample,
		      I:  Strided<Elem=SI>,
		      O:  MutStrided<Elem=SO>,
	{
		let     input  = input.as_stride();
		let mut output = output.as_stride_mut();
		let     length = input.len();
	
		// `input` and `output` buffers need to be the same length.
		debug_assert_eq!(input.len(), output.len());
	
		// Check the range are valid for the window.
		debug_assert!(self.range.is_valid(length));
	
		for (index, (input, output)) in input.iter().zip(output.iter_mut()).enumerate() {
			if index >= self.range.start().unwrap_or(0) as usize &&
			   index <= self.range.end().unwrap_or(length as u32) as usize
			{
				output.set_normalized(input.normalize() * self.buffer[index].normalize());
			}
		}
	}
	
	/// Applies the cached window in-place on the given data.
	pub fn apply_on<S, IO>(&self, mut data: IO)
		where S:  SampleMut,
		      IO: MutStrided<Elem=S>,
	{
		let mut data   = data.as_stride_mut();
		let     length = data.len();
	
		// Check the range are valid for the window.
		debug_assert!(self.range.is_valid(length));
	
		for (index, datum) in data.iter_mut().enumerate() {
			if index >= self.range.start().unwrap_or(0) as usize &&
			   index <= self.range.end().unwrap_or(length as u32) as usize
			{
				let value = datum.normalize();
	
				datum.set_normalized(value * self.buffer[index].normalize());
			}
		}
	}
}

impl<S: SampleMut> Deref for Window<S> {
	type Target = Vec<S>;

	fn deref(&self) -> &Vec<S> {
		&self.buffer
	}
}

impl<S: SampleMut> DerefMut for Window<S> {
	fn deref_mut(&mut self) -> &mut Vec<S> {
		&mut self.buffer
	}
}
