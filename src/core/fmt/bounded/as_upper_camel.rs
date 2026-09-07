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
    name=UpperCamelCanonical,
    over=Canonical,
    upper=true,
    docs=concat!(
        "A `Display` type for the [`AsUpperCamel`] trait in canonical format.",
        "\n\n",
        "This type is constructed by calling one of the [`as_upper_camel_canonical`] methods.",
        "\n\n",
        "[`as_upper_camel_canonical`]: AsUpperCamel::as_upper_camel_canonical"
    ),
}

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=UpperCamelDecorated,
    over=Decorated,
    upper=true,
    docs=concat!(
        "A `Display` type for the [`AsUpperCamel`] trait in decorated format.",
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
        "A `Display` type for the [`AsUpperCamel`] trait in delimited format.",
        "\n\n",
        "This type is constructed by calling one of the [`as_upper_camel_delimited`] methods.",
        "\n\n",
        "[`as_upper_camel_delimited`]: AsUpperCamel::as_upper_camel_delimited"
    ),
}

// =============================================================================
// TRAIT
// =============================================================================

/// Formats an identifier as a preset upper-camel identifier.
///
/// See the [`fmt`] module for more details.
///
/// [`fmt`]: crate::core::fmt
pub trait AsUpperCamel {
    /// Returns a displayable type that converts the provided input to upper
    /// camel in canonical form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`as_upper_camel_canonical_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`as_upper_camel_canonical_opts`]: Self::as_upper_camel_canonical_opts
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
    ///         .as_upper_camel_canonical()
    ///         .to_string(),
    ///     "LowerCamelCase"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .as_upper_camel_canonical()
    ///         .to_string(),
    ///     "LowerHybridCase"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .as_upper_camel_canonical()
    ///         .to_string(),
    ///     "LowerKebabCase"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .as_upper_camel_canonical()
    ///         .to_string(),
    ///     "LowerSnakeCase"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .as_upper_camel_canonical()
    ///         .to_string(),
    ///     "UpperCamelCase"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .as_upper_camel_canonical()
    ///         .to_string(),
    ///     "UpperHybridCase"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .as_upper_camel_canonical()
    ///         .to_string(),
    ///     "UpperKebabCase"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .as_upper_camel_canonical()
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
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .as_upper_camel_canonical()
    ///         .to_string(),
    ///     "_"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .as_upper_camel_canonical()
    ///         .to_string(),
    ///     "_2ExampleCamel"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_camel_canonical(&self) -> UpperCamelCanonical<'_> {
        self.as_upper_camel_canonical_opts::<Default>()
    }

    /// Returns a displayable type that converts the provided input to upper
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
    ///     UpperCamelIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .as_upper_camel_canonical_opts::<AllBoundaries>()
    ///         .to_string(),
    ///     "Abc123HttpDevServer" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .as_upper_camel_canonical_opts::<Default>()
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
    /// assert_eq!(
    ///     UpperCamelIdent::new("__HttpDEVServer__")?
    ///         .as_upper_camel_canonical_opts::<NoBoundaries>()
    ///         .to_string(),
    ///     "Http_Dev_Server" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperCamelIdent::new("__HttpDEVServer__")?
    ///         .as_upper_camel_canonical_opts::<Default>()
    ///         .to_string(),
    ///     "HttpDevServer" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_camel_canonical_opts<O: Options>(&self) -> UpperCamelCanonical<'_>;

    /// Returns a displayable type that converts the provided input to upper
    /// camel in decorated form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`as_upper_camel_decorated_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`as_upper_camel_decorated_opts`]: Self::as_upper_camel_decorated_opts
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
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
    #[inline]
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_camel_decorated(&self) -> UpperCamelDecorated<'_> {
        self.as_upper_camel_decorated_opts::<Default>()
    }

    /// Returns a displayable type that converts the provided input to upper
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
    ///     UpperCamelIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .as_upper_camel_decorated_opts::<AllBoundaries>()
    ///         .to_string(),
    ///     "__Abc123HttpDevServer__" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .as_upper_camel_decorated_opts::<Default>()
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
    /// assert_eq!(
    ///     UpperCamelIdent::new("__HttpDEVServer__")?
    ///         .as_upper_camel_decorated_opts::<NoBoundaries>()
    ///         .to_string(),
    ///     "__Http_Dev_Server__" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperCamelIdent::new("__HttpDEVServer__")?
    ///         .as_upper_camel_decorated_opts::<Default>()
    ///         .to_string(),
    ///     "__HttpDevServer__" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_camel_decorated_opts<O: Options>(&self) -> UpperCamelDecorated<'_>;

    /// Returns a displayable type that converts the provided input to upper
    /// camel in delimited form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`as_upper_camel_delimited_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`as_upper_camel_delimited_opts`]: Self::as_upper_camel_delimited_opts
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
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
    fn as_upper_camel_delimited(&self) -> UpperCamelDelimited<'_> {
        self.as_upper_camel_delimited_opts::<Default>()
    }

    /// Returns a displayable type that converts the provided input to upper
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
    ///     UpperCamelIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .as_upper_camel_delimited_opts::<AllBoundaries>()
    ///         .to_string(),
    ///     "__Abc_123_HttpDevServer__" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to (in this case identical)...
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .as_upper_camel_delimited_opts::<Default>()
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
    /// assert_eq!(
    ///     UpperCamelIdent::new("__HttpDEVServer__")?
    ///         .as_upper_camel_delimited_opts::<NoBoundaries>()
    ///         .to_string(),
    ///     "__Http_Dev_Server__" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperCamelIdent::new("__HttpDEVServer__")?
    ///         .as_upper_camel_delimited_opts::<Default>()
    ///         .to_string(),
    ///     "__HttpDevServer__" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_camel_delimited_opts<O: Options>(&self) -> UpperCamelDelimited<'_>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> AsUpperCamel for Ident<B, D, P> {
    #[inline]
    fn as_upper_camel_canonical_opts<O: Options>(&self) -> UpperCamelCanonical<'_> {
        UpperCamelCanonical(Canonical::new::<B, D, P::BaseProfile, Standard<O>>(
            self.as_str(),
            '_',
        ))
    }
    #[inline]
    fn as_upper_camel_decorated_opts<O: Options>(&self) -> UpperCamelDecorated<'_> {
        UpperCamelDecorated(Decorated::new::<B, D, P::BaseProfile, Standard<O>>(
            self.as_str(),
            '_',
            None,
        ))
    }
    #[inline]
    fn as_upper_camel_delimited_opts<O: Options>(&self) -> UpperCamelDelimited<'_> {
        UpperCamelDelimited(Delimited::new::<B, D, P::BaseProfile, Standard<O>>(
            self.as_str(),
            '_',
            None,
        ))
    }
}
