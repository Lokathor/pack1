/// Bytes for a [`i32`], aligned to 1, big-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct I32BE([u8; 4]);
impl I32BE {
  /// Constructs the value from a standard [`i32`].
  #[inline]
  #[must_use]
  pub const fn new(u: i32) -> Self {
    Self(u.to_be_bytes())
  }
  /// Turns the value into a standard [`i32`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> i32 {
    i32::from_be_bytes(self.0)
  }
}
impl From<i32> for I32BE {
  #[inline]
  #[must_use]
  fn from(value: i32) -> Self {
    Self::new(value)
  }
}
impl From<I32BE> for i32 {
  #[inline]
  #[must_use]
  fn from(value: I32BE) -> Self {
    value.get()
  }
}
int_fmt!(I32BE, i32);

/// Bytes for a [`i32`], aligned to 1, little-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct I32LE([u8; 4]);
impl I32LE {
  /// Constructs the value from a standard [`i32`].
  #[inline]
  #[must_use]
  pub const fn new(u: i32) -> Self {
    Self(u.to_le_bytes())
  }
  /// Turns the value into a standard [`i32`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> i32 {
    i32::from_le_bytes(self.0)
  }
}
impl From<i32> for I32LE {
  #[inline]
  #[must_use]
  fn from(value: i32) -> Self {
    Self::new(value)
  }
}
impl From<I32LE> for i32 {
  #[inline]
  #[must_use]
  fn from(value: I32LE) -> Self {
    value.get()
  }
}
int_fmt!(I32LE, i32);

/// Bytes for a [`i32`], aligned to 1, native-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct I32NE([u8; 4]);
impl I32NE {
  /// Constructs the value from a standard [`i32`].
  #[inline]
  #[must_use]
  pub const fn new(u: i32) -> Self {
    Self(u.to_ne_bytes())
  }
  /// Turns the value into a standard [`i32`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> i32 {
    i32::from_ne_bytes(self.0)
  }
}
impl From<i32> for I32NE {
  #[inline]
  #[must_use]
  fn from(value: i32) -> Self {
    Self::new(value)
  }
}
impl From<I32NE> for i32 {
  #[inline]
  #[must_use]
  fn from(value: I32NE) -> Self {
    value.get()
  }
}
int_fmt!(I32NE, i32);
