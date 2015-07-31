use num::{Zero, One};
use Precision;

pub trait Real: Zero + One + Clone {
	fn get(&self) -> Precision;
}

pub trait RealMut: Real {
	fn set(&mut self, value: Precision);
}

impl Real for u8 {
	fn get(&self) -> Precision {
		(*self as Precision - 128.0) / 128.0
	}
}

impl RealMut for u8 {
	fn set(&mut self, value: Precision) {
		*self = (value * 128.0 + 128.0) as u8;
	}
}

impl Real for i16 {
	fn get(&self) -> Precision {
		*self as Precision * (1.0 / (1 << 15) as Precision)
	}
}

impl RealMut for i16 {
	fn set(&mut self, value: Precision) {
		*self = (value * (1 << 15) as Precision) as i16;
	}
}

impl Real for i32 {
	fn get(&self) -> Precision {
		*self as Precision * (1.0 / (1 << 31) as Precision)
	}
}

impl RealMut for i32 {
	fn set(&mut self, value: Precision) {
		*self = (value * (1 << 31) as Precision) as i32;
	}
}

impl Real for f32 {
	fn get(&self) -> Precision {
		*self as Precision
	}
}

impl RealMut for f32 {
	fn set(&mut self, value: Precision) {
		*self = value as f32
	}
}

impl Real for f64 {
	fn get(&self) -> Precision {
		*self as Precision
	}
}

impl RealMut for f64 {
	fn set(&mut self, value: Precision) {
		*self = value as f64;
	}
}


