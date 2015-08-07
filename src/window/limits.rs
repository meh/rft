use std::ops::{Range, RangeTo, RangeFrom, RangeFull};

pub trait Limits: Clone {
	fn start(&self) -> Option<u32> {
		None
	}

	fn end(&self) -> Option<u32> {
		None
	}

	fn width(&self, size: usize) -> u32 {
		self.end().unwrap_or(size as u32) - self.start().unwrap_or(0)
	}

	fn is_valid(&self, size: usize) -> bool {
		if let Some(end) = self.end() {
			return end as usize <= size;
		}

		true
	}
}

impl Limits for Range<u32> {
	fn start(&self) -> Option<u32> {
		Some(self.start)
	}

	fn end(&self) -> Option<u32> {
		Some(self.end)
	}
}

impl Limits for RangeTo<u32> {
	fn end(&self) -> Option<u32> {
		Some(self.end)
	}
}

impl Limits for RangeFrom<u32> {
	fn start(&self) -> Option<u32> {
		Some(self.start)
	}
}

impl Limits for RangeFull { }
