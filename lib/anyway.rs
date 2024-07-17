//! # Anyway Encosure Scheme (AES)
//!
//! This encosure scheme uses the word "anyway" with formatting to store data.
//!
//! Bits in bytes are talked about as LSb to MSb.
//! This means that the 1's place bit is bit 0 and the 128's place bit is bit 7.
//! Byte sequences start with the MSb first.
//!
//! Every byte has 2 parts:
//! - A tail (bits 0-1)
//! - A body (bits 2-7)
//!
//! The tail dictates how many stars come before and after the body.
//!
//! | bit 0 | bit 1 | prefix | suffix |
//! | ----- | ----- | ------ | ------ |
//! | 0     | 0     |        |        |
//! | 1     | 0     | *      | *      |
//! | 0     | 1     | **     | **     |
//! | 1     | 1     | ***    | ***    |
//!
//! The body dictates the case of the letters in the word "anyway".
//!
//! | bit | case      |
//! | --- | --------- |
//! | 0   | uppercase |
//! | 1   | lowercase |
//!
//! The bits control each letter like this:
//!
//! | letter | bit |
//! | ------ | --- |
//! | A      | 2   |
//! | N      | 3   |
//! | Y      | 4   |
//! | W      | 5   |
//! | A      | 6   |
//! | Y      | 7   |
//!
//! For example, "A" would be:
//! - 65 in ASCII
//! - 01000001 in binary
//! - Body: 010000, Tail: 01
//! - Letters: `ANYWaY`
//! - Prefix & suffix: `*`
//! - Encoded as `*ANYWaY*`
//!
//! # Escaped AES (EAES)
//!
//! There is an escaped version of AES that adds escaped stars before and after each word.
//! It doesn't get rid of formatting.
//!
//! The decode step should ignore any `\*` character sequences to work in all places.
//!
//! This is done to make this easily copyable from places that use markdown formatting (like Discord).
//!
//! For example, "A" would be:
//! - Encoded in AES as `*ANYWaY*`
//! - Escape the stars: `*ANYWaY*` -> `\**ANYWaY*\*`
//!
//!
//! ## Examples
//!
//! ### Hello, world!
//!
//! The string "Hello, world!" would look like:
//! ```text
//! H: AnYWaY
//! e: *aNYwaY*
//! l: anYwaY
//! l: anYwaY
//! o: ***anYwaY***
//! ,: anYwAY
//!  : ANYwAY
//! w: ***aNywaY***
//! o: ***anYwaY***
//! r: **ANywaY**
//! l: anYwaY
//! d: aNYwaY
//! !: *ANYwAY*
//! ```
//!
//! Here's how it would look like in EAES:
//! ```text
//! H: AnYWaY
//! e: \**aNYwaY*\*
//! l: anYwaY
//! l: anYwaY
//! o: \*\*\****anYwaY***\*\*\*
//! ,: anYwAY
//!  : ANYwAY
//! w: \*\*\****aNywaY***\*\*\*
//! o: \*\*\****anYwaY***\*\*\*
//! r: \*\***ANywaY**\*\*
//! l: anYwaY
//! d: aNYwaY
//! !: \**ANYwAY*\*
//! ```
//!
//! The characters at the start of every row aren't present in actual encoded data.

use crate::to_bytes::ToBytes;
use std::{
    hint::unreachable_unchecked,
    str::{self, Utf8Error},
};

/// Checks if the input would be a valid separator for AES.
pub fn check_separator(separator: &str) -> bool {
    if separator.is_empty() {
        return false;
    }
    for c in separator.chars() {
        if matches!(
            c,
            'a' | 'n' | 'y' | 'w' | 'A' | 'N' | 'Y' | 'W' | '*' | '\\'
        ) {
            return false;
        }
    }

    true
}

/// Encodes input (in bytes) to AES.
///
/// For more information, check the module's documentation.
pub fn encode<T: ToBytes, S: AsRef<str>>(input: T, separator: S) -> String {
    encode_escape(input, separator, false)
}

/// Encodes input (in bytes) to EAES.
///
/// For more information, check the module's documentation.
pub fn encode_escaped<T: ToBytes, S: AsRef<str>>(input: T, separator: S) -> String {
    encode_escape(input, separator, true)
}

