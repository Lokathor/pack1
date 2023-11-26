#![no_std]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::missing_inline_in_public_items)]

//! Byte array newtypes for different primitive types.
//!
//! Because they're newtypes of byte arrays, they always have alignment 1.
//!
//! Each type has `new` and `get` functions, as well as [`From`] impls. The
//! `new` and `get` functions are `const fn` with int types, but not yet with
//! floating types.
//!
//! The intended usage of this crate is that you can use these these types in a
//! `repr(C)` struct, along with manual padding, and then have each field at the
//! exact byte offset you desire (and with the necessary endian-ness), without
//! the normal `repr(packed)` problem that it interferes with references.

macro_rules! int_fmt {
  ($i:ident, $base:ty) => {
    impl core::fmt::Binary for $i {
      #[inline]
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Binary::fmt(&<$base>::from(*self), f)
      }
    }
    impl core::fmt::Debug for $i {
      #[inline]
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&<$base>::from(*self), f)
      }
    }
    impl core::fmt::Display for $i {
      #[inline]
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&<$base>::from(*self), f)
      }
    }
    impl core::fmt::LowerExp for $i {
      #[inline]
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::LowerExp::fmt(&<$base>::from(*self), f)
      }
    }
    impl core::fmt::LowerHex for $i {
      #[inline]
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::LowerHex::fmt(&<$base>::from(*self), f)
      }
    }
    impl core::fmt::Octal for $i {
      #[inline]
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Octal::fmt(&<$base>::from(*self), f)
      }
    }
    impl core::fmt::UpperExp for $i {
      #[inline]
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::UpperExp::fmt(&<$base>::from(*self), f)
      }
    }
    impl core::fmt::UpperHex for $i {
      #[inline]
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::UpperHex::fmt(&<$base>::from(*self), f)
      }
    }
  };
}

macro_rules! float_fmt {
  ($i:ident, $base:ty) => {
    impl core::fmt::Debug for $i {
      #[inline]
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&<$base>::from(*self), f)
      }
    }
    impl core::fmt::Display for $i {
      #[inline]
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&<$base>::from(*self), f)
      }
    }
    impl core::fmt::LowerExp for $i {
      #[inline]
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::LowerExp::fmt(&<$base>::from(*self), f)
      }
    }
    impl core::fmt::UpperExp for $i {
      #[inline]
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::UpperExp::fmt(&<$base>::from(*self), f)
      }
    }
  };
}

mod _f32;
mod _f64;
mod _i16;
mod _i32;
mod _i64;
mod _u16;
mod _u32;
mod _u64;

pub use self::{_f32::*, _f64::*, _i16::*, _i32::*, _i64::*, _u16::*, _u32::*, _u64::*};
