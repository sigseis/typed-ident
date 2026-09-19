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

/// Provides methods for converting an identifier to upper-hybrid format, using
/// plain, canonical, decorated, or delimited forms.
///
/// See the [`convert`] module for more details.
///
/// <div class="warning">
///
/// **NOTE:** These traits are currently being reconsidered. See issue [#17](https://github.com/sigseis/typed-ident/issues/17).
///
/// </div>
///
/// # Default Delimiter
///
/// These method additionally takes a `default_delim` parameter, which is
/// to clarify which of the two delimiters should be produced if the
/// conversion operation needs to produce a delimiter.
///
/// Some forms will attempt to keep delimiters from the source text in certain
/// scenarios (such as the decorated and delimited forms). In these cases, the
/// formatter will attempt to keep the original source delimiter. If it is not
/// a valid hybrid-ident delimiter, then it will be mapped to the provided
/// `default_delim`.
///
/// [`convert`]: crate::alloc::convert
pub trait ToUpperHybrid {
    /// Returns a string of the provided input converted to upper hybrid plain
    /// form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_upper_hybrid_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`to_upper_hybrid_opts`]: Self::to_upper_hybrid_opts
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
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     "LowerCamelCase"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     "LowerHybridCase"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     "LowerKebabCase"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     "LowerSnakeCase"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     "UpperCamelCase"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     "UpperHybridCase"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     "UpperKebabCase"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     "UpperSnakeCase"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// This form will *NOT* attempt to validate the first character to ensure
    /// it could be considered a valid identifier (see
    /// [`to_upper_hybrid_canonical`] for a version that would validate this).
    ///
    /// [`to_upper_hybrid_canonical`]: Self::to_upper_hybrid_canonical
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     ""
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     "2ExampleCamel"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_hybrid(&self, default_delim: AsciiFlatLine) -> String {
        self.to_upper_hybrid_opts::<Default>(default_delim)
    }

    /// Returns a string of the provided input converted to upper hybrid plain
    /// form.
    ///
    /// Use this method if you want to transform the boundary policy of the
    /// input string, or if you want to persist the same customized policy (in
    /// place of simply using the default).
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
    ///         .to_upper_hybrid_opts::<AllBoundaries>(AsciiFlatLine::LowLine),
    ///     "Abc123HttpDevServer" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperHybridIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .to_upper_hybrid_opts::<Default>(AsciiFlatLine::LowLine),
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
    ///         .to_upper_hybrid_opts::<NoBoundaries>(AsciiFlatLine::LowLine),
    ///     "Http_Dev_Server" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperHybridIdent::new("__HttpDEVServer__")?
    ///         .to_upper_hybrid_opts::<Default>(AsciiFlatLine::LowLine),
    ///     "HttpDevServer" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_hybrid_opts<O: Options>(&self, default_delim: AsciiFlatLine) -> String;

    /// Returns a string of the provided input converted to upper hybrid
    /// canonical form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_upper_hybrid_canonical_opts`].
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
    /// This form *WILL* attempt to validate the first character to ensure it
    /// could be considered a valid identifier (see [`to_upper_hybrid`] for a
    /// version that would *NOT* validate this).
    ///
    /// [`to_upper_hybrid`]: Self::to_upper_hybrid
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

    /// Returns a string of the provided input converted to upper hybrid
    /// canonical form.
    ///
    /// Use this method if you want to transform the boundary policy of the
    /// input string, or if you want to persist the same customized policy (in
    /// place of simply using the default).
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

    /// Returns a string of the provided input converted to upper hybrid
    /// decorated form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_upper_hybrid_decorated_opts`].
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

    /// Returns a string of the provided input converted to upper hybrid
    /// decorated form.
    ///
    /// Use this method if you want to transform the boundary policy of the
    /// input string, or if you want to persist the same customized policy (in
    /// place of simply using the default).
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

    /// Returns a string of the provided input converted to upper hybrid
    /// delimited form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_upper_hybrid_delimited_opts`].
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

    /// Returns a string of the provided input converted to upper hybrid
    /// delimited form.
    ///
    /// Use this method if you want to transform the boundary policy of the
    /// input string, or if you want to persist the same customized policy (in
    /// place of simply using the default).
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
    fn to_upper_hybrid_opts<O: Options>(&self, default_delim: AsciiFlatLine) -> String {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_upper_hybrid_opts::<O>(default_delim))
            .expect("failed to format an identifier into a string");
        string
    }
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
