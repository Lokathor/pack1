/// Bytes for a [`u16`], aligned to 1, big-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct U16BE([u8; 2]);
impl U16BE {
  /// Constructs the value from a standard [`u16`].
  #[inline]
  #[must_use]
  pub const fn new(u: u16) -> Self {
    Self(u.to_be_bytes())
  }
  /// Turns the value into a standard [`u16`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> u16 {
    u16::from_be_bytes(self.0)
  }
}
impl From<u16> for U16BE {
  #[inline]
  #[must_use]
  fn from(value: u16) -> Self {
    Self::new(value)
  }
}
impl From<U16BE> for u16 {
  #[inline]
  #[must_use]
  fn from(value: U16BE) -> Self {
    value.get()
  }
}
int_fmt!(U16BE, u16);

/// Bytes for a [`u16`], aligned to 1, little-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct U16LE([u8; 2]);
impl U16LE {
  /// Constructs the value from a standard [`u16`].
  #[inline]
  #[must_use]
  pub const fn new(u: u16) -> Self {
    Self(u.to_le_bytes())
  }
  /// Turns the value into a standard [`u16`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> u16 {
    u16::from_le_bytes(self.0)
  }
}
impl From<u16> for U16LE {
  #[inline]
  #[must_use]
  fn from(value: u16) -> Self {
    Self::new(value)
  }
}
impl From<U16LE> for u16 {
  #[inline]
  #[must_use]
  fn from(value: U16LE) -> Self {
    value.get()
  }
}
int_fmt!(U16LE, u16);

/// Bytes for a [`u16`], aligned to 1, native-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct U16NE([u8; 2]);
impl U16NE {
  /// Constructs the value from a standard [`u16`].
  #[inline]
  #[must_use]
  pub const fn new(u: u16) -> Self {
    Self(u.to_ne_bytes())
  }
  /// Turns the value into a standard [`u16`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> u16 {
    u16::from_ne_bytes(self.0)
  }
}
impl From<u16> for U16NE {
  #[inline]
  #[must_use]
  fn from(value: u16) -> Self {
    Self::new(value)
  }
}
impl From<U16NE> for u16 {
  #[inline]
  #[must_use]
  fn from(value: U16NE) -> Self {
    value.get()
  }
}
int_fmt!(U16NE, u16);
