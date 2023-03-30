/// Bytes for a [`u64`], aligned to 1, big-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct U64BE([u8; 8]);
impl U64BE {
  /// Constructs the value from a standard [`u64`].
  #[inline]
  #[must_use]
  pub const fn new(u: u64) -> Self {
    Self(u.to_be_bytes())
  }
  /// Turns the value into a standard [`u64`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> u64 {
    u64::from_be_bytes(self.0)
  }
}
impl From<u64> for U64BE {
  #[inline]
  #[must_use]
  fn from(value: u64) -> Self {
    Self::new(value)
  }
}
impl From<U64BE> for u64 {
  #[inline]
  #[must_use]
  fn from(value: U64BE) -> Self {
    value.get()
  }
}
int_fmt!(U64BE, u64);

/// Bytes for a [`u64`], aligned to 1, little-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct U64LE([u8; 8]);
impl U64LE {
  /// Constructs the value from a standard [`u64`].
  #[inline]
  #[must_use]
  pub const fn new(u: u64) -> Self {
    Self(u.to_le_bytes())
  }
  /// Turns the value into a standard [`u64`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> u64 {
    u64::from_le_bytes(self.0)
  }
}
impl From<u64> for U64LE {
  #[inline]
  #[must_use]
  fn from(value: u64) -> Self {
    Self::new(value)
  }
}
impl From<U64LE> for u64 {
  #[inline]
  #[must_use]
  fn from(value: U64LE) -> Self {
    value.get()
  }
}
int_fmt!(U64LE, u64);

/// Bytes for a [`u64`], aligned to 1, native-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct U64NE([u8; 8]);
impl U64NE {
  /// Constructs the value from a standard [`u64`].
  #[inline]
  #[must_use]
  pub const fn new(u: u64) -> Self {
    Self(u.to_ne_bytes())
  }
  /// Turns the value into a standard [`u64`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> u64 {
    u64::from_ne_bytes(self.0)
  }
}
impl From<u64> for U64NE {
  #[inline]
  #[must_use]
  fn from(value: u64) -> Self {
    Self::new(value)
  }
}
impl From<U64NE> for u64 {
  #[inline]
  #[must_use]
  fn from(value: U64NE) -> Self {
    value.get()
  }
}
int_fmt!(U64NE, u64);
