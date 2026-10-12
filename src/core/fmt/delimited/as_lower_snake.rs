// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Identifier;
use crate::fmt::delimited::{Canonical, Decorated};

// =============================================================================
// TYPEDEFS
// =============================================================================

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=LowerSnake,
    over=Canonical,
    upper=false,
    docs=concat!(
        "A `Display` type for the [`AsLowerSnake`] trait.",
        "\n\n",
        "This type is constructed by calling the [`as_lower_snake`] method.",
        "\n\n",
        "[`as_lower_snake`]: AsLowerSnake::as_lower_snake",
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

/// Provides methods for formatting an identifier in lower-snake format, using
/// canonical, or decorated forms.
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
/// [`fmt`]: crate::fmt
pub trait AsLowerSnake {
    /// Returns a displayable type that converts the provided input to lower
    /// snake in canonical form.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`fmt`]: crate::fmt
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .as_lower_snake()
    ///         .to_string(),
    ///     "lower_camel_case"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_lower_snake()
    ///         .to_string(),
    ///     "lower_hybrid_case"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_lower_snake()
    ///         .to_string(),
    ///     "lower_kebab_case"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_lower_snake()
    ///         .to_string(),
    ///     "lower_snake_case"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_lower_snake()
    ///         .to_string(),
    ///     "upper_camel_case"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_lower_snake()
    ///         .to_string(),
    ///     "upper_hybrid_case"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_lower_snake()
    ///         .to_string(),
    ///     "upper_kebab_case"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_lower_snake()
    ///         .to_string(),
    ///     "upper_snake_case"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// This form *WILL* attempt to validate the first character to ensure it
    /// could be considered a valid identifier.
    ///
    /// ```
    /// # use typed_ident::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .as_lower_snake()
    ///         .to_string(),
    ///     "_"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .as_lower_snake()
    ///         .to_string(),
    ///     "_2_example_camel"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_snake(&self) -> LowerSnake<'_>;

    /// Returns a displayable type that converts the provided input to lower
    /// snake in decorated form.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`fmt`]: crate::fmt
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::fmt::*;
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
impl<I: Identifier + ?Sized> AsLowerSnake for I {
    #[inline]
    fn as_lower_snake(&self) -> LowerSnake<'_> {
        LowerSnake(Canonical::new::<I>(self.as_str(), '_'))
    }
    #[inline]
    fn as_lower_snake_decorated(&self) -> LowerSnakeDecorated<'_> {
        LowerSnakeDecorated(Decorated::new::<I>(self.as_str(), '_'))
    }
}
