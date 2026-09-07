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
    name=LowerSnakeCanonical,
    over=Canonical,
    upper=false,
    docs=concat!(
        "A `Display` type for the [`AsLowerSnake`] trait.",
        "\n\n",
        "This type is constructed by calling the [`as_lower_snake_canonical`] method.",
        "\n\n",
        "[`as_lower_snake_canonical`]: AsLowerSnake::as_lower_snake_canonical",
    ),
}

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=LowerSnakeDecorated,
    over=Decorated,
    upper=false,
    docs=concat!(
        "A `Display` type for the [`AsLowerSnake`] trait.",
        "\n\n",
        "This type is constructed by calling the [`as_lower_snake_decorated`] method.",
        "\n\n",
        "[`as_lower_snake_decorated`]: AsLowerSnake::as_lower_snake_decorated",
    ),
}

// =============================================================================
// TRAIT
// =============================================================================

/// Formats an identifier as a preset lower-snake identifier.
///
/// See the [`fmt`] module for more details.
///
/// [`fmt`]: crate::core::fmt
pub trait AsLowerSnake {
    /// Returns a displayable type that converts the provided input to lower
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
    ///         .as_lower_snake_canonical()
    ///         .to_string(),
    ///     "lower_camel_case"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_lower_snake_canonical()
    ///         .to_string(),
    ///     "lower_hybrid_case"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_lower_snake_canonical()
    ///         .to_string(),
    ///     "lower_kebab_case"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_lower_snake_canonical()
    ///         .to_string(),
    ///     "lower_snake_case"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_lower_snake_canonical()
    ///         .to_string(),
    ///     "upper_camel_case"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_lower_snake_canonical()
    ///         .to_string(),
    ///     "upper_hybrid_case"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_lower_snake_canonical()
    ///         .to_string(),
    ///     "upper_kebab_case"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_lower_snake_canonical()
    ///         .to_string(),
    ///     "upper_snake_case"
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
    ///         .as_lower_snake_canonical()
    ///         .to_string(),
    ///     "_"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .as_lower_snake_canonical()
    ///         .to_string(),
    ///     "_2_example_camel"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_snake_canonical(&self) -> LowerSnakeCanonical<'_>;

    /// Returns a displayable type that converts the provided input to lower
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
    ///         .as_lower_snake_decorated()
    ///         .to_string(),
    ///     "__lower__camel_case__"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_lower_snake_decorated()
    ///         .to_string(),
    ///     "__lower__hybrid_case__"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_lower_snake_decorated()
    ///         .to_string(),
    ///     "__lower__kebab_case__"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_lower_snake_decorated()
    ///         .to_string(),
    ///     "__lower__snake_case__"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_lower_snake_decorated()
    ///         .to_string(),
    ///     "__upper__camel_case__"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_lower_snake_decorated()
    ///         .to_string(),
    ///     "__upper__hybrid_case__"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_lower_snake_decorated()
    ///         .to_string(),
    ///     "__upper__kebab_case__"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_lower_snake_decorated()
    ///         .to_string(),
    ///     "__upper__snake_case__"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_snake_decorated(&self) -> LowerSnakeDecorated<'_>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> AsLowerSnake for Ident<B, D, P> {
    #[inline]
    fn as_lower_snake_canonical(&self) -> LowerSnakeCanonical<'_> {
        LowerSnakeCanonical(Canonical::new::<B, D, P::BaseProfile>(self.as_str(), '_'))
    }
    #[inline]
    fn as_lower_snake_decorated(&self) -> LowerSnakeDecorated<'_> {
        LowerSnakeDecorated(Decorated::new::<B, D, P::BaseProfile>(self.as_str(), '_'))
    }
}
