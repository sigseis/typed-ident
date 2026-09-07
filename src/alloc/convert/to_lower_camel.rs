// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Ident;
use crate::core::fmt::AsLowerCamel;
use crate::syntax::boundary::Boundary;
use crate::syntax::boundary::options::{Default, Options};
use crate::syntax::delimiter::Delimiter;
use crate::syntax::profile::Profile;
use core::fmt::Write;
use std_alloc::string::String;

// =============================================================================
// TRAITS
// =============================================================================

/// Converts an identifier into a preset lower-camel identifier string.
///
/// See the [`convert`] module for more details.
///
/// [`convert`]: crate::alloc::convert
pub trait ToLowerCamel {
    /// Returns a displayable type that converts the provided input to lower
    /// camel in canonical form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_lower_camel_canonical_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`to_lower_camel_canonical_opts`]: Self::to_lower_camel_canonical_opts
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_lower_camel_canonical(),
    ///     "lowerCamelCase"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_camel_canonical(),
    ///     "lowerHybridCase"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_camel_canonical(),
    ///     "lowerKebabCase"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_camel_canonical(),
    ///     "lowerSnakeCase"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_camel_canonical(),
    ///     "upperCamelCase"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_camel_canonical(),
    ///     "upperHybridCase"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_camel_canonical(),
    ///     "upperKebabCase"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_camel_canonical(),
    ///     "upperSnakeCase"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Notice that it will attempt to keep necessary prefix delimiters.
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .to_lower_camel_canonical(),
    ///     "_"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .to_lower_camel_canonical(),
    ///     "_2ExampleCamel"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_camel_canonical(&self) -> String {
        self.to_lower_camel_canonical_opts::<Default>()
    }

    /// Returns a displayable type that converts the provided input to lower
    /// camel in canonical form, over some provided boundary options.
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
    /// assert_eq!(
    ///     LowerCamelIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_camel_canonical_opts::<AllBoundaries>(),
    ///     "abc123HttpDevServer" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerCamelIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_camel_canonical_opts::<Default>(),
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
    /// assert_eq!(
    ///     LowerCamelIdent::new("__httpDEVServer__")?
    ///         .to_lower_camel_canonical_opts::<NoBoundaries>(),
    ///     "http_dev_server" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerCamelIdent::new("__httpDEVServer__")?
    ///         .to_lower_camel_canonical_opts::<Default>(),
    ///     "httpDevServer" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_camel_canonical_opts<O: Options>(&self) -> String;

    /// Returns a displayable type that converts the provided input to lower
    /// camel in decorated form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_lower_camel_decorated_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`to_lower_camel_decorated_opts`]: Self::to_lower_camel_decorated_opts
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_lower_camel_decorated(),
    ///     "__lower__camelCase__"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_camel_decorated(),
    ///     "__lower__hybridCase__"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_camel_decorated(),
    ///     "__lower__kebabCase__"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_camel_decorated(),
    ///     "__lower__snakeCase__"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_camel_decorated(),
    ///     "__upper__camelCase__"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_camel_decorated(),
    ///     "__upper__hybridCase__"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_camel_decorated(),
    ///     "__upper__kebabCase__"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_camel_decorated(),
    ///     "__upper__snakeCase__"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_camel_decorated(&self) -> String {
        self.to_lower_camel_decorated_opts::<Default>()
    }

    /// Returns a displayable type that converts the provided input to lower
    /// camel in decorated form, over some provided boundary options.
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
    /// assert_eq!(
    ///     LowerCamelIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_camel_decorated_opts::<AllBoundaries>(),
    ///     "__abc123HttpDevServer__" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerCamelIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_camel_decorated_opts::<Default>(),
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
    /// assert_eq!(
    ///     LowerCamelIdent::new("__httpDEVServer__")?
    ///         .to_lower_camel_decorated_opts::<NoBoundaries>(),
    ///     "__http_dev_server__" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerCamelIdent::new("__httpDEVServer__")?
    ///         .to_lower_camel_decorated_opts::<Default>(),
    ///     "__httpDevServer__" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_camel_decorated_opts<O: Options>(&self) -> String;

    /// Returns a displayable type that converts the provided input to lower
    /// camel in delimited form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_lower_camel_delimited_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`to_lower_camel_delimited_opts`]: Self::to_lower_camel_delimited_opts
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_lower_camel_delimited(),
    ///     "__lower__camel_case__"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_camel_delimited(),
    ///     "__lower__hybrid_case__"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_camel_delimited(),
    ///     "__lower__kebab_case__"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_camel_delimited(),
    ///     "__lower__snake_case__"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_camel_delimited(),
    ///     "__upper__camel_case__"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_camel_delimited(),
    ///     "__upper__hybrid_case__"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_camel_delimited(),
    ///     "__upper__kebab_case__"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_camel_delimited(),
    ///     "__upper__snake_case__"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_camel_delimited(&self) -> String {
        self.to_lower_camel_delimited_opts::<Default>()
    }

    /// Returns a displayable type that converts the provided input to lower
    /// camel in delimited form, over some provided boundary options.
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
    /// assert_eq!(
    ///     LowerCamelIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_camel_delimited_opts::<AllBoundaries>(),
    ///     "__abc_123_httpDevServer__" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to (in this case identical)...
    /// assert_eq!(
    ///     LowerCamelIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_camel_delimited_opts::<Default>(),
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
    /// assert_eq!(
    ///     LowerCamelIdent::new("__httpDEVServer__")?
    ///         .to_lower_camel_delimited_opts::<NoBoundaries>(),
    ///     "__http_dev_server__" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerCamelIdent::new("__httpDEVServer__")?
    ///         .to_lower_camel_delimited_opts::<Default>(),
    ///     "__httpDevServer__" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_camel_delimited_opts<O: Options>(&self) -> String;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> ToLowerCamel for Ident<B, D, P> {
    #[inline]
    fn to_lower_camel_canonical_opts<O: Options>(&self) -> String {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_lower_camel_canonical_opts::<O>())
            .expect("failed to format an identifier into a string");
        string
    }
    #[inline]
    fn to_lower_camel_decorated_opts<O: Options>(&self) -> String {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_lower_camel_decorated_opts::<O>())
            .expect("failed to format an identifier into a string");
        string
    }
    #[inline]
    fn to_lower_camel_delimited_opts<O: Options>(&self) -> String {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_lower_camel_delimited_opts::<O>())
            .expect("failed to format an identifier into a string");
        string
    }
}
