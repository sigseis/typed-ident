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
    name=LowerKebab,
    over=Canonical,
    upper=false,
    docs=concat!(
        "A `Display` type for the [`AsLowerKebab`] trait.",
        "\n\n",
        "This type is constructed by calling the [`as_lower_kebab`] method.",
        "\n\n",
        "[`as_lower_kebab`]: AsLowerKebab::as_lower_kebab",
    ),
}

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=LowerKebabCanonical,
    over=Canonical,
    upper=false,
    docs=concat!(
        "A `Display` type for the [`AsLowerKebab`] trait.",
        "\n\n",
        "This type is constructed by calling the [`as_lower_kebab_canonical`] method.",
        "\n\n",
        "[`as_lower_kebab_canonical`]: AsLowerKebab::as_lower_kebab_canonical",
    ),
}

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=LowerKebabDecorated,
    over=Decorated,
    upper=false,
    docs=concat!(
        "A `Display` type for the [`AsLowerKebab`] trait.",
        "\n\n",
        "This type is constructed by calling the [`as_lower_kebab_decorated`] method.",
        "\n\n",
        "[`as_lower_kebab_decorated`]: AsLowerKebab::as_lower_kebab_decorated",
    ),
}

// =============================================================================
// TRAIT
// =============================================================================

/// Provides methods for formatting an identifier in lower-kebab format, using
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
pub trait AsLowerKebab {
    /// Returns a displayable type that converts the provided input to lower
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
    ///         .as_lower_kebab()
    ///         .to_string(),
    ///     "lower-camel-case"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_lower_kebab()
    ///         .to_string(),
    ///     "lower-hybrid-case"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_lower_kebab()
    ///         .to_string(),
    ///     "lower-kebab-case"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_lower_kebab()
    ///         .to_string(),
    ///     "lower-snake-case"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_lower_kebab()
    ///         .to_string(),
    ///     "upper-camel-case"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_lower_kebab()
    ///         .to_string(),
    ///     "upper-hybrid-case"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_lower_kebab()
    ///         .to_string(),
    ///     "upper-kebab-case"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_lower_kebab()
    ///         .to_string(),
    ///     "upper-snake-case"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// This form will *NOT* attempt to validate the first character to ensure
    /// it could be considered a valid identifier (see
    /// [`as_lower_kebab_canonical`] for a version that would validate this).
    ///
    /// [`as_lower_kebab_canonical`]: Self::as_lower_kebab_canonical
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .as_lower_kebab()
    ///         .to_string(),
    ///     ""
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .as_lower_kebab()
    ///         .to_string(),
    ///     "2-example-camel"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_kebab(&self) -> LowerKebab<'_>;

    /// Returns a displayable type that converts the provided input to lower
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
    ///         .as_lower_kebab_canonical()
    ///         .to_string(),
    ///     "lower-camel-case"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_lower_kebab_canonical()
    ///         .to_string(),
    ///     "lower-hybrid-case"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_lower_kebab_canonical()
    ///         .to_string(),
    ///     "lower-kebab-case"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_lower_kebab_canonical()
    ///         .to_string(),
    ///     "lower-snake-case"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_lower_kebab_canonical()
    ///         .to_string(),
    ///     "upper-camel-case"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_lower_kebab_canonical()
    ///         .to_string(),
    ///     "upper-hybrid-case"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_lower_kebab_canonical()
    ///         .to_string(),
    ///     "upper-kebab-case"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_lower_kebab_canonical()
    ///         .to_string(),
    ///     "upper-snake-case"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// This form *WILL* attempt to validate the first character to ensure it
    /// could be considered a valid identifier (see [`as_lower_kebab`] for a
    /// version that would *NOT* validate this).
    ///
    /// [`as_lower_kebab`]: Self::as_lower_kebab
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .as_lower_kebab_canonical()
    ///         .to_string(),
    ///     "-"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .as_lower_kebab_canonical()
    ///         .to_string(),
    ///     "-2-example-camel"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_kebab_canonical(&self) -> LowerKebabCanonical<'_>;

    /// Returns a displayable type that converts the provided input to lower
    /// kebab in decorated form.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .as_lower_kebab_decorated()
    ///         .to_string(),
    ///     "--lower--camel-case--"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_lower_kebab_decorated()
    ///         .to_string(),
    ///     "--lower--hybrid-case--"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_lower_kebab_decorated()
    ///         .to_string(),
    ///     "--lower--kebab-case--"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_lower_kebab_decorated()
    ///         .to_string(),
    ///     "--lower--snake-case--"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_lower_kebab_decorated()
    ///         .to_string(),
    ///     "--upper--camel-case--"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_lower_kebab_decorated()
    ///         .to_string(),
    ///     "--upper--hybrid-case--"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_lower_kebab_decorated()
    ///         .to_string(),
    ///     "--upper--kebab-case--"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_lower_kebab_decorated()
    ///         .to_string(),
    ///     "--upper--snake-case--"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_kebab_decorated(&self) -> LowerKebabDecorated<'_>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> AsLowerKebab for Ident<B, D, P> {
    #[inline]
    fn as_lower_kebab(&self) -> LowerKebab<'_> {
        LowerKebab(Canonical::new::<B, D, P::CharProfile>(
            self.as_str(),
            '-',
            false,
        ))
    }
    #[inline]
    fn as_lower_kebab_canonical(&self) -> LowerKebabCanonical<'_> {
        LowerKebabCanonical(Canonical::new::<B, D, P::CharProfile>(
            self.as_str(),
            '-',
            true,
        ))
    }
    #[inline]
    fn as_lower_kebab_decorated(&self) -> LowerKebabDecorated<'_> {
        LowerKebabDecorated(Decorated::new::<B, D, P::CharProfile>(self.as_str(), '-'))
    }
}
