use num::{self, Zero, One};
use Precision;

pub trait Complex: Zero + One + Clone {
	fn real(&self) -> Precision;
	fn imag(&self) -> Precision;

	#[inline]
	fn to_num(&self) -> num::Complex<Precision> {
		num::Complex::new(self.real(), self.imag())
	}

	#[inline]
	fn to_real(&self) -> Precision {
		self.to_num().norm()
	}
}

pub trait ComplexMut: Complex {
	fn set_real(&mut self, value: Precision);
	fn set_imag(&mut self, value: Precision);

	#[inline]
	fn set<C: Complex>(&mut self, value: &C) {
		self.set_real(value.real());
		self.set_imag(value.imag());
	}

	#[inline]
	fn mul<C: Complex>(&mut self, value: &C) {
		let real = self.real();
		let imag = self.imag();

		self.set_real(real * value.real() - imag * value.imag());
		self.set_imag(real * value.imag() + imag * value.real());
	}

	#[inline]
	fn scale(&mut self, value: Precision) {
		let real = self.real();
		let imag = self.imag();

		self.set_real(real * value);
		self.set_imag(imag * value);
	}

	#[inline]
	fn div<C: Complex>(&mut self, value: &C) {
		let real = self.real();
		let imag = self.imag();
		let sqr  = self.real() * self.real() + self.imag() * self.imag();

		self.set_real((real * value.real() + imag * value.imag()) / sqr);
		self.set_imag((imag * value.real() - real * value.imag()) / sqr);
	}

	#[inline]
	fn unscale(&mut self, value: Precision) {
		let real = self.real();
		let imag = self.imag();

		self.set_real(real / value);
		self.set_imag(imag / value);
	}
}

impl Complex for num::Complex<f32> {
	#[inline(always)]
	fn real(&self) -> Precision {
		self.re as Precision
	}

	#[inline(always)]
	fn imag(&self) -> Precision {
		self.im as Precision
	}
}

impl Complex for num::Complex<f64> {
	#[inline(always)]
	fn real(&self) -> Precision {
		self.re as Precision
	}

	#[inline(always)]
	fn imag(&self) -> Precision {
		self.im as Precision
	}
}

impl ComplexMut for num::Complex<f32> {
	#[inline(always)]
	fn set_real(&mut self, value: Precision) {
		self.re = value as f32;
	}

	#[inline(always)]
	fn set_imag(&mut self, value: Precision) {
		self.im = value as f32;
	}
}

impl ComplexMut for num::Complex<f64> {
	#[inline(always)]
	fn set_real(&mut self, value: Precision) {
		self.re = value as f64;
	}

	#[inline(always)]
	fn set_imag(&mut self, value: Precision) {
		self.im = value as f64;
	}
}

#[cfg(test)]
mod tests {
	use super::Complex;
	use std::{u8, i16, i32};

	macro_rules! assert_approx_eq {
		($a:expr, $r:expr, $i:expr, $p:expr) => (
			assert_eq!(format!("{:.1$}", $a.real(), $p), format!("{:.1$}", $r, $p));
			assert_eq!(format!("{:.1$}", $a.imag(), $p), format!("{:.1$}", $i, $p));
		)
	}

	#[test]
	fn u8() {
		assert_approx_eq!(u8::MAX,  1.00, 0.00, 2);
		assert_approx_eq!(u8::MIN, -0.99, 0.00, 2);
	}

	#[test]
	fn i16() {
		assert_approx_eq!(i16::MAX,  1.00, 0.00, 2);
		assert_approx_eq!(i16::MIN, -1.00, 0.00, 2);
	}

	#[test]
	fn i32() {
		assert_approx_eq!(i32::MAX,  1.00, 0.00, 2);
		assert_approx_eq!(i32::MIN, -1.00, 0.00, 2);
	}
}
