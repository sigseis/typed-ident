// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Identifier;
use crate::fmt::bounded::{Canonical, Decorated, Delimited};
use crate::syntax::boundary::Standard;

// =============================================================================
// TYPEDEFS
// =============================================================================

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=LowerCamel,
    over=Canonical,
    upper=false,
    docs=concat!(
        "A `Display` type for the [`AsLowerCamel`] trait in canonical form.",
        "\n\n",
        "This type is constructed by calling one of the [`as_lower_camel`] methods.",
        "\n\n",
        "[`as_lower_camel`]: AsLowerCamel::as_lower_camel"
    ),
}

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=LowerCamelDecorated,
    over=Decorated,
    upper=false,
    docs=concat!(
        "A `Display` type for the [`AsLowerCamel`] trait in decorated form.",
        "\n\n",
        "This type is constructed by calling one of the [`as_lower_camel_decorated`] methods.",
        "\n\n",
        "[`as_lower_camel_decorated`]: AsLowerCamel::as_lower_camel_decorated"
    ),
}

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=LowerCamelDelimited,
    over=Delimited,
    upper=false,
    docs=concat!(
        "A `Display` type for the [`AsLowerCamel`] trait in delimited form.",
        "\n\n",
        "This type is constructed by calling one of the [`as_lower_camel_delimited`] methods.",
        "\n\n",
        "[`as_lower_camel_delimited`]: AsLowerCamel::as_lower_camel_delimited"
    ),
}

// =============================================================================
// TRAIT
// =============================================================================

/// Provides methods for formatting an identifier in lower-camel format, using
/// canonical, decorated, or delimited forms.
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
pub trait AsLowerCamel {
    /// Returns a displayable type that converts the provided input to lower
    /// camel in canonical form.
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
    ///         .as_lower_camel()
    ///         .to_string(),
    ///     "lowerCamelCase"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_lower_camel()
    ///         .to_string(),
    ///     "lowerHybridCase"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_lower_camel()
    ///         .to_string(),
    ///     "lowerKebabCase"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_lower_camel()
    ///         .to_string(),
    ///     "lowerSnakeCase"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_lower_camel()
    ///         .to_string(),
    ///     "upperCamelCase"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_lower_camel()
    ///         .to_string(),
    ///     "upperHybridCase"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_lower_camel()
    ///         .to_string(),
    ///     "upperKebabCase"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_lower_camel()
    ///         .to_string(),
    ///     "upperSnakeCase"
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
    ///         .as_lower_camel()
    ///         .to_string(),
    ///     "_"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .as_lower_camel()
    ///         .to_string(),
    ///     "_2ExampleCamel"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_camel(&self) -> LowerCamel<'_>;

    /// Returns a displayable type that converts the provided input to lower
    /// camel in decorated form.
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
    ///         .as_lower_camel_decorated()
    ///         .to_string(),
    ///     "__lower__camelCase__"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_lower_camel_decorated()
    ///         .to_string(),
    ///     "__lower__hybridCase__"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_lower_camel_decorated()
    ///         .to_string(),
    ///     "__lower__kebabCase__"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_lower_camel_decorated()
    ///         .to_string(),
    ///     "__lower__snakeCase__"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_lower_camel_decorated()
    ///         .to_string(),
    ///     "__upper__camelCase__"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_lower_camel_decorated()
    ///         .to_string(),
    ///     "__upper__hybridCase__"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_lower_camel_decorated()
    ///         .to_string(),
    ///     "__upper__kebabCase__"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_lower_camel_decorated()
    ///         .to_string(),
    ///     "__upper__snakeCase__"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_camel_decorated(&self) -> LowerCamelDecorated<'_>;

    /// Returns a displayable type that converts the provided input to lower
    /// camel in delimited form.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`fmt`]: crate::fmt
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .as_lower_camel_delimited()
    ///         .to_string(),
    ///     "__lower__camel_case__"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_lower_camel_delimited()
    ///         .to_string(),
    ///     "__lower__hybrid_case__"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_lower_camel_delimited()
    ///         .to_string(),
    ///     "__lower__kebab_case__"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_lower_camel_delimited()
    ///         .to_string(),
    ///     "__lower__snake_case__"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_lower_camel_delimited()
    ///         .to_string(),
    ///     "__upper__camel_case__"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_lower_camel_delimited()
    ///         .to_string(),
    ///     "__upper__hybrid_case__"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_lower_camel_delimited()
    ///         .to_string(),
    ///     "__upper__kebab_case__"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_lower_camel_delimited()
    ///         .to_string(),
    ///     "__upper__snake_case__"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_camel_delimited(&self) -> LowerCamelDelimited<'_>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<I: Identifier + ?Sized> AsLowerCamel for I {
    #[inline]
    fn as_lower_camel(&self) -> LowerCamel<'_> {
        LowerCamel(Canonical::new::<I, Standard>(self.as_str(), '_'))
    }
    #[inline]
    fn as_lower_camel_decorated(&self) -> LowerCamelDecorated<'_> {
        LowerCamelDecorated(Decorated::new::<I, Standard>(self.as_str(), '_', None))
    }
    #[inline]
    fn as_lower_camel_delimited(&self) -> LowerCamelDelimited<'_> {
        LowerCamelDelimited(Delimited::new::<I, Standard>(self.as_str(), '_', None))
    }
}
