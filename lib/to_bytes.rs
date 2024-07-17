//! # To Bytes
//!
//! This module features the [`ToBytes`] trait. That's it.

use std::str;

/// This trait is used in the encosure schemes as the type of `input`.
///
/// It provides the `to_bytes` function that turns the value provided into a `&[u8]`.
pub trait ToBytes {
    /// This function turns `&self` into `&[u8]` (a slice of bytes).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use aes_rust::to_bytes::ToBytes;
    /// assert_eq!("ABC".to_bytes(), [65, 66, 67]);
    /// assert_eq!([0, 1, 2, 3].to_bytes(), [0, 1, 2, 3]);
    /// ```
    fn to_bytes(&self) -> &[u8];
}

impl ToBytes for &str {
    fn to_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl ToBytes for String {
    fn to_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<const N: usize> ToBytes for [u8; N] {
    fn to_bytes(&self) -> &[u8] {
        self
    }
}
impl<'a, const N: usize> ToBytes for &'a [u8; N] {
    fn to_bytes(&self) -> &[u8] {
        #[allow(clippy::explicit_auto_deref)]
        *self
    }
}

impl ToBytes for &[u8] {
    fn to_bytes(&self) -> &[u8] {
        self
    }
}

impl ToBytes for Vec<u8> {
    fn to_bytes(&self) -> &[u8] {
        self
    }
}
