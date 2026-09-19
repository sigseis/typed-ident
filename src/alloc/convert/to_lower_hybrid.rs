// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Ident;
use crate::core::fmt::AsLowerHybrid;
use crate::syntax::boundary::Boundary;
use crate::syntax::boundary::options::{Default, Options};
use crate::syntax::delimiter::{AsciiFlatLine, Delimiter};
use crate::syntax::profile::Profile;
use core::fmt::Write;
use std_alloc::string::String;

// =============================================================================
// TRAITS
// =============================================================================

/// Provides methods for converting an identifier to lower-hybrid format, using
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
pub trait ToLowerHybrid {
    /// Returns a string of the provided input converted to lower hybrid plain
    /// form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_lower_hybrid_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`to_lower_hybrid_opts`]: Self::to_lower_hybrid_opts
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
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     "lowerCamelCase"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     "lowerHybridCase"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     "lowerKebabCase"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     "lowerSnakeCase"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     "upperCamelCase"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     "upperHybridCase"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     "upperKebabCase"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     "upperSnakeCase"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// This form will *NOT* attempt to validate the first character to ensure
    /// it could be considered a valid identifier (see
    /// [`to_lower_hybrid_canonical`] for a version that would validate this).
    ///
    /// [`to_lower_hybrid_canonical`]: Self::to_lower_hybrid_canonical
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     ""
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     "2ExampleCamel"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_hybrid(&self, default_delim: AsciiFlatLine) -> String {
        self.to_lower_hybrid_opts::<Default>(default_delim)
    }

    /// Returns a string of the provided input converted to lower hybrid plain
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
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_hybrid_opts::<AllBoundaries>(AsciiFlatLine::LowLine),
    ///     "abc123HttpDevServer" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_hybrid_opts::<Default>(AsciiFlatLine::LowLine),
    ///     "abc_123HttpDevServer" // Digits aren't recognized as boundaries.
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
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .to_lower_hybrid_opts::<NoBoundaries>(AsciiFlatLine::LowLine),
    ///     "http_dev_server" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .to_lower_hybrid_opts::<Default>(AsciiFlatLine::LowLine),
    ///     "httpDevServer" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_hybrid_opts<O: Options>(&self, default_delim: AsciiFlatLine) -> String;

