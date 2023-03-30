/// Bytes for a [`f32`], aligned to 1, big-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct F32BE([u8; 4]);
impl F32BE {
  /// Constructs the value from a standard [`f32`].
  #[inline]
  #[must_use]
  pub fn new(u: f32) -> Self {
    Self(u.to_be_bytes())
  }
  /// Turns the value into a standard [`f32`].
  #[inline]
  #[must_use]
  pub fn get(self) -> f32 {
    f32::from_be_bytes(self.0)
  }
}
impl From<f32> for F32BE {
  #[inline]
  #[must_use]
  fn from(value: f32) -> Self {
    Self::new(value)
  }
}
impl From<F32BE> for f32 {
  #[inline]
  #[must_use]
  fn from(value: F32BE) -> Self {
    value.get()
  }
}
float_fmt!(F32BE, f32);

/// Bytes for a [`f32`], aligned to 1, little-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct F32LE([u8; 4]);
impl F32LE {
  /// Constructs the value from a standard [`f32`].
  #[inline]
  #[must_use]
  pub fn new(u: f32) -> Self {
    Self(u.to_le_bytes())
  }
  /// Turns the value into a standard [`f32`].
  #[inline]
  #[must_use]
  pub fn get(self) -> f32 {
    f32::from_le_bytes(self.0)
  }
}
impl From<f32> for F32LE {
  #[inline]
  #[must_use]
  fn from(value: f32) -> Self {
    Self::new(value)
  }
}
impl From<F32LE> for f32 {
  #[inline]
  #[must_use]
  fn from(value: F32LE) -> Self {
    value.get()
  }
}
float_fmt!(F32LE, f32);

/// Bytes for a [`f32`], aligned to 1, native-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct F32NE([u8; 4]);
impl F32NE {
  /// Constructs the value from a standard [`f32`].
  #[inline]
  #[must_use]
  pub fn new(u: f32) -> Self {
    Self(u.to_ne_bytes())
  }
  /// Turns the value into a standard [`f32`].
  #[inline]
  #[must_use]
  pub fn get(self) -> f32 {
    f32::from_ne_bytes(self.0)
  }
}
impl From<f32> for F32NE {
  #[inline]
  #[must_use]
  fn from(value: f32) -> Self {
    Self::new(value)
  }
}
impl From<F32NE> for f32 {
  #[inline]
  #[must_use]
  fn from(value: F32NE) -> Self {
    value.get()
  }
}
float_fmt!(F32NE, f32);
