/// Bytes for a [`i64`], aligned to 1, big-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct I64BE([u8; 8]);
impl I64BE {
  /// Constructs the value from a standard [`i64`].
  #[inline]
  #[must_use]
  pub const fn new(u: i64) -> Self {
    Self(u.to_be_bytes())
  }
  /// Turns the value into a standard [`i64`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> i64 {
    i64::from_be_bytes(self.0)
  }
}
impl From<i64> for I64BE {
  #[inline]
  #[must_use]
  fn from(value: i64) -> Self {
    Self::new(value)
  }
}
impl From<I64BE> for i64 {
  #[inline]
  #[must_use]
  fn from(value: I64BE) -> Self {
    value.get()
  }
}
int_fmt!(I64BE, i64);

/// Bytes for a [`i64`], aligned to 1, little-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct I64LE([u8; 8]);
impl I64LE {
  /// Constructs the value from a standard [`i64`].
  #[inline]
  #[must_use]
  pub const fn new(u: i64) -> Self {
    Self(u.to_le_bytes())
  }
  /// Turns the value into a standard [`i64`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> i64 {
    i64::from_le_bytes(self.0)
  }
}
impl From<i64> for I64LE {
  #[inline]
  #[must_use]
  fn from(value: i64) -> Self {
    Self::new(value)
  }
}
impl From<I64LE> for i64 {
  #[inline]
  #[must_use]
  fn from(value: I64LE) -> Self {
    value.get()
  }
}
int_fmt!(I64LE, i64);

/// Bytes for a [`i64`], aligned to 1, native-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct I64NE([u8; 8]);
impl I64NE {
  /// Constructs the value from a standard [`i64`].
  #[inline]
  #[must_use]
  pub const fn new(u: i64) -> Self {
    Self(u.to_ne_bytes())
  }
  /// Turns the value into a standard [`i64`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> i64 {
    i64::from_ne_bytes(self.0)
  }
}
impl From<i64> for I64NE {
  #[inline]
  #[must_use]
  fn from(value: i64) -> Self {
    Self::new(value)
  }
}
impl From<I64NE> for i64 {
  #[inline]
  #[must_use]
  fn from(value: I64NE) -> Self {
    value.get()
  }
}
int_fmt!(I64NE, i64);