    /// Returns a string of the provided input converted to lower hybrid
    /// canonical form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_lower_hybrid_canonical_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`to_lower_hybrid_canonical_opts`]: Self::to_lower_hybrid_canonical_opts
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
    ///         .to_lower_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "lowerCamelCase"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "lowerHybridCase"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "lowerKebabCase"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "lowerSnakeCase"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "upperCamelCase"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "upperHybridCase"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "upperKebabCase"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "upperSnakeCase"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// This form *WILL* attempt to validate the first character to ensure it
    /// could be considered a valid identifier (see [`to_lower_hybrid`] for a
    /// version that would *NOT* validate this).
    ///
    /// [`to_lower_hybrid`]: Self::to_lower_hybrid
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .to_lower_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "_"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .to_lower_hybrid_canonical(AsciiFlatLine::LowLine),
    ///     "_2ExampleCamel"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_hybrid_canonical(&self, default_delim: AsciiFlatLine) -> String {
        self.to_lower_hybrid_canonical_opts::<Default>(default_delim)
    }

    /// Returns a string of the provided input converted to lower hybrid
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
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_hybrid_canonical_opts::<AllBoundaries>(AsciiFlatLine::LowLine),
    ///     "abc123HttpDevServer" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_hybrid_canonical_opts::<Default>(AsciiFlatLine::LowLine),
    ///     "abc_123HttpDevServer" // Digits aren't recognized as boundaries.
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
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .to_lower_hybrid_canonical_opts::<NoBoundaries>(AsciiFlatLine::LowLine),
    ///     "http_dev_server" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .to_lower_hybrid_canonical_opts::<Default>(AsciiFlatLine::LowLine),
    ///     "httpDevServer" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_hybrid_canonical_opts<O: Options>(&self, default_delim: AsciiFlatLine) -> String;

    /// Returns a string of the provided input converted to lower hybrid
    /// decorated form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_lower_hybrid_decorated_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`to_lower_hybrid_decorated_opts`]: Self::to_lower_hybrid_decorated_opts
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
    ///         .to_lower_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     "__lower__camelCase__"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     "_-lower-_hybridCase-_"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     "--lower--kebabCase--"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     "__lower__snakeCase__"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     "__upper__camelCase__"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     "-_upper_-hybridCase_-"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     "--upper--kebabCase--"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     "__upper__snakeCase__"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_hybrid_decorated(&self, default_delim: AsciiFlatLine) -> String {
        self.to_lower_hybrid_decorated_opts::<Default>(default_delim)
    }

    /// Returns a string of the provided input converted to lower hybrid
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
    /// Transforming to a more-bounded policy:
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_hybrid_decorated_opts::<AllBoundaries>(AsciiFlatLine::LowLine),
    ///     "__abc123HttpDevServer__" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_hybrid_decorated_opts::<Default>(AsciiFlatLine::LowLine),
    ///     "__abc_123HttpDevServer__" // Digits aren't recognized as boundaries.
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
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .to_lower_hybrid_decorated_opts::<NoBoundaries>(AsciiFlatLine::LowLine),
    ///     "__http_dev_server__" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .to_lower_hybrid_decorated_opts::<Default>(AsciiFlatLine::LowLine),
    ///     "__httpDevServer__" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_hybrid_decorated_opts<O: Options>(&self, default_delim: AsciiFlatLine) -> String;

    /// Returns a string of the provided input converted to lower hybrid
    /// delimited form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_lower_hybrid_delimited_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`to_lower_hybrid_delimited_opts`]: Self::to_lower_hybrid_delimited_opts
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
    ///         .to_lower_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     "__lower__camel_case__"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     "_-lower-_hybrid-case-_"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     "--lower--kebab-case--"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     "__lower__snake_case__"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     "__upper__camel_case__"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     "-_upper_-hybrid_case_-"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     "--upper--kebab-case--"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     "__upper__snake_case__"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_hybrid_delimited(&self, default_delim: AsciiFlatLine) -> String {
        self.to_lower_hybrid_delimited_opts::<Default>(default_delim)
    }

    /// Returns a string of the provided input converted to lower hybrid
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
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_hybrid_delimited_opts::<AllBoundaries>(AsciiFlatLine::LowLine),
    ///     "__abc_123_httpDevServer__" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to (in this case identical)...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_hybrid_delimited_opts::<Default>(AsciiFlatLine::LowLine),
    ///     "__abc_123_httpDevServer__" // Digits aren't recognized as boundaries.
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
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .to_lower_hybrid_delimited_opts::<NoBoundaries>(AsciiFlatLine::LowLine),
    ///     "__http_dev_server__" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .to_lower_hybrid_delimited_opts::<Default>(AsciiFlatLine::LowLine),
    ///     "__httpDevServer__" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_hybrid_delimited_opts<O: Options>(&self, default_delim: AsciiFlatLine) -> String;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> ToLowerHybrid for Ident<B, D, P> {
    #[inline]
    fn to_lower_hybrid_opts<O: Options>(&self, default_delim: AsciiFlatLine) -> String {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_lower_hybrid_opts::<O>(default_delim))
            .expect("failed to format an identifier into a string");
        string
    }
    #[inline]
    fn to_lower_hybrid_canonical_opts<O: Options>(&self, default_delim: AsciiFlatLine) -> String {
        let mut string = String::with_capacity(self.len());
        write!(
            string,
            "{}",
            self.as_lower_hybrid_canonical_opts::<O>(default_delim)
        )
        .expect("failed to format an identifier into a string");
        string
    }
    #[inline]
    fn to_lower_hybrid_decorated_opts<O: Options>(&self, default_delim: AsciiFlatLine) -> String {
        let mut string = String::with_capacity(self.len());
        write!(
            string,
            "{}",
            self.as_lower_hybrid_decorated_opts::<O>(default_delim)
        )
        .expect("failed to format an identifier into a string");
        string
    }
    #[inline]
    fn to_lower_hybrid_delimited_opts<O: Options>(&self, default_delim: AsciiFlatLine) -> String {
        let mut string = String::with_capacity(self.len());
        write!(
            string,
            "{}",
            self.as_lower_hybrid_delimited_opts::<O>(default_delim)
        )
        .expect("failed to format an identifier into a string");
        string
    }
}
