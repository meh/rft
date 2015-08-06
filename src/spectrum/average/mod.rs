mod linear;
pub use self::linear::Linear;

mod logarithmic;
pub use self::logarithmic::Logarithmic;

use {Precision};
use strided::{Strided, MutStrided, Stride, MutStride};

pub trait Average {
	type Arguments;

	fn size(args: &Self::Arguments) -> usize;
	fn compute(args: &Self::Arguments, input: Stride<Precision>, mut output: MutStride<Precision>);
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
