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
    name=UpperSnake,
    over=Canonical,
    upper=true,
    docs=concat!(
        "A `Display` type for the [`AsUpperSnake`] trait.",
        "\n\n",
        "This type is constructed by calling the [`as_upper_snake`] method.",
        "\n\n",
        "[`as_upper_snake`]: AsUpperSnake::as_upper_snake",
    ),
}

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

/// Provides methods for formatting an identifier in upper-snake format, using
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
pub trait AsUpperSnake {
    /// Returns a displayable type that converts the provided input to upper
    /// snake in plain form.
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
    ///         .as_upper_snake()
    ///         .to_string(),
    ///     "LOWER_CAMEL_CASE"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_upper_snake()
    ///         .to_string(),
    ///     "LOWER_HYBRID_CASE"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_upper_snake()
    ///         .to_string(),
    ///     "LOWER_KEBAB_CASE"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_upper_snake()
    ///         .to_string(),
    ///     "LOWER_SNAKE_CASE"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_upper_snake()
    ///         .to_string(),
    ///     "UPPER_CAMEL_CASE"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_upper_snake()
    ///         .to_string(),
    ///     "UPPER_HYBRID_CASE"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_upper_snake()
    ///         .to_string(),
    ///     "UPPER_KEBAB_CASE"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_upper_snake()
    ///         .to_string(),
    ///     "UPPER_SNAKE_CASE"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// This form will *NOT* attempt to validate the first character to ensure
    /// it could be considered a valid identifier (see
    /// [`as_upper_snake_canonical`] for a version that would validate this).
    ///
    /// [`as_upper_snake_canonical`]: Self::as_upper_snake_canonical
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .as_upper_snake()
    ///         .to_string(),
    ///     ""
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .as_upper_snake()
    ///         .to_string(),
    ///     "2_EXAMPLE_CAMEL"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_snake(&self) -> UpperSnake<'_>;

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
    /// This form *WILL* attempt to validate the first character to ensure it
    /// could be considered a valid identifier (see [`as_upper_snake`] for a
    /// version that would *NOT* validate this).
    ///
    /// [`as_upper_snake`]: Self::as_upper_snake
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
    fn as_upper_snake(&self) -> UpperSnake<'_> {
        UpperSnake(Canonical::new::<B, D, P::CharProfile>(
            self.as_str(),
            '_',
            false,
        ))
    }
    #[inline]
    fn as_upper_snake_canonical(&self) -> UpperSnakeCanonical<'_> {
        UpperSnakeCanonical(Canonical::new::<B, D, P::CharProfile>(
            self.as_str(),
            '_',
            true,
        ))
    }
    #[inline]
    fn as_upper_snake_decorated(&self) -> UpperSnakeDecorated<'_> {
        UpperSnakeDecorated(Decorated::new::<B, D, P::CharProfile>(self.as_str(), '_'))
    }
}
