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
    name=UpperHybridCanonical,
    over=Canonical,
    upper=true,
    docs=concat!(
        "A `Display` type for the [`AsUpperHybrid`] trait in canonical format.",
        "\n\n",
        "This type is constructed by calling one of the [`as_upper_hybrid_canonical`] methods.",
        "\n\n",
        "[`as_upper_hybrid_canonical`]: AsUpperHybrid::as_upper_hybrid_canonical"
    ),
}

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=UpperHybridDecorated,
    over=Decorated,
    upper=true,
    docs=concat!(
        "A `Display` type for the [`AsUpperHybrid`] trait in decorated format.",
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
        "A `Display` type for the [`AsUpperHybrid`] trait in delimited format.",
        "\n\n",
        "This type is constructed by calling one of the [`as_upper_hybrid_delimited`] methods.",
        "\n\n",
        "[`as_upper_hybrid_delimited`]: AsUpperHybrid::as_upper_hybrid_delimited"
    ),
}

// =============================================================================
// TRAIT
// =============================================================================

/// Formats an identifier as a preset upper-hybrid identifier.
///
/// See the [`fmt`] module for more details.
///
/// [`fmt`]: crate::core::fmt
pub trait AsUpperHybrid {
    /// Returns a displayable type that converts the provided input to upper
    /// hybrid in canonical form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`as_upper_hybrid_canonical_opts`].
    ///
    /// This method additionally takes a `default_delim` parameter, which is
    /// to clarify which of the two delimiters should be produces if the
    /// formatting operation needs to produces a delimiter.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`as_upper_hybrid_canonical_opts`]: Self::as_upper_hybrid_canonical_opts
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
    ///         .as_upper_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "LowerCamelCase"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_upper_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "LowerHybridCase"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_upper_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "LowerKebabCase"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_upper_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "LowerSnakeCase"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_upper_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "UpperCamelCase"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_upper_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "UpperHybridCase"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_upper_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "UpperKebabCase"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_upper_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "UpperSnakeCase"
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
    ///         .as_upper_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "_"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .as_upper_hybrid_canonical(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "_2ExampleCamel"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_hybrid_canonical(&self, default_delim: AsciiFlatLine) -> UpperHybridCanonical<'_> {
        self.as_upper_hybrid_canonical_opts::<Default>(default_delim)
    }

    /// Returns a displayable type that converts the provided input to upper
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
    ///     UpperHybridIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .as_upper_hybrid_canonical_opts::<AllBoundaries>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "Abc123HttpDevServer" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperHybridIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .as_upper_hybrid_canonical_opts::<Default>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "Abc_123HttpDevServer" // Digits aren't recognized as boundaries.
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
    ///     UpperHybridIdent::new("__HttpDEVServer__")?
    ///         .as_upper_hybrid_canonical_opts::<NoBoundaries>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "Http_Dev_Server" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperHybridIdent::new("__HttpDEVServer__")?
    ///         .as_upper_hybrid_canonical_opts::<Default>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "HttpDevServer" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_hybrid_canonical_opts<O: Options>(
        &self,
        default_delim: AsciiFlatLine,
    ) -> UpperHybridCanonical<'_>;

    /// Returns a displayable type that converts the provided input to upper
    /// hybrid in decorated form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`as_upper_hybrid_decorated_opts`].
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
    /// [`as_upper_hybrid_decorated_opts`]: Self::as_upper_hybrid_decorated_opts
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
    #[inline]
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_hybrid_decorated(&self, default_delim: AsciiFlatLine) -> UpperHybridDecorated<'_> {
        self.as_upper_hybrid_decorated_opts::<Default>(default_delim)
    }

    /// Returns a displayable type that converts the provided input to upper
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
    ///     UpperHybridIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .as_upper_hybrid_decorated_opts::<AllBoundaries>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__Abc123HttpDevServer__" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperHybridIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .as_upper_hybrid_decorated_opts::<Default>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__Abc_123HttpDevServer__" // Digits aren't recognized as boundaries.
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
    ///     UpperHybridIdent::new("__HttpDEVServer__")?
    ///         .as_upper_hybrid_decorated_opts::<NoBoundaries>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__Http_Dev_Server__" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperHybridIdent::new("__HttpDEVServer__")?
    ///         .as_upper_hybrid_decorated_opts::<Default>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__HttpDevServer__" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_hybrid_decorated_opts<O: Options>(
        &self,
        default_delim: AsciiFlatLine,
    ) -> UpperHybridDecorated<'_>;

    /// Returns a displayable type that converts the provided input to upper
    /// hybrid in delimited form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`as_upper_hybrid_delimited_opts`].
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
    /// [`as_upper_hybrid_delimited_opts`]: Self::as_upper_hybrid_delimited_opts
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
    fn as_upper_hybrid_delimited(&self, default_delim: AsciiFlatLine) -> UpperHybridDelimited<'_> {
        self.as_upper_hybrid_delimited_opts::<Default>(default_delim)
    }

    /// Returns a displayable type that converts the provided input to upper
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
    ///     UpperHybridIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .as_upper_hybrid_delimited_opts::<AllBoundaries>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__Abc_123_HttpDevServer__" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to (in this case identical)...
    /// assert_eq!(
    ///     UpperHybridIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .as_upper_hybrid_delimited_opts::<Default>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__Abc_123_HttpDevServer__" // Digits aren't recognized as boundaries.
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
    ///     UpperHybridIdent::new("__HttpDEVServer__")?
    ///         .as_upper_hybrid_delimited_opts::<NoBoundaries>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__Http_Dev_Server__" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperHybridIdent::new("__HttpDEVServer__")?
    ///         .as_upper_hybrid_delimited_opts::<Default>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "__HttpDevServer__" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_hybrid_delimited_opts<O: Options>(
        &self,
        default_delim: AsciiFlatLine,
    ) -> UpperHybridDelimited<'_>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> AsUpperHybrid for Ident<B, D, P> {
    #[inline]
    fn as_upper_hybrid_canonical_opts<O: Options>(
        &self,
        default_delim: AsciiFlatLine,
    ) -> UpperHybridCanonical<'_> {
        UpperHybridCanonical(Canonical::new::<B, D, P::BaseProfile, Standard<O>>(
            self.as_str(),
            default_delim.as_char(),
        ))
    }
    #[inline]
    fn as_upper_hybrid_decorated_opts<O: Options>(
        &self,
        default_delim: AsciiFlatLine,
    ) -> UpperHybridDecorated<'_> {
        UpperHybridDecorated(Decorated::new::<B, D, P::BaseProfile, Standard<O>>(
            self.as_str(),
            default_delim.as_char(),
            Some(match default_delim {
                AsciiFlatLine::LowLine => '-',
                AsciiFlatLine::HyphenMinus => '_',
            }),
        ))
    }
    #[inline]
    fn as_upper_hybrid_delimited_opts<O: Options>(
        &self,
        default_delim: AsciiFlatLine,
    ) -> UpperHybridDelimited<'_> {
        UpperHybridDelimited(Delimited::new::<B, D, P::BaseProfile, Standard<O>>(
            self.as_str(),
            default_delim.as_char(),
            Some(match default_delim {
                AsciiFlatLine::LowLine => '-',
                AsciiFlatLine::HyphenMinus => '_',
            }),
        ))
    }
}
