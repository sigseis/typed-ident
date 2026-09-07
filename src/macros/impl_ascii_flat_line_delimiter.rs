macro_rules! impl_ascii_flat_line_delimiter {
    (
        name=$name:ident,
        char=$char:literal,
        docs=$docs:expr,
    ) => {
        #[doc = $docs]
        #[derive(Copy, Clone, Debug, Default, Eq, Ord)]
        pub struct $name;

        // ---------------------------------------------------------------------
        impl crate::syntax::delimiter::UnitDelimiter for $name {
            const CHAR: char = $char;
            const STR: &'static str = concat!($char);
        }

        // ---------------------------------------------------------------------
        impl AsRef<char> for $name {
            fn as_ref(&self) -> &char {
                &Self::CHAR
            }
        }

        // ---------------------------------------------------------------------
        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                Self::STR
            }
        }

        // ---------------------------------------------------------------------
        impl core::cmp::PartialEq<char> for $name {
            fn eq(&self, rhs: &char) -> bool {
                $char.eq(rhs)
            }
        }

        // ---------------------------------------------------------------------
        impl core::cmp::PartialEq<$name> for char {
            fn eq(&self, _: &$name) -> bool {
                *self == $char
            }
        }

        // ---------------------------------------------------------------------
        impl<D: Delimiter> core::cmp::PartialEq<D> for $name {
            fn eq(&self, rhs: &D) -> bool {
                $char == rhs.as_char()
            }
        }

        // ---------------------------------------------------------------------
        impl core::cmp::PartialOrd<char> for $name {
            fn partial_cmp(&self, rhs: &char) -> Option<core::cmp::Ordering> {
                $char.partial_cmp(rhs)
            }
        }

        // ---------------------------------------------------------------------
        impl core::cmp::PartialOrd<$name> for char {
            fn partial_cmp(&self, rhs: &$name) -> Option<core::cmp::Ordering> {
                self.partial_cmp(&rhs.as_char())
            }
        }

        // ---------------------------------------------------------------------
        impl<D: Delimiter> core::cmp::PartialOrd<D> for $name {
            fn partial_cmp(&self, rhs: &D) -> Option<core::cmp::Ordering> {
                $char.partial_cmp(&rhs.as_char())
            }
        }

        // ---------------------------------------------------------------------
        impl core::convert::From<$name> for char {
            #[inline]
            fn from(_: $name) -> Self {
                $char
            }
        }

        // ---------------------------------------------------------------------
        impl core::convert::TryFrom<AsciiFlatLine> for $name {
            type Error = TryFromCharError;

            #[inline]
            fn try_from(orig: AsciiFlatLine) -> Result<Self, Self::Error> {
                match orig {
                    AsciiFlatLine::$name => Ok(Self),
                    _ => Err(TryFromCharError(())),
                }
            }
        }

        // ---------------------------------------------------------------------
        impl core::convert::TryFrom<AsciiPunctuation> for $name {
            type Error = TryFromCharError;

            #[inline]
            fn try_from(orig: AsciiPunctuation) -> Result<Self, Self::Error> {
                match orig {
                    AsciiPunctuation::$name => Ok(Self),
                    _ => Err(TryFromCharError(())),
                }
            }
        }

        // ---------------------------------------------------------------------
        impl core::convert::TryFrom<char> for $name {
            type Error = TryFromCharError;

            #[inline]
            fn try_from(orig: char) -> Result<Self, Self::Error> {
                match orig {
                    $char => Ok(Self),
                    _ => Err(TryFromCharError(())),
                }
            }
        }

        // ---------------------------------------------------------------------
        impl core::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                use core::fmt::Write;
                f.write_char($char)
            }
        }

        // ---------------------------------------------------------------------
        impl core::hash::Hash for $name {
            #[inline]
            fn hash<H>(&self, state: &mut H)
            where
                H: core::hash::Hasher,
            {
                $char.hash(state)
            }
        }

        // ---------------------------------------------------------------------
        /// Proof: It's always safe to implement this against yourself.
        // ---------------------------------------------------------------------
        impl crate::syntax::SubsetOf<$name> for $name {}

        // ---------------------------------------------------------------------
        /// Proof: AsciiPunctuation very obviously contains a flat-line symbol ('_' or '-').
        // ---------------------------------------------------------------------
        impl crate::syntax::SubsetOf<AsciiPunctuation> for $name {}

        // ---------------------------------------------------------------------
        /// Proof: AsciiFlatLine very obviously contains a flat-line symbol ('_' or '-').
        // ---------------------------------------------------------------------
        impl crate::syntax::SubsetOf<AsciiFlatLine> for $name {}
    };
}
