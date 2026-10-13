// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Identifier;
use crate::fmt::bounded::{Canonical, Decorated, Delimited};
use crate::syntax::boundary::Standard;
use crate::syntax::delimiter::{AsciiFlatLine, Delimiter};

// =============================================================================
// TYPEDEFS
// =============================================================================

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=UpperHybrid,
    over=Canonical,
    upper=true,
    docs=concat!(
        "A `Display` type for the [`AsUpperHybrid`] trait in canonical form.",
        "\n\n",
        "This type is constructed by calling one of the [`as_upper_hybrid`] methods.",
        "\n\n",
        "[`as_upper_hybrid`]: AsUpperHybrid::as_upper_hybrid"
    ),
}

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=UpperHybridDecorated,
    over=Decorated,
    upper=true,
    docs=concat!(
        "A `Display` type for the [`AsUpperHybrid`] trait in decorated form.",
        "\n\n",
        "This type is constructed by calling one of the [`as_upper_hybrid_decorated`] methods.",
        "\n\n",
        "[`as_upper_hybrid_decorated`]: AsUpperHybrid::as_upper_hybrid_decorated"
    ),
}

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=UpperHybridDelimited,
    over=Delimited,
    upper=true,
    docs=concat!(
        "A `Display` type for the [`AsUpperHybrid`] trait in delimited form.",
        "\n\n",
        "This type is constructed by calling one of the [`as_upper_hybrid_delimited`] methods.",
        "\n\n",
        "[`as_upper_hybrid_delimited`]: AsUpperHybrid::as_upper_hybrid_delimited"
    ),
}

// =============================================================================
// TRAIT
// =============================================================================

/// Provides methods for formatting an identifier in upper-hybrid format, using
/// canonical, decorated, or delimited forms.
///
/// See the [`fmt`] module for more details.
///
/// # Default Delimiter
///
/// These method additionally takes a `default_delim` parameter, which is
/// to clarify which of the two delimiters should be produced if the
/// formatting operation needs to produce a delimiter.
///
/// Some forms will attempt to keep delimiters from the source text in certain
/// scenarios (such as the decorated and delimited forms). In these cases, the
/// formatter will attempt to keep the original source delimiter. If it is not
/// a valid hybrid-ident delimiter, then it will be mapped to the provided
/// `default_delim`.
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
pub trait AsUpperHybrid {
    /// Returns a displayable type that converts the provided input to upper
    /// hybrid in canonical form.
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
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .as_upper_hybrid(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "LowerCamelCase"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_upper_hybrid(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "LowerHybridCase"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_upper_hybrid(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "LowerKebabCase"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_upper_hybrid(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "LowerSnakeCase"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_upper_hybrid(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "UpperCamelCase"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_upper_hybrid(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "UpperHybridCase"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_upper_hybrid(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "UpperKebabCase"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_upper_hybrid(AsciiFlatLine::LowLine)
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
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .as_upper_hybrid(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "_"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .as_upper_hybrid(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "_2ExampleCamel"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_hybrid(&self, default_delim: AsciiFlatLine) -> UpperHybrid<'_>;

    /// Returns a displayable type that converts the provided input to upper
    /// hybrid in decorated form.
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
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .as_upper_hybrid_decorated(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__Lower__CamelCase__"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_upper_hybrid_decorated(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "_-Lower-_HybridCase-_"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_upper_hybrid_decorated(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "--Lower--KebabCase--"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_upper_hybrid_decorated(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__Lower__SnakeCase__"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_upper_hybrid_decorated(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__Upper__CamelCase__"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_upper_hybrid_decorated(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "-_Upper_-HybridCase_-"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_upper_hybrid_decorated(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "--Upper--KebabCase--"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_upper_hybrid_decorated(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__Upper__SnakeCase__"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_hybrid_decorated(&self, default_delim: AsciiFlatLine) -> UpperHybridDecorated<'_>;

    /// Returns a displayable type that converts the provided input to upper
    /// hybrid in delimited form.
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
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .as_upper_hybrid_delimited(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__Lower__Camel_Case__"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_upper_hybrid_delimited(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "_-Lower-_Hybrid-Case-_"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_upper_hybrid_delimited(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "--Lower--Kebab-Case--"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_upper_hybrid_delimited(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__Lower__Snake_Case__"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_upper_hybrid_delimited(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__Upper__Camel_Case__"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_upper_hybrid_delimited(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "-_Upper_-Hybrid_Case_-"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_upper_hybrid_delimited(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "--Upper--Kebab-Case--"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_upper_hybrid_delimited(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__Upper__Snake_Case__"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_hybrid_delimited(&self, default_delim: AsciiFlatLine) -> UpperHybridDelimited<'_>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<I: Identifier + ?Sized> AsUpperHybrid for I {
    #[inline]
    fn as_upper_hybrid(&self, default_delim: AsciiFlatLine) -> UpperHybrid<'_> {
        UpperHybrid(Canonical::new::<I, Standard>(
            self.as_str(),
            default_delim.as_char(),
        ))
    }
    #[inline]
    fn as_upper_hybrid_decorated(&self, default_delim: AsciiFlatLine) -> UpperHybridDecorated<'_> {
        UpperHybridDecorated(Decorated::new::<I, Standard>(
            self.as_str(),
            default_delim.as_char(),
            Some(match default_delim {
                AsciiFlatLine::LowLine => '-',
                AsciiFlatLine::HyphenMinus => '_',
            }),
        ))
    }
    #[inline]
    fn as_upper_hybrid_delimited(&self, default_delim: AsciiFlatLine) -> UpperHybridDelimited<'_> {
        UpperHybridDelimited(Delimited::new::<I, Standard>(
            self.as_str(),
            default_delim.as_char(),
            Some(match default_delim {
                AsciiFlatLine::LowLine => '-',
                AsciiFlatLine::HyphenMinus => '_',
            }),
        ))
    }
}
