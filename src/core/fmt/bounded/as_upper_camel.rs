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
    name=UpperCamel,
    over=Canonical,
    upper=true,
    docs=concat!(
        "A `Display` type for the [`AsUpperCamel`] trait in canonical form.",
        "\n\n",
        "This type is constructed by calling one of the [`as_upper_camel`] methods.",
        "\n\n",
        "[`as_upper_camel`]: AsUpperCamel::as_upper_camel"
    ),
}

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=UpperCamelDecorated,
    over=Decorated,
    upper=true,
    docs=concat!(
        "A `Display` type for the [`AsUpperCamel`] trait in decorated form.",
        "\n\n",
        "This type is constructed by calling one of the [`as_upper_camel_decorated`] methods.",
        "\n\n",
        "[`as_upper_camel_decorated`]: AsUpperCamel::as_upper_camel_decorated"
    ),
}

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=UpperCamelDelimited,
    over=Delimited,
    upper=true,
    docs=concat!(
        "A `Display` type for the [`AsUpperCamel`] trait in delimited form.",
        "\n\n",
        "This type is constructed by calling one of the [`as_upper_camel_delimited`] methods.",
        "\n\n",
        "[`as_upper_camel_delimited`]: AsUpperCamel::as_upper_camel_delimited"
    ),
}

// =============================================================================
// TRAIT
// =============================================================================

/// Provides methods for formatting an identifier in upper-camel format, using
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
pub trait AsUpperCamel {
    /// Returns a displayable type that converts the provided input to upper
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
    ///         .as_upper_camel()
    ///         .to_string(),
    ///     "LowerCamelCase"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_upper_camel()
    ///         .to_string(),
    ///     "LowerHybridCase"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_upper_camel()
    ///         .to_string(),
    ///     "LowerKebabCase"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_upper_camel()
    ///         .to_string(),
    ///     "LowerSnakeCase"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_upper_camel()
    ///         .to_string(),
    ///     "UpperCamelCase"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_upper_camel()
    ///         .to_string(),
    ///     "UpperHybridCase"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_upper_camel()
    ///         .to_string(),
    ///     "UpperKebabCase"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_upper_camel()
    ///         .to_string(),
    ///     "UpperSnakeCase"
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
    ///         .as_upper_camel()
    ///         .to_string(),
    ///     "_"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .as_upper_camel()
    ///         .to_string(),
    ///     "_2ExampleCamel"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_camel(&self) -> UpperCamel<'_>;

    /// Returns a displayable type that converts the provided input to upper
    /// camel in decorated form.
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
    ///         .as_upper_camel_decorated()
    ///         .to_string(),
    ///     "__Lower__CamelCase__"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_upper_camel_decorated()
    ///         .to_string(),
    ///     "__Lower__HybridCase__"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_upper_camel_decorated()
    ///         .to_string(),
    ///     "__Lower__KebabCase__"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_upper_camel_decorated()
    ///         .to_string(),
    ///     "__Lower__SnakeCase__"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_upper_camel_decorated()
    ///         .to_string(),
    ///     "__Upper__CamelCase__"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_upper_camel_decorated()
    ///         .to_string(),
    ///     "__Upper__HybridCase__"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_upper_camel_decorated()
    ///         .to_string(),
    ///     "__Upper__KebabCase__"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_upper_camel_decorated()
    ///         .to_string(),
    ///     "__Upper__SnakeCase__"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_camel_decorated(&self) -> UpperCamelDecorated<'_>;

    /// Returns a displayable type that converts the provided input to upper
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
    ///         .as_upper_camel_delimited()
    ///         .to_string(),
    ///     "__Lower__Camel_Case__"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_upper_camel_delimited()
    ///         .to_string(),
    ///     "__Lower__Hybrid_Case__"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_upper_camel_delimited()
    ///         .to_string(),
    ///     "__Lower__Kebab_Case__"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_upper_camel_delimited()
    ///         .to_string(),
    ///     "__Lower__Snake_Case__"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_upper_camel_delimited()
    ///         .to_string(),
    ///     "__Upper__Camel_Case__"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_upper_camel_delimited()
    ///         .to_string(),
    ///     "__Upper__Hybrid_Case__"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_upper_camel_delimited()
    ///         .to_string(),
    ///     "__Upper__Kebab_Case__"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_upper_camel_delimited()
    ///         .to_string(),
    ///     "__Upper__Snake_Case__"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_camel_delimited(&self) -> UpperCamelDelimited<'_>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<I: Identifier + ?Sized> AsUpperCamel for I {
    #[inline]
    fn as_upper_camel(&self) -> UpperCamel<'_> {
        UpperCamel(Canonical::new::<I, Standard>(self.as_str(), '_'))
    }
    #[inline]
    fn as_upper_camel_decorated(&self) -> UpperCamelDecorated<'_> {
        UpperCamelDecorated(Decorated::new::<I, Standard>(self.as_str(), '_', None))
    }
    #[inline]
    fn as_upper_camel_delimited(&self) -> UpperCamelDelimited<'_> {
        UpperCamelDelimited(Delimited::new::<I, Standard>(self.as_str(), '_', None))
    }
}
