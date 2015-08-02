use num::{Zero, One};
use {Precision, Complex};

pub trait Sample: Zero + One + Clone {
	fn normalize(&self) -> Precision;
}

impl Sample for u8 {
	fn normalize(&self) -> Precision {
		(*self as Precision - 128.0) / 128.0
	}
}

impl Sample for i16 {
	fn normalize(&self) -> Precision {
		*self as Precision * (1.0 / (1 << 15) as Precision)
	}
}

impl Sample for i32 {
	fn normalize(&self) -> Precision {
		*self as Precision * (1.0 / (1 << 31) as Precision)
	}
}

impl Sample for f32 {
	fn normalize(&self) -> Precision {
		*self as Precision
	}
}

impl Sample for f64 {
	fn normalize(&self) -> Precision {
		*self as Precision
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
