extern crate num;

extern crate strided;

#[cfg(feature = "f32")]
pub type Precision = f32;
#[cfg(feature = "f64")]
pub type Precision = f64;

mod real;
pub use real::{Real, RealMut};

mod complex;
pub use complex::{Complex, ComplexMut};
