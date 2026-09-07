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
    name=UpperKebabCanonical,
    over=Canonical,
    upper=true,
    docs=concat!(
        "A `Display` type for the [`AsUpperKebab`] trait.",
        "\n\n",
        "This type is constructed by calling the [`as_lower_kebab_canonical`] method.",
        "\n\n",
        "[`as_lower_kebab_canonical`]: AsUpperKebab::as_lower_kebab_canonical",
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
        "This type is constructed by calling the [`as_lower_kebab_decorated`] method.",
        "\n\n",
        "[`as_lower_kebab_decorated`]: AsUpperKebab::as_lower_kebab_decorated",
    ),
}

// =============================================================================
// TRAIT
// =============================================================================

/// Formats an identifier as a preset upper-kebab identifier.
///
/// See the [`fmt`] module for more details.
///
/// [`fmt`]: crate::core::fmt
pub trait AsUpperKebab {
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
    /// Notice that it will attempt to keep necessary prefix delimiters.
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
    fn as_upper_kebab_canonical(&self) -> UpperKebabCanonical<'_> {
        UpperKebabCanonical(Canonical::new::<B, D, P::BaseProfile>(self.as_str(), '-'))
    }
    #[inline]
    fn as_upper_kebab_decorated(&self) -> UpperKebabDecorated<'_> {
        UpperKebabDecorated(Decorated::new::<B, D, P::BaseProfile>(self.as_str(), '-'))
    }
}
