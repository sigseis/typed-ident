// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::alloc::IntoIntermediate;
use crate::syntax::{CasedProfile, Delimiter};
use crate::{Error, Fragment};

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
pub struct CharIntermediate {
    buffer: [u8; 4],
    length: usize,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl CharIntermediate {
    #[inline]
    fn new<D: Delimiter, P: CasedProfile>(c: char) -> Result<Self, Error> {
        let mut buffer = [0u8; 4];
        let length = {
            let string = c.encode_utf8(&mut buffer);
            P::is_fragment::<D>(string)?;
            string.len()
        };
        Ok(Self { buffer, length })
    }
    fn new_delim<D: Delimiter>(delim: D) -> Self {
        let mut buffer = [0u8; 4];
        Self {
            length: delim.as_char().encode_utf8(&mut buffer).len(),
            buffer,
        }
    }
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        &self.buffer[..self.length]
    }
    #[inline]
    fn as_str(&self) -> &str {
        str::from_utf8(self.as_bytes()).expect(concat!(
            "somehow `CharIntermediate` was filled with data that formed an ",
            "invalid UTF-8 string slice; this should not be possible, is ",
            "memory corrupted somehow?",
        ))
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B, D, P> AsRef<Fragment<B, D, P>> for CharIntermediate {
    #[inline(always)]
    fn as_ref(&self) -> &Fragment<B, D, P> {
        // Safe because we only allow valid fragment characters to construct.
        Fragment::new_unchecked(self.as_str())
    }
}

// -----------------------------------------------------------------------------
impl AsRef<str> for CharIntermediate {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

// -----------------------------------------------------------------------------
impl core::fmt::Display for CharIntermediate {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_str().fmt(f)
    }
}

// -----------------------------------------------------------------------------
impl<B, D: Delimiter, P: CasedProfile> IntoIntermediate<B, D, P> for char {
    type Intermediate = CharIntermediate;

    #[inline(always)]
    fn into_intermediate(self) -> Result<Self::Intermediate, Error> {
        CharIntermediate::new::<D, P>(self)
    }
}

// -----------------------------------------------------------------------------
impl<B, D: Delimiter, P: CasedProfile> IntoIntermediate<B, D, P> for D {
    type Intermediate = CharIntermediate;

    #[inline(always)]
    fn into_intermediate(self) -> Result<Self::Intermediate, Error> {
        Ok(CharIntermediate::new_delim(self))
    }
}