/// Encodes input (in bytes) to AES or EAES depending on if `escape` is true.
///
/// For more information, check the module's documentation.
pub fn encode_escape<T: ToBytes, S: AsRef<str>>(input: T, separator: S, escape: bool) -> String {
    let separator = separator.as_ref();
    let separator = match check_separator(separator) {
        true => separator,
        false => ", ",
    };

    let data = input.to_bytes();

    let mut ret = String::with_capacity(data.len() * 6);

    for val in data {
        let tail = val & 0b11;
        let body = (val & 0b11111100) >> 2;

        let fix = match tail {
            0 => "",
            1 => "*",
            2 => "**",
            3 => "***",
            // SAFETY: `tail`` cannot be over 3 (0b11)
            _ => unsafe { unreachable_unchecked() },
        };
        let fix2 = match tail {
            0 => "",
            1 => "\\*",
            2 => "\\*\\*",
            3 => "\\*\\*\\*",
            // SAFETY: `tail`` cannot be over 3 (0b11)
            _ => unsafe { unreachable_unchecked() },
        };

        // `&str`s are immutable, so we have to use a `&mut [u8]`
        let line: &mut [u8] = &mut [0; 6];

        line[0] = b'A' + (32 * (body & 1));
        line[1] = b'N' + (32 * ((body >> 1) & 1));
        line[2] = b'Y' + (32 * ((body >> 2) & 1));
        line[3] = b'W' + (32 * ((body >> 3) & 1));
        line[4] = b'A' + (32 * ((body >> 4) & 1));
        line[5] = b'Y' + (32 * ((body >> 5) & 1));

        // SAFETY: line only has valid ASCII characters
        let line_str: &str = unsafe { str::from_utf8_unchecked(line) };

        if escape {
            ret.push_str(fix2);
            ret.push_str(fix);
            ret.push_str(line_str);
            ret.push_str(fix);
            ret.push_str(fix2);
        } else {
            ret.push_str(fix);
            ret.push_str(line_str);
            ret.push_str(fix);
        }

        ret.push_str(separator);
    }
    for _ in 0..separator.len() {
        ret.pop();
    }
    ret
}

/// Decodes AES and EAES to a string.
///
/// Returns an error alongside a vector with the decoded data if it can't be constructed into a [`String`].
///
/// For more information, check the module's documentation.
pub fn decode_to_string(text: &str) -> Result<String, (Utf8Error, Vec<u8>)> {
    let vec = decode(text);

    let string = str::from_utf8(&vec);
    match string {
        // SAFETY: `from_utf8` is performer earlier
        Ok(_) => unsafe { Ok(String::from_utf8_unchecked(vec)) },
        Err(e) => Err((e, vec)),
    }
}

/// Decodes AES and EAES to a vector.
///
/// For more information, check the module's documentation.
pub fn decode(text: &str) -> Vec<u8> {
    let data = text.as_bytes();

    let mut body_idx: u8 = 0;
    let mut body: u8 = 0;
    let mut stars: u8 = 0;

    let mut ret = Vec::new();

    let mut idx: usize = 0;

    while idx < data.len() {
        while idx < data.len()
            && !matches!(
                data[idx],
                b'a' | b'n' | b'y' | b'w' | b'A' | b'N' | b'Y' | b'W' | b'*' | b'\\'
            )
        {
            idx += 1
        }
        if idx == data.len() {
            break;
        }
        let val = data[idx];

        match val {
            b'\\' => {
                idx += 2;
                continue;
            }
            b'*' => stars += 1,

            96.. => {
                body += 1 << body_idx;
                body_idx += 1;
            }
            ..=95 => body_idx += 1,
        }

        #[allow(unused_assignments)]
        if body_idx == 6 {
            body_idx = 0;
            stars %= 4; // eradicate potential edge cases

            let value = (body << 2) + stars;
            ret.push(value);

            while idx < data.len()
                && matches!(
                    data[idx],
                    b'a' | b'n' | b'y' | b'w' | b'A' | b'N' | b'Y' | b'W' | b'*' | b'\\'
                )
            {
                idx += 1
            }

            body = 0;
            stars = 0;

            continue;
        }

        idx += 1;
    }

    ret
}
