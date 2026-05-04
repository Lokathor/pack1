/// Bytes for a [`i128`], aligned to 1, big-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct I128BE([u8; 16]);
impl I128BE {
  /// Constructs the value from a standard [`i128`].
  #[inline]
  #[must_use]
  pub const fn new(u: i128) -> Self {
    Self(u.to_be_bytes())
  }
  /// Turns the value into a standard [`i128`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> i128 {
    i128::from_be_bytes(self.0)
  }
}
impl From<i128> for I128BE {
  #[inline]
  #[must_use]
  fn from(value: i128) -> Self {
    Self::new(value)
  }
}
impl From<I128BE> for i128 {
  #[inline]
  #[must_use]
  fn from(value: I128BE) -> Self {
    value.get()
  }
}
int_fmt!(I128BE, i128);

/// Bytes for a [`i128`], aligned to 1, little-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct I128LE([u8; 16]);
impl I128LE {
  /// Constructs the value from a standard [`i128`].
  #[inline]
  #[must_use]
  pub const fn new(u: i128) -> Self {
    Self(u.to_le_bytes())
  }
  /// Turns the value into a standard [`i128`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> i128 {
    i128::from_le_bytes(self.0)
  }
}
impl From<i128> for I128LE {
  #[inline]
  #[must_use]
  fn from(value: i128) -> Self {
    Self::new(value)
  }
}
impl From<I128LE> for i128 {
  #[inline]
  #[must_use]
  fn from(value: I128LE) -> Self {
    value.get()
  }
}
int_fmt!(I128LE, i128);

/// Bytes for a [`i128`], aligned to 1, native-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct I128NE([u8; 16]);
impl I128NE {
  /// Constructs the value from a standard [`i128`].
  #[inline]
  #[must_use]
  pub const fn new(u: i128) -> Self {
    Self(u.to_ne_bytes())
  }
  /// Turns the value into a standard [`i128`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> i128 {
    i128::from_ne_bytes(self.0)
  }
}
impl From<i128> for I128NE {
  #[inline]
  #[must_use]
  fn from(value: i128) -> Self {
    Self::new(value)
  }
}
impl From<I128NE> for i128 {
  #[inline]
  #[must_use]
  fn from(value: I128NE) -> Self {
    value.get()
  }
}
int_fmt!(I128NE, i128);
