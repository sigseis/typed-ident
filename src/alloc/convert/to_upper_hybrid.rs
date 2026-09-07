// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Ident;
use crate::core::fmt::AsUpperHybrid;
use crate::syntax::boundary::Boundary;
use crate::syntax::boundary::options::{Default, Options};
use crate::syntax::delimiter::{AsciiFlatLine, Delimiter};
use crate::syntax::profile::Profile;
use core::fmt::Write;
use std_alloc::string::String;

// =============================================================================
// TRAITS
// =============================================================================

/// Converts an identifier into a preset upper-hybrid identifier string.
///
/// See the [`convert`] module for more details.
///
/// [`convert`]: crate::alloc::convert
///
/// # Examples
///
/// ```
/// # use typed_ident::alloc::convert::*;
/// # use typed_ident::presets::unicode::*;
/// # use typed_ident::syntax::delimiter::*;
/// assert_eq!(
///     LowerCamelIdent::new("lowerCamel")?.to_upper_hybrid_canonical(AsciiFlatLine::LowLine),
///     "LowerCamel"
/// );
/// assert_eq!(
///     LowerHybridIdent::new("lowerHybrid")?.to_upper_hybrid_canonical(AsciiFlatLine::LowLine),
///     "LowerHybrid"
/// );
/// assert_eq!(
///     LowerKebabIdent::new("lower-kebab")?.to_upper_hybrid_canonical(AsciiFlatLine::LowLine),
///     "LowerKebab"
/// );
/// assert_eq!(
///     LowerSnakeIdent::new("lower_snake")?.to_upper_hybrid_canonical(AsciiFlatLine::LowLine),
///     "LowerSnake"
/// );
/// assert_eq!(
///     UpperCamelIdent::new("UpperCamel")?.to_upper_hybrid_canonical(AsciiFlatLine::LowLine),
///     "UpperCamel"
/// );
/// assert_eq!(
///     UpperHybridIdent::new("UpperHybrid")?.to_upper_hybrid_canonical(AsciiFlatLine::LowLine),
///     "UpperHybrid"
/// );
/// assert_eq!(
///     UpperKebabIdent::new("UPPER-KEBAB")?.to_upper_hybrid_canonical(AsciiFlatLine::LowLine),
///     "UpperKebab"
/// );
/// assert_eq!(
///     UpperSnakeIdent::new("UPPER_SNAKE")?.to_upper_hybrid_canonical(AsciiFlatLine::LowLine),
///     "UpperSnake"
/// );
/// # Ok::<(), typed_ident::Error>(())
/// ```
pub trait ToUpperHybrid {
    /// Returns a displayable type that converts the provided input to upper
    /// hybrid in canonical form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_upper_hybrid_canonical_opts`].
    ///
    /// This method additionally takes a `default_delim` parameter, which is
    /// to clarify which of the two delimiters should be produces if the
    /// formatting operation needs to produces a delimiter.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`to_upper_hybrid_canonical_opts`]: Self::to_upper_hybrid_canonical_opts
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_upper_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "LowerCamelCase"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_upper_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "LowerHybridCase"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_upper_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "LowerKebabCase"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_upper_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "LowerSnakeCase"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_upper_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "UpperCamelCase"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_upper_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "UpperHybridCase"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_upper_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "UpperKebabCase"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_upper_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "UpperSnakeCase"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Notice that it will attempt to keep necessary prefix delimiters.
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .to_upper_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "_"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .to_upper_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "_2ExampleCamel"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_hybrid_canonical(&self, default_delim: AsciiFlatLine) -> String {
        self.to_upper_hybrid_canonical_opts::<Default>(default_delim)
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
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     UpperHybridIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .to_upper_hybrid_canonical_opts::<AllBoundaries>(AsciiFlatLine::LowLine),
    ///     "Abc123HttpDevServer" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperHybridIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .to_upper_hybrid_canonical_opts::<Default>(AsciiFlatLine::LowLine),
    ///     "Abc_123HttpDevServer" // Digits aren't recognized as boundaries.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Transforming to a less-bounded policy:
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     UpperHybridIdent::new("__HttpDEVServer__")?
    ///         .to_upper_hybrid_canonical_opts::<NoBoundaries>(AsciiFlatLine::LowLine),
    ///     "Http_Dev_Server" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperHybridIdent::new("__HttpDEVServer__")?
    ///         .to_upper_hybrid_canonical_opts::<Default>(AsciiFlatLine::LowLine),
    ///     "HttpDevServer" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_hybrid_canonical_opts<O: Options>(&self, default_delim: AsciiFlatLine) -> String;

    /// Returns a displayable type that converts the provided input to upper
    /// hybrid in decorated form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_upper_hybrid_decorated_opts`].
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
    /// [`to_upper_hybrid_decorated_opts`]: Self::to_upper_hybrid_decorated_opts
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_upper_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     "__Lower__CamelCase__"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_upper_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     "_-Lower-_HybridCase-_"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_upper_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     "--Lower--KebabCase--"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_upper_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     "__Lower__SnakeCase__"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_upper_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     "__Upper__CamelCase__"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_upper_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     "-_Upper_-HybridCase_-"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_upper_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     "--Upper--KebabCase--"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_upper_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     "__Upper__SnakeCase__"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_hybrid_decorated(&self, default_delim: AsciiFlatLine) -> String {
        self.to_upper_hybrid_decorated_opts::<Default>(default_delim)
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
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     UpperHybridIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .to_upper_hybrid_decorated_opts::<AllBoundaries>(AsciiFlatLine::LowLine),
    ///     "__Abc123HttpDevServer__" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperHybridIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .to_upper_hybrid_decorated_opts::<Default>(AsciiFlatLine::LowLine),
    ///     "__Abc_123HttpDevServer__" // Digits aren't recognized as boundaries.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Transforming to a less-bounded policy:
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     UpperHybridIdent::new("__HttpDEVServer__")?
    ///         .to_upper_hybrid_decorated_opts::<NoBoundaries>(AsciiFlatLine::LowLine),
    ///     "__Http_Dev_Server__" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperHybridIdent::new("__HttpDEVServer__")?
    ///         .to_upper_hybrid_decorated_opts::<Default>(AsciiFlatLine::LowLine),
    ///     "__HttpDevServer__" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_hybrid_decorated_opts<O: Options>(&self, default_delim: AsciiFlatLine) -> String;

    /// Returns a displayable type that converts the provided input to upper
    /// hybrid in delimited form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_upper_hybrid_delimited_opts`].
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
    /// [`to_upper_hybrid_delimited_opts`]: Self::to_upper_hybrid_delimited_opts
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_upper_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     "__Lower__Camel_Case__"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_upper_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     "_-Lower-_Hybrid-Case-_"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_upper_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     "--Lower--Kebab-Case--"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_upper_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     "__Lower__Snake_Case__"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_upper_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     "__Upper__Camel_Case__"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_upper_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     "-_Upper_-Hybrid_Case_-"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_upper_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     "--Upper--Kebab-Case--"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_upper_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     "__Upper__Snake_Case__"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_hybrid_delimited(&self, default_delim: AsciiFlatLine) -> String {
        self.to_upper_hybrid_delimited_opts::<Default>(default_delim)
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
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     UpperHybridIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .to_upper_hybrid_delimited_opts::<AllBoundaries>(AsciiFlatLine::LowLine),
    ///     "__Abc_123_HttpDevServer__" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to (in this case identical)...
    /// assert_eq!(
    ///     UpperHybridIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .to_upper_hybrid_delimited_opts::<Default>(AsciiFlatLine::LowLine),
    ///     "__Abc_123_HttpDevServer__" // Digits aren't recognized as boundaries.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Transforming to a less-bounded policy:
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     UpperHybridIdent::new("__HttpDEVServer__")?
    ///         .to_upper_hybrid_delimited_opts::<NoBoundaries>(AsciiFlatLine::LowLine),
    ///     "__Http_Dev_Server__" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperHybridIdent::new("__HttpDEVServer__")?
    ///         .to_upper_hybrid_delimited_opts::<Default>(AsciiFlatLine::LowLine),
    ///     "__HttpDevServer__" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_hybrid_delimited_opts<O: Options>(&self, default_delim: AsciiFlatLine) -> String;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> ToUpperHybrid for Ident<B, D, P> {
    #[inline]
    fn to_upper_hybrid_canonical_opts<O: Options>(&self, default_delim: AsciiFlatLine) -> String {
        let mut string = String::with_capacity(self.len());
        write!(
            string,
            "{}",
            self.as_upper_hybrid_canonical_opts::<O>(default_delim)
        )
        .expect("failed to format an identifier into a string");
        string
    }
    #[inline]
    fn to_upper_hybrid_decorated_opts<O: Options>(&self, default_delim: AsciiFlatLine) -> String {
        let mut string = String::with_capacity(self.len());
        write!(
            string,
            "{}",
            self.as_upper_hybrid_decorated_opts::<O>(default_delim)
        )
        .expect("failed to format an identifier into a string");
        string
    }
    #[inline]
    fn to_upper_hybrid_delimited_opts<O: Options>(&self, default_delim: AsciiFlatLine) -> String {
        let mut string = String::with_capacity(self.len());
        write!(
            string,
            "{}",
            self.as_upper_hybrid_delimited_opts::<O>(default_delim)
        )
        .expect("failed to format an identifier into a string");
        string
    }
}
