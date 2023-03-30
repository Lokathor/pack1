/// Bytes for a [`i16`], aligned to 1, big-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct I16BE([u8; 2]);
impl I16BE {
  /// Constructs the value from a standard [`i16`].
  #[inline]
  #[must_use]
  pub const fn new(u: i16) -> Self {
    Self(u.to_be_bytes())
  }
  /// Turns the value into a standard [`i16`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> i16 {
    i16::from_be_bytes(self.0)
  }
}
impl From<i16> for I16BE {
  #[inline]
  #[must_use]
  fn from(value: i16) -> Self {
    Self::new(value)
  }
}
impl From<I16BE> for i16 {
  #[inline]
  #[must_use]
  fn from(value: I16BE) -> Self {
    value.get()
  }
}
int_fmt!(I16BE, i16);

/// Bytes for a [`i16`], aligned to 1, little-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct I16LE([u8; 2]);
impl I16LE {
  /// Constructs the value from a standard [`i16`].
  #[inline]
  #[must_use]
  pub const fn new(u: i16) -> Self {
    Self(u.to_le_bytes())
  }
  /// Turns the value into a standard [`i16`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> i16 {
    i16::from_le_bytes(self.0)
  }
}
impl From<i16> for I16LE {
  #[inline]
  #[must_use]
  fn from(value: i16) -> Self {
    Self::new(value)
  }
}
impl From<I16LE> for i16 {
  #[inline]
  #[must_use]
  fn from(value: I16LE) -> Self {
    value.get()
  }
}
int_fmt!(I16LE, i16);

/// Bytes for a [`i16`], aligned to 1, native-endian.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct I16NE([u8; 2]);
impl I16NE {
  /// Constructs the value from a standard [`i16`].
  #[inline]
  #[must_use]
  pub const fn new(u: i16) -> Self {
    Self(u.to_ne_bytes())
  }
  /// Turns the value into a standard [`i16`].
  #[inline]
  #[must_use]
  pub const fn get(self) -> i16 {
    i16::from_ne_bytes(self.0)
  }
}
impl From<i16> for I16NE {
  #[inline]
  #[must_use]
  fn from(value: i16) -> Self {
    Self::new(value)
  }
}
impl From<I16NE> for i16 {
  #[inline]
  #[must_use]
  fn from(value: I16NE) -> Self {
    value.get()
  }
}
int_fmt!(I16NE, i16);
