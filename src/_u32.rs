/// Bytes for a [`u32`], aligned to 1, big-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct U32BE([u8; 4]);
impl U32BE {
  /// Constructs the value from a standard [`u32`].
  #[inline]
  #[must_use]
  pub const fn new(u: u32) -> Self {
    Self(u.to_be_bytes())
  }
  /// Turns the value into a standard [`u32`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> u32 {
    u32::from_be_bytes(self.0)
  }
}
impl From<u32> for U32BE {
  #[inline]
  #[must_use]
  fn from(value: u32) -> Self {
    Self::new(value)
  }
}
impl From<U32BE> for u32 {
  #[inline]
  #[must_use]
  fn from(value: U32BE) -> Self {
    value.get()
  }
}
int_fmt!(U32BE, u32);

/// Bytes for a [`u32`], aligned to 1, little-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct U32LE([u8; 4]);
impl U32LE {
  /// Constructs the value from a standard [`u32`].
  #[inline]
  #[must_use]
  pub const fn new(u: u32) -> Self {
    Self(u.to_le_bytes())
  }
  /// Turns the value into a standard [`u32`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> u32 {
    u32::from_le_bytes(self.0)
  }
}
impl From<u32> for U32LE {
  #[inline]
  #[must_use]
  fn from(value: u32) -> Self {
    Self::new(value)
  }
}
impl From<U32LE> for u32 {
  #[inline]
  #[must_use]
  fn from(value: U32LE) -> Self {
    value.get()
  }
}
int_fmt!(U32LE, u32);

/// Bytes for a [`u32`], aligned to 1, native-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct U32NE([u8; 4]);
impl U32NE {
  /// Constructs the value from a standard [`u32`].
  #[inline]
  #[must_use]
  pub const fn new(u: u32) -> Self {
    Self(u.to_ne_bytes())
  }
  /// Turns the value into a standard [`u32`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> u32 {
    u32::from_ne_bytes(self.0)
  }
}
impl From<u32> for U32NE {
  #[inline]
  #[must_use]
  fn from(value: u32) -> Self {
    Self::new(value)
  }
}
impl From<U32NE> for u32 {
  #[inline]
  #[must_use]
  fn from(value: U32NE) -> Self {
    value.get()
  }
}
int_fmt!(U32NE, u32);
