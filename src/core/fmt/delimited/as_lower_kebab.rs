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

/// Formats an identifier as a preset lower-camel identifier.
///
/// See the [`fmt`] module for more details.
///
/// [`fmt`]: crate::core::fmt
pub trait AsLowerKebab {
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
    /// Notice that it will attempt to keep necessary prefix delimiters.
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
    fn as_lower_kebab_canonical(&self) -> LowerKebabCanonical<'_> {
        LowerKebabCanonical(Canonical::new::<B, D, P::BaseProfile>(self.as_str(), '-'))
    }
    #[inline]
    fn as_lower_kebab_decorated(&self) -> LowerKebabDecorated<'_> {
        LowerKebabDecorated(Decorated::new::<B, D, P::BaseProfile>(self.as_str(), '-'))
    }
}
