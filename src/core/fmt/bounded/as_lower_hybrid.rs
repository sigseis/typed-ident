// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::Ident;
use crate::core::fmt::bounded::{Canonical, Decorated, Delimited};
use crate::syntax::boundary::Standard;
use crate::syntax::boundary::options::{Default, Options};
use crate::syntax::delimiter::AsciiFlatLine;
use crate::syntax::{Boundary, Delimiter, Profile};

// =============================================================================
// TYPEDEFS
// =============================================================================

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=LowerHybridCanonical,
    over=Canonical,
    upper=false,
    docs=concat!(
        "A `Display` type for the [`AsLowerHybrid`] trait in canonical format.",
        "\n\n",
        "This type is constructed by calling one of the [`as_lower_hybrid_canonical`] methods.",
        "\n\n",
        "[`as_lower_hybrid_canonical`]: AsLowerHybrid::as_lower_hybrid_canonical"
    ),
}

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=LowerHybridDecorated,
    over=Decorated,
    upper=false,
    docs=concat!(
        "A `Display` type for the [`AsLowerHybrid`] trait in decorated format.",
        "\n\n",
        "This type is constructed by calling one of the [`as_lower_hybrid_decorated`] methods.",
        "\n\n",
        "[`as_lower_hybrid_decorated`]: AsLowerHybrid::as_lower_hybrid_decorated"
    ),
}

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=LowerHybridDelimited,
    over=Delimited,
    upper=false,
    docs=concat!(
        "A `Display` type for the [`AsLowerHybrid`] trait in delimited format.",
        "\n\n",
        "This type is constructed by calling one of the [`as_lower_hybrid_delimited`] methods.",
        "\n\n",
        "[`as_lower_hybrid_delimited`]: AsLowerHybrid::as_lower_hybrid_delimited"
    ),
}

// =============================================================================
// TRAIT
// =============================================================================

