// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::Ident;
use crate::core::fmt::bounded::{Canonical, Decorated, Delimited};
use crate::syntax::boundary::Standard;
use crate::syntax::boundary::options::{Default, Options};
use crate::syntax::{Boundary, Delimiter, Profile};

// =============================================================================
// TYPEDEFS
// =============================================================================

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=LowerCamelCanonical,
    over=Canonical,
    upper=false,
    docs=concat!(
        "A `Display` type for the [`AsLowerCamel`] trait in canonical format.",
        "\n\n",
        "This type is constructed by calling one of the [`as_lower_camel_canonical`] methods.",
        "\n\n",
        "[`as_lower_camel_canonical`]: AsLowerCamel::as_lower_camel_canonical"
    ),
}

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=LowerCamelDecorated,
    over=Decorated,
    upper=false,
    docs=concat!(
        "A `Display` type for the [`AsLowerCamel`] trait in decorated format.",
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
        "A `Display` type for the [`AsLowerCamel`] trait in delimited format.",
        "\n\n",
        "This type is constructed by calling one of the [`as_lower_camel_delimited`] methods.",
        "\n\n",
        "[`as_lower_camel_delimited`]: AsLowerCamel::as_lower_camel_delimited"
    ),
}

// =============================================================================
// TRAIT
// =============================================================================

/// Formats an identifier as a preset lower-camel identifier.
///
/// See the [`fmt`] module for more details.
///
/// [`fmt`]: crate::core::fmt
pub trait AsLowerCamel {
    /// Returns a displayable type that converts the provided input to lower
    /// camel in canonical form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`as_lower_camel_canonical_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`as_lower_camel_canonical_opts`]: Self::as_lower_camel_canonical_opts
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .as_lower_camel_canonical()
    ///         .to_string(),
    ///     "lowerCamelCase"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_lower_camel_canonical()
    ///         .to_string(),
    ///     "lowerHybridCase"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_lower_camel_canonical()
    ///         .to_string(),
    ///     "lowerKebabCase"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_lower_camel_canonical()
    ///         .to_string(),
    ///     "lowerSnakeCase"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_lower_camel_canonical()
    ///         .to_string(),
    ///     "upperCamelCase"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_lower_camel_canonical()
    ///         .to_string(),
    ///     "upperHybridCase"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_lower_camel_canonical()
    ///         .to_string(),
    ///     "upperKebabCase"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_lower_camel_canonical()
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
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .as_lower_camel_canonical()
    ///         .to_string(),
    ///     "_"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .as_lower_camel_canonical()
    ///         .to_string(),
    ///     "_2ExampleCamel"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_camel_canonical(&self) -> LowerCamelCanonical<'_> {
        self.as_lower_camel_canonical_opts::<Default>()
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
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__abc_123_httpDEVServer__")?
    ///         .as_lower_camel_canonical_opts::<AllBoundaries>()
    ///         .to_string(),
    ///     "abc123HttpDevServer" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerCamelIdent::new("__abc_123_httpDEVServer__")?
    ///         .as_lower_camel_canonical_opts::<Default>()
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
    /// assert_eq!(
    ///     LowerCamelIdent::new("__httpDEVServer__")?
    ///         .as_lower_camel_canonical_opts::<NoBoundaries>()
    ///         .to_string(),
    ///     "http_dev_server" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerCamelIdent::new("__httpDEVServer__")?
    ///         .as_lower_camel_canonical_opts::<Default>()
    ///         .to_string(),
    ///     "httpDevServer" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_camel_canonical_opts<O: Options>(&self) -> LowerCamelCanonical<'_>;

    /// Returns a displayable type that converts the provided input to lower
    /// camel in decorated form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`as_lower_camel_decorated_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`as_lower_camel_decorated_opts`]: Self::as_lower_camel_decorated_opts
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
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
    #[inline]
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_camel_decorated(&self) -> LowerCamelDecorated<'_> {
        self.as_lower_camel_decorated_opts::<Default>()
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
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__abc_123_httpDEVServer__")?
    ///         .as_lower_camel_decorated_opts::<AllBoundaries>()
    ///         .to_string(),
    ///     "__abc123HttpDevServer__" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerCamelIdent::new("__abc_123_httpDEVServer__")?
    ///         .as_lower_camel_decorated_opts::<Default>()
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
    /// assert_eq!(
    ///     LowerCamelIdent::new("__httpDEVServer__")?
    ///         .as_lower_camel_decorated_opts::<NoBoundaries>()
    ///         .to_string(),
    ///     "__http_dev_server__" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerCamelIdent::new("__httpDEVServer__")?
    ///         .as_lower_camel_decorated_opts::<Default>()
    ///         .to_string(),
    ///     "__httpDevServer__" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_camel_decorated_opts<O: Options>(&self) -> LowerCamelDecorated<'_>;

    /// Returns a displayable type that converts the provided input to lower
    /// camel in delimited form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`as_lower_camel_delimited_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`as_lower_camel_delimited_opts`]: Self::as_lower_camel_delimited_opts
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
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
    fn as_lower_camel_delimited(&self) -> LowerCamelDelimited<'_> {
        self.as_lower_camel_delimited_opts::<Default>()
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
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__abc_123_httpDEVServer__")?
    ///         .as_lower_camel_delimited_opts::<AllBoundaries>()
    ///         .to_string(),
    ///     "__abc_123_httpDevServer__" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to (in this case identical)...
    /// assert_eq!(
    ///     LowerCamelIdent::new("__abc_123_httpDEVServer__")?
    ///         .as_lower_camel_delimited_opts::<Default>()
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
    /// assert_eq!(
    ///     LowerCamelIdent::new("__httpDEVServer__")?
    ///         .as_lower_camel_delimited_opts::<NoBoundaries>()
    ///         .to_string(),
    ///     "__http_dev_server__" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerCamelIdent::new("__httpDEVServer__")?
    ///         .as_lower_camel_delimited_opts::<Default>()
    ///         .to_string(),
    ///     "__httpDevServer__" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_lower_camel_delimited_opts<O: Options>(&self) -> LowerCamelDelimited<'_>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> AsLowerCamel for Ident<B, D, P> {
    #[inline]
    fn as_lower_camel_canonical_opts<O: Options>(&self) -> LowerCamelCanonical<'_> {
        LowerCamelCanonical(Canonical::new::<B, D, P::BaseProfile, Standard<O>>(
            self.as_str(),
            '_',
        ))
    }
    #[inline]
    fn as_lower_camel_decorated_opts<O: Options>(&self) -> LowerCamelDecorated<'_> {
        LowerCamelDecorated(Decorated::new::<B, D, P::BaseProfile, Standard<O>>(
            self.as_str(),
            '_',
            None,
        ))
    }
    #[inline]
    fn as_lower_camel_delimited_opts<O: Options>(&self) -> LowerCamelDelimited<'_> {
        LowerCamelDelimited(Delimited::new::<B, D, P::BaseProfile, Standard<O>>(
            self.as_str(),
            '_',
            None,
        ))
    }
}
