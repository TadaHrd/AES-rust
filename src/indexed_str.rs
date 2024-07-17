//! Indexed String Slice

/// An Indexed `&str`.
///
/// Its fields are accessible, but it is advised to use the [`IndexedStr::make_array`] function.
#[derive(Clone, Copy, Debug)]
pub struct IndexedStr<'a> {
    /// The string slice
    pub str: &'a str,
    /// The index
    pub idx: usize,
}

impl<'a> IndexedStr<'a> {
    /// Makes an array of [`IndexedStr`]s.
    #[allow(clippy::uninit_assumed_init, invalid_value)]
    pub const fn make_array<const LEN: usize>(array: &'a [&'a str; LEN]) -> [Self; LEN] {
        let mut ret: [Self; LEN] = [unsafe { std::mem::MaybeUninit::uninit().assume_init() }; LEN];

        let mut idx = 0;
        while idx < LEN {
            ret[idx].str = array[idx];
            ret[idx].idx = idx;
            idx += 1
        }

        ret
    }
}

impl<'a> std::fmt::Display for IndexedStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.str)
    }
}
