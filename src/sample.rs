use std::{i8, i16, i32};
use num::{Zero, One};
use {Precision, Complex};

/// Trait for sample data, usually from a music file, used for input.
pub trait Sample: Zero + One + Clone {
	/// Returns a normalized value between `-1.0` and `1.0`.
	fn normalize(&self) -> Precision;
}

/// Trait for mutable sample data, used for output.
pub trait SampleMut: Sample {
	/// Sets a normalized value between `-1.0` and `1.0`.
	fn set_normalized(&mut self, value: Precision);
}

impl Sample for u8 {
	#[inline(always)]
	fn normalize(&self) -> Precision {
		-((*self as Precision - i8::MAX as Precision) / i8::MIN as Precision)
	}
}

impl SampleMut for u8 {
	#[inline(always)]
	fn set_normalized(&mut self, value: Precision) {
		if value >= 0.0 {
			*self = ((value * 128.0) + 127.0) as u8
		}
		else {
			*self = ((value * 128.0) + 128.0) as u8
		}
	}
}

impl Sample for i16 {
	#[inline(always)]
	fn normalize(&self) -> Precision {
		*self as Precision / -(i16::MIN as Precision)
	}
}

impl SampleMut for i16 {
	#[inline(always)]
	fn set_normalized(&mut self, value: Precision) {
		if value >= 0.0 {
			*self = (value as f64 * i16::MAX as f64) as i16;
		}
		else {
			*self = -(value as f64 * i16::MIN as f64) as i16;
		}
	}
}

impl Sample for i32 {
	#[inline(always)]
	fn normalize(&self) -> Precision {
		*self as Precision / -(i32::MIN as Precision)
	}
}

impl SampleMut for i32 {
	#[inline(always)]
	fn set_normalized(&mut self, value: Precision) {
		if value >= 0.0 {
			*self = (value as f64 * i32::MAX as f64) as i32;
		}
		else {
			*self = -(value as f64 * i32::MIN as f64) as i32;
		}
	}
}

impl Sample for f32 {
	#[inline(always)]
	fn normalize(&self) -> Precision {
		*self as Precision
	}
}

impl SampleMut for f32 {
	#[inline(always)]
	fn set_normalized(&mut self, value: Precision) {
		*self = value as f32;
	}
}

impl Sample for f64 {
	#[inline(always)]
	fn normalize(&self) -> Precision {
		*self as Precision
	}
}

impl SampleMut for f64 {
	#[inline(always)]
	fn set_normalized(&mut self, value: Precision) {
		*self = value as f64;
	}
}

impl<S: Sample> Complex for S {
	#[inline(always)]
	fn real(&self) -> Precision {
		self.normalize()
	}

	#[inline(always)]
	fn imag(&self) -> Precision {
		0.0
	}
}

#[cfg(test)]
mod tests {
	use super::{Sample, SampleMut};
	use std::{u8, i16, i32};

	macro_rules! assert_approx_eq {
		($a:expr, $b:expr, $p:expr) => (
			assert_eq!(format!("{:.1$}", $a, $p), format!("{:.1$}", $b, $p));
			assert_eq!(format!("{:.1$}", $a, $p), format!("{:.1$}", $b, $p));
		)
	}

	#[test]
	fn u8() {
		assert_approx_eq!(Sample::normalize(&u8::MAX),  1.00, 2);
		assert_approx_eq!(Sample::normalize(&u8::MIN), -0.99, 2);
	}

	#[test]
	fn u8_mut() {
		let mut v = 0u8;

		SampleMut::set_normalized(&mut v, 1.0);
		assert_eq!(v, u8::MAX);

		SampleMut::set_normalized(&mut v, -1.0);
		assert_eq!(v, u8::MIN);
	}

	#[test]
	fn i16() {
		assert_approx_eq!(Sample::normalize(&i16::MAX),  1.00, 2);
		assert_approx_eq!(Sample::normalize(&i16::MIN), -1.00, 2);
	}

	#[test]
	fn i16_mut() {
		let mut v = 0i16;

		SampleMut::set_normalized(&mut v, 1.0);
		assert_eq!(v, i16::MAX);

		SampleMut::set_normalized(&mut v, -1.0);
		assert_eq!(v, i16::MIN);
	}


	#[test]
	fn i32() {
		assert_approx_eq!(Sample::normalize(&i32::MAX),  1.00, 2);
		assert_approx_eq!(Sample::normalize(&i32::MIN), -1.00, 2);
	}

	#[test]
	fn i32_mut() {
		let mut v = 0i32;

		SampleMut::set_normalized(&mut v, 1.0);
		assert_eq!(v, i32::MAX);

		SampleMut::set_normalized(&mut v, -1.0);
		assert_eq!(v, i32::MIN);
	}
}