/// Formats an identifier as a preset lower-hybrid identifier.
///
/// See the [`fmt`] module for more details.
///
/// [`fmt`]: crate::core::fmt
pub trait AsLowerHybrid {
    /// Returns a displayable type that converts the provided input to lower
    /// hybrid in canonical form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`as_lower_hybrid_canonical_opts`].
    ///
    /// This method additionally takes a `default_delim` parameter, which is
    /// to clarify which of the two delimiters should be produces if the
    /// formatting operation needs to produces a delimiter.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`as_lower_hybrid_canonical_opts`]: Self::as_lower_hybrid_canonical_opts
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .as_lower_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "lowerCamelCase"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_lower_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "lowerHybridCase"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_lower_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "lowerKebabCase"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_lower_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "lowerSnakeCase"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_lower_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "upperCamelCase"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_lower_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "upperHybridCase"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_lower_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "upperKebabCase"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_lower_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "upperSnakeCase"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Notice that it will attempt to keep necessary prefix delimiters.
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .as_lower_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "_"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .as_lower_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "_2ExampleCamel"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_hybrid_canonical(&self, default_delim: AsciiFlatLine) -> LowerHybridCanonical<'_> {
        self.as_lower_hybrid_canonical_opts::<Default>(default_delim)
    }

    /// Returns a displayable type that converts the provided input to lower
    /// hybrid in canonical form, over some provided boundary options.
    ///
    /// Use this method if you want to transform the boundary policy of the
    /// input string, or if you want to persist the same customized policy (in
    /// place of simply using the default).
    ///
    /// This method additionally takes a `default_delim` parameter, which is
    /// to clarify which of the two delimiters should be produces if the
    /// formatting operation needs to produces a delimiter.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// Transforming to a more-bounded policy:
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .as_lower_hybrid_canonical_opts::<AllBoundaries>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "abc123HttpDevServer" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .as_lower_hybrid_canonical_opts::<Default>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "abc_123HttpDevServer" // Digits aren't recognized as boundaries.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Transforming to a less-bounded policy:
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .as_lower_hybrid_canonical_opts::<NoBoundaries>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "http_dev_server" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .as_lower_hybrid_canonical_opts::<Default>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "httpDevServer" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_hybrid_canonical_opts<O: Options>(
        &self,
        default_delim: AsciiFlatLine,
    ) -> LowerHybridCanonical<'_>;

    /// Returns a displayable type that converts the provided input to lower
    /// hybrid in decorated form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`as_lower_hybrid_decorated_opts`].
    ///
    /// This method additionally takes a `default_delim` parameter, which is
    /// to clarify which of the two delimiters should be produces if the
    /// formatting operation needs to produces a delimiter.
    ///
    /// The decorated formatter will attempt to keep the original delimiters.
    /// But it will map delimiters outside of the character set of
    /// `AsciiFlatLine`, and there can be instances where a delimiter is
    /// produced to forcibly separate two chunks on an options difference.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`as_lower_hybrid_decorated_opts`]: Self::as_lower_hybrid_decorated_opts
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .as_lower_hybrid_decorated(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__lower__camelCase__"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_lower_hybrid_decorated(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "_-lower-_hybridCase-_"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_lower_hybrid_decorated(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "--lower--kebabCase--"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_lower_hybrid_decorated(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__lower__snakeCase__"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_lower_hybrid_decorated(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__upper__camelCase__"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_lower_hybrid_decorated(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "-_upper_-hybridCase_-"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_lower_hybrid_decorated(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "--upper--kebabCase--"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_lower_hybrid_decorated(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__upper__snakeCase__"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_hybrid_decorated(&self, default_delim: AsciiFlatLine) -> LowerHybridDecorated<'_> {
        self.as_lower_hybrid_decorated_opts::<Default>(default_delim)
    }

    /// Returns a displayable type that converts the provided input to lower
    /// hybrid in decorated form, over some provided boundary options.
    ///
    /// Use this method if you want to transform the boundary policy of the
    /// input string, or if you want to persist the same customized policy (in
    /// place of simply using the default).
    ///
    /// This method additionally takes a `default_delim` parameter, which is
    /// to clarify which of the two delimiters should be produces if the
    /// formatting operation needs to produces a delimiter.
    ///
    /// The decorated formatter will attempt to keep the original delimiters.
    /// But it will map delimiters outside of the character set of
    /// `AsciiFlatLine`, and there can be instances where a delimiter is
    /// produced to forcibly separate two chunks on an options difference.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`fmt`]: crate::core::fmt
    ///
    /// Transforming to a more-bounded policy:
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .as_lower_hybrid_decorated_opts::<AllBoundaries>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__abc123HttpDevServer__" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .as_lower_hybrid_decorated_opts::<Default>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__abc_123HttpDevServer__" // Digits aren't recognized as boundaries.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Transforming to a less-bounded policy:
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .as_lower_hybrid_decorated_opts::<NoBoundaries>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__http_dev_server__" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .as_lower_hybrid_decorated_opts::<Default>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__httpDevServer__" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_hybrid_decorated_opts<O: Options>(
        &self,
        default_delim: AsciiFlatLine,
    ) -> LowerHybridDecorated<'_>;

    /// Returns a displayable type that converts the provided input to lower
    /// hybrid in delimited form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`as_lower_hybrid_delimited_opts`].
    ///
    /// This method additionally takes a `default_delim` parameter, which is
    /// to clarify which of the two delimiters should be produces if the
    /// formatting operation needs to produces a delimiter.
    ///
    /// The decorated formatter will attempt to keep the original delimiters.
    /// But it will map delimiters outside of the character set of
    /// `AsciiFlatLine`, and there can be instances where a delimiter is
    /// produced to forcibly separate two chunks on an options difference.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`as_lower_hybrid_delimited_opts`]: Self::as_lower_hybrid_delimited_opts
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .as_lower_hybrid_delimited(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__lower__camel_case__"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_lower_hybrid_delimited(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "_-lower-_hybrid-case-_"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_lower_hybrid_delimited(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "--lower--kebab-case--"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_lower_hybrid_delimited(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__lower__snake_case__"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_lower_hybrid_delimited(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__upper__camel_case__"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_lower_hybrid_delimited(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "-_upper_-hybrid_case_-"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_lower_hybrid_delimited(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "--upper--kebab-case--"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_lower_hybrid_delimited(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__upper__snake_case__"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_hybrid_delimited(&self, default_delim: AsciiFlatLine) -> LowerHybridDelimited<'_> {
        self.as_lower_hybrid_delimited_opts::<Default>(default_delim)
    }

    /// Returns a displayable type that converts the provided input to lower
    /// hybrid in delimited form, over some provided boundary options.
    ///
    /// Use this method if you want to transform the boundary policy of the
    /// input string, or if you want to persist the same customized policy (in
    /// place of simply using the default).
    ///
    /// This method additionally takes a `default_delim` parameter, which is
    /// to clarify which of the two delimiters should be produces if the
    /// formatting operation needs to produces a delimiter.
    ///
    /// The decorated formatter will attempt to keep the original delimiters.
    /// But it will map delimiters outside of the character set of
    /// `AsciiFlatLine`, and there can be instances where a delimiter is
    /// produced to forcibly separate two chunks on an options difference.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// Transforming to a more-bounded policy:
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .as_lower_hybrid_delimited_opts::<AllBoundaries>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__abc_123_httpDevServer__" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to (in this case identical)...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .as_lower_hybrid_delimited_opts::<Default>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__abc_123_httpDevServer__" // Digits aren't recognized as boundaries.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Transforming to a less-bounded policy:
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .as_lower_hybrid_delimited_opts::<NoBoundaries>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__http_dev_server__" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .as_lower_hybrid_delimited_opts::<Default>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__httpDevServer__" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_hybrid_delimited_opts<O: Options>(
        &self,
        default_delim: AsciiFlatLine,
    ) -> LowerHybridDelimited<'_>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> AsLowerHybrid for Ident<B, D, P> {
    #[inline]
    fn as_lower_hybrid_canonical_opts<O: Options>(
        &self,
        default_delim: AsciiFlatLine,
    ) -> LowerHybridCanonical<'_> {
        LowerHybridCanonical(Canonical::new::<B, D, P::BaseProfile, Standard<O>>(
            self.as_str(),
            default_delim.as_char(),
        ))
    }
    #[inline]
    fn as_lower_hybrid_decorated_opts<O: Options>(
        &self,
        default_delim: AsciiFlatLine,
    ) -> LowerHybridDecorated<'_> {
        LowerHybridDecorated(Decorated::new::<B, D, P::BaseProfile, Standard<O>>(
            self.as_str(),
            default_delim.as_char(),
            Some(match default_delim {
                AsciiFlatLine::LowLine => '-',
                AsciiFlatLine::HyphenMinus => '_',
            }),
        ))
    }
    #[inline]
    fn as_lower_hybrid_delimited_opts<O: Options>(
        &self,
        default_delim: AsciiFlatLine,
    ) -> LowerHybridDelimited<'_> {
        LowerHybridDelimited(Delimited::new::<B, D, P::BaseProfile, Standard<O>>(
            self.as_str(),
            default_delim.as_char(),
            Some(match default_delim {
                AsciiFlatLine::LowLine => '-',
                AsciiFlatLine::HyphenMinus => '_',
            }),
        ))
    }
}
