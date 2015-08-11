use std::ops;

/// Trait for a window range argument.
pub trait Range: Clone {
	/// The start of the range, if any.
	fn start(&self) -> Option<u32> {
		None
	}

	/// The end of the range, if any.
	fn end(&self) -> Option<u32> {
		None
	}

	/// The total width of the range.
	fn width(&self, size: usize) -> u32 {
		self.end().unwrap_or(size as u32) - self.start().unwrap_or(0)
	}

	/// Checks if the range is valid for the given size.
	fn is_valid(&self, size: usize) -> bool {
		if let Some(end) = self.end() {
			return end as usize <= size;
		}

		true
	}
}

impl Range for ops::Range<u32> {
	fn start(&self) -> Option<u32> {
		Some(self.start)
	}

	fn end(&self) -> Option<u32> {
		Some(self.end)
	}
}

impl Range for ops::RangeTo<u32> {
	fn end(&self) -> Option<u32> {
		Some(self.end)
	}
}

impl Range for ops::RangeFrom<u32> {
	fn start(&self) -> Option<u32> {
		Some(self.start)
	}
}

impl Range for ops::RangeFull { }
