/// Bytes for a [`f64`], aligned to 1, big-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct F64BE([u8; 8]);
impl F64BE {
  /// Constructs the value from a standard [`f64`].
  #[inline]
  #[must_use]
  pub fn new(u: f64) -> Self {
    Self(u.to_be_bytes())
  }
  /// Turns the value into a standard [`f64`].
  #[inline]
  #[must_use]
  pub fn get(self) -> f64 {
    f64::from_be_bytes(self.0)
  }
}
impl From<f64> for F64BE {
  #[inline]
  #[must_use]
  fn from(value: f64) -> Self {
    Self::new(value)
  }
}
impl From<F64BE> for f64 {
  #[inline]
  #[must_use]
  fn from(value: F64BE) -> Self {
    value.get()
  }
}
float_fmt!(F64BE, f64);

/// Bytes for a [`f64`], aligned to 1, little-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct F64LE([u8; 8]);
impl F64LE {
  /// Constructs the value from a standard [`f64`].
  #[inline]
  #[must_use]
  pub fn new(u: f64) -> Self {
    Self(u.to_le_bytes())
  }
  /// Turns the value into a standard [`f64`].
  #[inline]
  #[must_use]
  pub fn get(self) -> f64 {
    f64::from_le_bytes(self.0)
  }
}
impl From<f64> for F64LE {
  #[inline]
  #[must_use]
  fn from(value: f64) -> Self {
    Self::new(value)
  }
}
impl From<F64LE> for f64 {
  #[inline]
  #[must_use]
  fn from(value: F64LE) -> Self {
    value.get()
  }
}
float_fmt!(F64LE, f64);

/// Bytes for a [`f64`], aligned to 1, native-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct F64NE([u8; 8]);
impl F64NE {
  /// Constructs the value from a standard [`f64`].
  #[inline]
  #[must_use]
  pub fn new(u: f64) -> Self {
    Self(u.to_ne_bytes())
  }
  /// Turns the value into a standard [`f64`].
  #[inline]
  #[must_use]
  pub fn get(self) -> f64 {
    f64::from_ne_bytes(self.0)
  }
}
impl From<f64> for F64NE {
  #[inline]
  #[must_use]
  fn from(value: f64) -> Self {
    Self::new(value)
  }
}
impl From<F64NE> for f64 {
  #[inline]
  #[must_use]
  fn from(value: F64NE) -> Self {
    value.get()
  }
}
float_fmt!(F64NE, f64);
