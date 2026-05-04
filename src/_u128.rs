/// Bytes for a [`u128`], aligned to 1, big-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct U128BE([u8; 16]);
impl U128BE {
  /// Constructs the value from a standard [`u128`].
  #[inline]
  #[must_use]
  pub const fn new(u: u128) -> Self {
    Self(u.to_be_bytes())
  }
  /// Turns the value into a standard [`u128`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> u128 {
    u128::from_be_bytes(self.0)
  }
}
impl From<u128> for U128BE {
  #[inline]
  #[must_use]
  fn from(value: u128) -> Self {
    Self::new(value)
  }
}
impl From<U128BE> for u128 {
  #[inline]
  #[must_use]
  fn from(value: U128BE) -> Self {
    value.get()
  }
}
int_fmt!(U128BE, u128);

/// Bytes for a [`u128`], aligned to 1, little-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct U128LE([u8; 16]);
impl U128LE {
  /// Constructs the value from a standard [`u128`].
  #[inline]
  #[must_use]
  pub const fn new(u: u128) -> Self {
    Self(u.to_le_bytes())
  }
  /// Turns the value into a standard [`u128`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> u128 {
    u128::from_le_bytes(self.0)
  }
}
impl From<u128> for U128LE {
  #[inline]
  #[must_use]
  fn from(value: u128) -> Self {
    Self::new(value)
  }
}
impl From<U128LE> for u128 {
  #[inline]
  #[must_use]
  fn from(value: U128LE) -> Self {
    value.get()
  }
}
int_fmt!(U128LE, u128);

/// Bytes for a [`u128`], aligned to 1, native-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct U128NE([u8; 16]);
impl U128NE {
  /// Constructs the value from a standard [`u128`].
  #[inline]
  #[must_use]
  pub const fn new(u: u128) -> Self {
    Self(u.to_ne_bytes())
  }
  /// Turns the value into a standard [`u128`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> u128 {
    u128::from_ne_bytes(self.0)
  }
}
impl From<u128> for U128NE {
  #[inline]
  #[must_use]
  fn from(value: u128) -> Self {
    Self::new(value)
  }
}
impl From<U128NE> for u128 {
  #[inline]
  #[must_use]
  fn from(value: U128NE) -> Self {
    value.get()
  }
}
int_fmt!(U128NE, u128);
