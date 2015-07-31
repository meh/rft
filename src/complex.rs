use num::{self, Zero, One};
use {Precision, Real};

pub trait Complex: Zero + One {
	fn real(&self) -> Precision;
	fn imag(&self) -> Precision;

	#[inline]
	fn exp(&self) -> num::Complex<Precision> {
		num::Complex::new(
			self.real().exp() * self.imag().cos(),
			self.real().exp() * self.imag().sin())
	}

	#[inline]
	fn add(&self, value: Precision) -> num::Complex<Precision> {
		num::Complex::new(self.real() + value, self.imag())
	}

	#[inline]
	fn sub(&self, value: Precision) -> num::Complex<Precision> {
		num::Complex::new(self.real() - value, self.imag())
	}

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
}

impl Complex for num::Complex<f32> {
	fn real(&self) -> Precision {
		self.re as Precision
	}

	fn imag(&self) -> Precision {
		self.im as Precision
	}
}

impl Complex for num::Complex<f64> {
	fn real(&self) -> Precision {
		self.re as Precision
	}

	fn imag(&self) -> Precision {
		self.im as Precision
	}
}

impl ComplexMut for num::Complex<f32> {
	fn set_real(&mut self, value: Precision) {
		self.re = value as f32;
	}

	fn set_imag(&mut self, value: Precision) {
		self.im = value as f32;
	}
}

impl ComplexMut for num::Complex<f64> {
	fn set_real(&mut self, value: Precision) {
		self.re = value as f64;
	}

	fn set_imag(&mut self, value: Precision) {
		self.im = value as f64;
	}
}

impl<R: Real> Complex for R {
	fn real(&self) -> Precision {
		self.get()
	}

	fn imag(&self) -> Precision {
		0.0
	}
}
