use std::ops::{Neg, Add, Sub, Mul, Div};

//pub trait PseudoRing: Sized + Neg<Output=Self> + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> {
pub trait PseudoRing: Sized + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> {
  fn zero() -> Self;
  fn mag(self) -> Self;
}

pub trait PseudoField: Sized + PseudoRing + Div<Output=Self> {
  fn one() -> Self;
}

impl PseudoRing for u8 {
  #[inline] fn zero() -> u8 { 0 }
  #[inline] fn mag(self) -> u8 { self }
}

impl PseudoField for u8 {
  #[inline] fn one() -> u8 { 1 }
}

impl PseudoRing for u32 {
  #[inline] fn zero() -> u32 { 0 }
  #[inline] fn mag(self) -> u32 { self }
}

impl PseudoField for u32 {
  #[inline] fn one() -> u32 { 1 }
}

impl PseudoRing for f32 {
  #[inline] fn zero() -> f32 { 0.0 }
  #[inline] fn mag(self) -> f32 { self.abs() }
}

impl PseudoField for f32 {
  #[inline] fn one() -> f32 { 1.0 }
}

impl PseudoRing for f64 {
  #[inline] fn zero() -> f64 { 0.0 }
  #[inline] fn mag(self) -> f64 { self.abs() }
}

impl PseudoField for f64 {
  #[inline] fn one() -> f64 { 1.0 }
}
