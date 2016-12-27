use std::ops::{Neg, Add, Sub, Mul, Div};

pub trait Ring: Sized + Neg<Output=Self> + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> {
  fn zero() -> Self;
}

pub trait Field: Sized + Ring + Div<Output=Self> {
  fn one() -> Self;
}

impl Ring for f32 {
  #[inline]
  fn zero() -> f32 {
    0.0
  }
}

impl Ring for f64 {
  #[inline]
  fn zero() -> f64 {
    0.0
  }
}

impl Field for f32 {
  #[inline]
  fn one() -> f32 {
    1.0
  }
}

impl Field for f64 {
  #[inline]
  fn one() -> f64 {
    1.0
  }
}
