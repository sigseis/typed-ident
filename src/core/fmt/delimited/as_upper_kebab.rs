// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::Ident;
use crate::core::fmt::delimited::{Canonical, Decorated};
use crate::syntax::{Boundary, Delimiter, Profile};

// =============================================================================
// TYPEDEFS
// =============================================================================

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=UpperKebab,
    over=Canonical,
    upper=true,
    docs=concat!(
        "A `Display` type for the [`AsUpperKebab`] trait.",
        "\n\n",
        "This type is constructed by calling the [`as_upper_kebab`] method.",
        "\n\n",
        "[`as_upper_kebab`]: AsUpperKebab::as_upper_kebab",
    ),
}

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=UpperKebabCanonical,
    over=Canonical,
    upper=true,
    docs=concat!(
        "A `Display` type for the [`AsUpperKebab`] trait.",
        "\n\n",
        "This type is constructed by calling the [`as_upper_kebab_canonical`] method.",
        "\n\n",
        "[`as_upper_kebab_canonical`]: AsUpperKebab::as_upper_kebab_canonical",
    ),
}

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=UpperKebabDecorated,
    over=Decorated,
    upper=true,
    docs=concat!(
        "A `Display` type for the [`AsUpperKebab`] trait.",
        "\n\n",
        "This type is constructed by calling the [`as_upper_kebab_decorated`] method.",
        "\n\n",
        "[`as_upper_kebab_decorated`]: AsUpperKebab::as_upper_kebab_decorated",
    ),
}

// =============================================================================
// TRAIT
// =============================================================================

/// Provides methods for formatting an identifier in upper-kebab format, using
/// plain, canonical, or decorated forms.
///
/// For explicit-delimited identifiers like this one, there's no difference
/// between the decorated and delimited forms. So a delimited method is not
/// provided.
///
/// See the [`fmt`] module for more details.
///
/// # Return Values
///
/// All of the methods on this trait return types that borrow the source
/// identifier, and implement the `Display` trait so that they are usable from
/// a formatting call (or convertible to a `String`, via `ToString`).
///
/// Calling these methods is cheap, since the work isn't done until we actually
/// use it for a formatting operation.
///
/// [`fmt`]: crate::core::fmt
pub trait AsUpperKebab {
    /// Returns a displayable type that converts the provided input to upper
    /// kebab in plain form.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .as_upper_kebab()
    ///         .to_string(),
    ///     "LOWER-CAMEL-CASE"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_upper_kebab()
    ///         .to_string(),
    ///     "LOWER-HYBRID-CASE"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_upper_kebab()
    ///         .to_string(),
    ///     "LOWER-KEBAB-CASE"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_upper_kebab()
    ///         .to_string(),
    ///     "LOWER-SNAKE-CASE"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_upper_kebab()
    ///         .to_string(),
    ///     "UPPER-CAMEL-CASE"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_upper_kebab()
    ///         .to_string(),
    ///     "UPPER-HYBRID-CASE"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_upper_kebab()
    ///         .to_string(),
    ///     "UPPER-KEBAB-CASE"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_upper_kebab()
    ///         .to_string(),
    ///     "UPPER-SNAKE-CASE"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// This form will *NOT* attempt to validate the first character to ensure
    /// it could be considered a valid identifier (see
    /// [`as_upper_kebab_canonical`] for a version that would validate this).
    ///
    /// [`as_upper_kebab_canonical`]: Self::as_upper_kebab_canonical
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .as_upper_kebab()
    ///         .to_string(),
    ///     ""
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .as_upper_kebab()
    ///         .to_string(),
    ///     "2-EXAMPLE-CAMEL"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_kebab(&self) -> UpperKebab<'_>;

    /// Returns a displayable type that converts the provided input to upper
    /// kebab in canonical form.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .as_upper_kebab_canonical()
    ///         .to_string(),
    ///     "LOWER-CAMEL-CASE"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_upper_kebab_canonical()
    ///         .to_string(),
    ///     "LOWER-HYBRID-CASE"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_upper_kebab_canonical()
    ///         .to_string(),
    ///     "LOWER-KEBAB-CASE"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_upper_kebab_canonical()
    ///         .to_string(),
    ///     "LOWER-SNAKE-CASE"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_upper_kebab_canonical()
    ///         .to_string(),
    ///     "UPPER-CAMEL-CASE"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_upper_kebab_canonical()
    ///         .to_string(),
    ///     "UPPER-HYBRID-CASE"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_upper_kebab_canonical()
    ///         .to_string(),
    ///     "UPPER-KEBAB-CASE"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_upper_kebab_canonical()
    ///         .to_string(),
    ///     "UPPER-SNAKE-CASE"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// This form *WILL* attempt to validate the first character to ensure it
    /// could be considered a valid identifier (see [`as_upper_kebab`] for a
    /// version that would *NOT* validate this).
    ///
    /// [`as_upper_kebab`]: Self::as_upper_kebab
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .as_upper_kebab_canonical()
    ///         .to_string(),
    ///     "-"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .as_upper_kebab_canonical()
    ///         .to_string(),
    ///     "-2-EXAMPLE-CAMEL"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_kebab_canonical(&self) -> UpperKebabCanonical<'_>;

    /// Returns a displayable type that converts the provided input to upper
    /// kebab in decorated form.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .as_upper_kebab_decorated()
    ///         .to_string(),
    ///     "--LOWER--CAMEL-CASE--"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_upper_kebab_decorated()
    ///         .to_string(),
    ///     "--LOWER--HYBRID-CASE--"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_upper_kebab_decorated()
    ///         .to_string(),
    ///     "--LOWER--KEBAB-CASE--"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_upper_kebab_decorated()
    ///         .to_string(),
    ///     "--LOWER--SNAKE-CASE--"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_upper_kebab_decorated()
    ///         .to_string(),
    ///     "--UPPER--CAMEL-CASE--"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_upper_kebab_decorated()
    ///         .to_string(),
    ///     "--UPPER--HYBRID-CASE--"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_upper_kebab_decorated()
    ///         .to_string(),
    ///     "--UPPER--KEBAB-CASE--"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_upper_kebab_decorated()
    ///         .to_string(),
    ///     "--UPPER--SNAKE-CASE--"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_kebab_decorated(&self) -> UpperKebabDecorated<'_>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> AsUpperKebab for Ident<B, D, P> {
    #[inline]
    fn as_upper_kebab(&self) -> UpperKebab<'_> {
        UpperKebab(Canonical::new::<B, D, P::CharProfile>(
            self.as_str(),
            '-',
            false,
        ))
    }
    #[inline]
    fn as_upper_kebab_canonical(&self) -> UpperKebabCanonical<'_> {
        UpperKebabCanonical(Canonical::new::<B, D, P::CharProfile>(
            self.as_str(),
            '-',
            true,
        ))
    }
    #[inline]
    fn as_upper_kebab_decorated(&self) -> UpperKebabDecorated<'_> {
        UpperKebabDecorated(Decorated::new::<B, D, P::CharProfile>(self.as_str(), '-'))
    }
}
