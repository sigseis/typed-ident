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
    name=UpperSnakeCanonical,
    over=Canonical,
    upper=true,
    docs=concat!(
        "A `Display` type for the [`AsUpperSnake`] trait.",
        "\n\n",
        "This type is constructed by calling the [`as_upper_snake_canonical`] method.",
        "\n\n",
        "[`as_upper_snake_canonical`]: AsUpperSnake::as_upper_snake_canonical",
    ),
}

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=UpperSnakeDecorated,
    over=Decorated,
    upper=true,
    docs=concat!(
        "A `Display` type for the [`AsUpperSnake`] trait.",
        "\n\n",
        "This type is constructed by calling the [`as_upper_snake_decorated`] method.",
        "\n\n",
        "[`as_upper_snake_decorated`]: AsUpperSnake::as_upper_snake_decorated",
    ),
}

// =============================================================================
// TRAIT
// =============================================================================

/// Formats an identifier as a preset upper-snake identifier.
///
/// See the [`fmt`] module for more details.
///
/// [`fmt`]: crate::core::fmt
pub trait AsUpperSnake {
    /// Returns a displayable type that converts the provided input to upper
    /// snake in canonical form.
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
    ///         .as_upper_snake_canonical()
    ///         .to_string(),
    ///     "LOWER_CAMEL_CASE"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_upper_snake_canonical()
    ///         .to_string(),
    ///     "LOWER_HYBRID_CASE"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_upper_snake_canonical()
    ///         .to_string(),
    ///     "LOWER_KEBAB_CASE"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_upper_snake_canonical()
    ///         .to_string(),
    ///     "LOWER_SNAKE_CASE"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_upper_snake_canonical()
    ///         .to_string(),
    ///     "UPPER_CAMEL_CASE"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_upper_snake_canonical()
    ///         .to_string(),
    ///     "UPPER_HYBRID_CASE"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_upper_snake_canonical()
    ///         .to_string(),
    ///     "UPPER_KEBAB_CASE"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_upper_snake_canonical()
    ///         .to_string(),
    ///     "UPPER_SNAKE_CASE"
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
    ///         .as_upper_snake_canonical()
    ///         .to_string(),
    ///     "_"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .as_upper_snake_canonical()
    ///         .to_string(),
    ///     "_2_EXAMPLE_CAMEL"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_snake_canonical(&self) -> UpperSnakeCanonical<'_>;

    /// Returns a displayable type that converts the provided input to upper
    /// snake in decorated form.
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
    ///         .as_upper_snake_decorated()
    ///         .to_string(),
    ///     "__LOWER__CAMEL_CASE__"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_upper_snake_decorated()
    ///         .to_string(),
    ///     "__LOWER__HYBRID_CASE__"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_upper_snake_decorated()
    ///         .to_string(),
    ///     "__LOWER__KEBAB_CASE__"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_upper_snake_decorated()
    ///         .to_string(),
    ///     "__LOWER__SNAKE_CASE__"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_upper_snake_decorated()
    ///         .to_string(),
    ///     "__UPPER__CAMEL_CASE__"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_upper_snake_decorated()
    ///         .to_string(),
    ///     "__UPPER__HYBRID_CASE__"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_upper_snake_decorated()
    ///         .to_string(),
    ///     "__UPPER__KEBAB_CASE__"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_upper_snake_decorated()
    ///         .to_string(),
    ///     "__UPPER__SNAKE_CASE__"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_snake_decorated(&self) -> UpperSnakeDecorated<'_>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> AsUpperSnake for Ident<B, D, P> {
    #[inline]
    fn as_upper_snake_canonical(&self) -> UpperSnakeCanonical<'_> {
        UpperSnakeCanonical(Canonical::new::<B, D, P::BaseProfile>(self.as_str(), '_'))
    }
    #[inline]
    fn as_upper_snake_decorated(&self) -> UpperSnakeDecorated<'_> {
        UpperSnakeDecorated(Decorated::new::<B, D, P::BaseProfile>(self.as_str(), '_'))
    }
}
