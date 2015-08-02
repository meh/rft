use num::{Zero, One};
use Precision;

pub trait Real: Zero + One + Clone {
	fn normalize(&self) -> Precision;
}

impl Real for u8 {
	fn normalize(&self) -> Precision {
		(*self as Precision - 128.0) / 128.0
	}
}

impl Real for i16 {
	fn normalize(&self) -> Precision {
		*self as Precision * (1.0 / (1 << 15) as Precision)
	}
}

impl Real for i32 {
	fn normalize(&self) -> Precision {
		*self as Precision * (1.0 / (1 << 31) as Precision)
	}
}

impl Real for f32 {
	fn normalize(&self) -> Precision {
		*self as Precision
	}
}

impl Real for f64 {
	fn normalize(&self) -> Precision {
		*self as Precision
	}
}
