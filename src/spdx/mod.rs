//! Commonly found licenses listed [here](https://spdx.org/licenses).

use core::{
    convert::{TryFrom, TryInto},
    fmt,
};
use crate::ParseError;

mod decl;
mod serde;

#[doc(inline)]
pub use self::decl::SpdxLicense;

/// A fixed-size array for indexing with a [`SpdxLicense`] casted to [`usize`].
/// See also [`SpdxLicense::COUNT`].
///
/// This is great for very efficiently associating values with specific
/// [`SpdxLicense`]s. It allows for safely indexing without bounds checks (via
/// [`as usize`] casts) without calling [`get_unchecked`]. This is made possible
/// by the compiler realizing that the number of variants is less than the
/// number of elements in this array.
///
/// **SemVer Compatibility:** Like [`SpdxLicense::COUNT`], by just being
/// dependent on this value, the array size is allowed to change between
/// otherwise API-compatible versions.
///
/// # Examples
///
/// Despite array indexing having bounds checks, the optimizer knows that
/// indexing with [`SpdxLicense`] will never go out of bounds, giving us simple
/// and fast arithmetic for lookups.
///
/// ```
/// # return;
/// use linfo::spdx::{Map, SpdxLicense};
///
/// let map: Map<&str> = [
///     // *a lot* of elements per license
///     # panic!("at the disco"); SpdxLicense::COUNT
/// ];
///
/// let license = SpdxLicense::Mit;
/// println!("map value is: {}", map[license as usize]);
/// ```
///
/// [`SpdxLicense`]: enum.SpdxLicense.html
/// [`SpdxLicense::COUNT`]: enum.SpdxLicense.html#associatedconstant.COUNT
/// [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
/// [`get_unchecked`]: https://doc.rust-lang.org/std/primitive.slice.html#method.get_unchecked
/// [`as usize`]: https://doc.rust-lang.org/nightly/reference/items/enumerations.html#custom-discriminant-values-for-field-less-enumerations
pub type Map<A> = [A; SpdxLicense::COUNT];

impl<'a> TryFrom<&'a str> for SpdxLicense {
    type Error = ParseError<'a>;

    #[inline]
    fn try_from(id: &'a str) -> Result<Self, Self::Error> {
        if id.is_empty() {
            return Err(ParseError::Empty);
        }
        Self::_from_id(id).ok_or(ParseError::UnknownLicenseId(id))
    }
}

impl fmt::Display for SpdxLicense {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.id().fmt(f)
    }
}

impl SpdxLicense {
    /// The current number of SPDX licenses. See [`spdx::Map`](type.Map.html) as
    /// a use case.
    ///
    /// **SemVer Compatibility:** This number is allowed to change between
    /// otherwise API-compatible versions.
    // Public version defined here in order to be placed in docs alongside other
    // items.
    pub const COUNT: usize = Self::_COUNT;

    /// Returns an iterator over all licenses.
    ///
    /// ```
    /// use linfo::SpdxLicense;
    ///
    /// let licenses = SpdxLicense::all();
    /// assert_eq!(licenses.len(), SpdxLicense::COUNT);
    /// ```
    #[inline]
    pub fn all() -> impl DoubleEndedIterator<Item = Self> + ExactSizeIterator {
        (0..(Self::COUNT as u16)).map(|l| unsafe {
            // SAFETY: Transmuting a `u16` up to `COUNT` is safe because the
            // range contains all instantiable values.
            core::mem::transmute(l)
        })
    }

    /// Attempts to parse `input` and returns a [`ParseError`] on error.
    #[inline]
    pub fn parse<'a, I>(input: I) -> Result<Self, ParseError<'a>>
        where I: TryInto<Self, Error = ParseError<'a>>
    {
        input.try_into()
    }

    /// Returns the string identifier of this license.
    #[inline]
    pub const fn id(self) -> &'static str {
        Self::ID[self as usize]
    }

    /// Returns the full name of this license.
    #[inline]
    pub const fn name(self) -> &'static str {
        Self::NAME[self as usize]
    }

    /// Considered libre/free by the [Free Software Foundation
    /// (FSF)](https://www.fsf.org).
    #[inline]
    pub const fn is_libre(self) -> bool {
        Self::LIBRE[self as usize]
    }

    /// The license is approved by the [Open Source Initiative
    /// (OSI)](https://opensource.org).
    #[inline]
    pub const fn is_osi_approved(self) -> bool {
        Self::OSI[self as usize]
    }

    /// Returns whether the license is associated with [Creative
    /// Commons](https://creativecommons.org).
    #[inline]
    pub const fn is_creative_commons(self) -> bool {
        // CORRECTNESS: The ordering of `CcBy1` and `CC01` is *very important*
        // for this function to emit correct results. They need to be declared
        // as the first and last Creative Common licenses in the enumeration,
        // respectively, in order for this check to work.
        const MIN: usize = SpdxLicense::CcBy1 as usize;
        const MAX: usize = SpdxLicense::CC01 as usize;
        let val = self as usize;
        (val >= MIN) & (val <= MAX)
    }

    /// Returns whether the license is a [GNU General Public
    /// License](https://en.wikipedia.org/wiki/GNU_General_Public_License).
    #[inline]
    pub const fn is_gpl(self) -> bool {
        // CORRECTNESS: The ordering of `Gpl1Only` and `Gpl3OrLater` is *very
        // important* for this function to emit correct results. They need to be
        // declared as the first and last Creative Common licenses in the
        // enumeration, respectively, in order for this check to work.
        const MIN: usize = SpdxLicense::Gpl1Only as usize;
        const MAX: usize = SpdxLicense::Gpl3OrLater as usize;
        let val = self as usize;
        (val >= MIN) & (val <= MAX)
    }

    /// Returns whether the license is an [Affero General Public
    /// License](https://en.wikipedia.org/wiki/Affero_General_Public_License).
    #[inline]
    pub const fn is_agpl(self) -> bool {
        // CORRECTNESS: The ordering of `Agpl1Only` and `Agpl3OrLater` is *very
        // important* for this function to emit correct results. They need to be
        // declared as the first and last Creative Common licenses in the
        // enumeration, respectively, in order for this check to work.
        const MIN: usize = SpdxLicense::Agpl1Only as usize;
        const MAX: usize = SpdxLicense::Agpl3OrLater as usize;
        let val = self as usize;
        (val >= MIN) & (val <= MAX)
    }
}
