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
    name=UpperHybrid,
    over=Canonical,
    upper=true,
    docs=concat!(
        "A `Display` type for the [`AsUpperHybrid`] trait in plain form.",
        "\n\n",
        "This type is constructed by calling one of the [`as_upper_hybrid`] methods.",
        "\n\n",
        "[`as_upper_hybrid`]: AsUpperHybrid::as_upper_hybrid"
    ),
}

// -----------------------------------------------------------------------------
impl_displayable_type! {
    name=UpperHybridCanonical,
    over=Canonical,
    upper=true,
    docs=concat!(
        "A `Display` type for the [`AsUpperHybrid`] trait in canonical form.",
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
/// plain, canonical, decorated, or delimited forms.
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
/// [`fmt`]: crate::core::fmt
pub trait AsUpperHybrid {
    /// Returns a displayable type that converts the provided input to upper
    /// hybrid in plain form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`as_upper_hybrid_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`as_upper_hybrid_opts`]: Self::as_upper_hybrid_opts
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
    /// This form will *NOT* attempt to validate the first character to ensure
    /// it could be considered a valid identifier (see
    /// [`as_upper_hybrid_canonical`] for a version that would validate this).
    ///
    /// [`as_upper_hybrid_canonical`]: Self::as_upper_hybrid_canonical
    ///
    /// ```
    /// # use typed_ident::core::fmt::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .as_upper_hybrid(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     ""
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .as_upper_hybrid(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "2ExampleCamel"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_hybrid(&self, default_delim: AsciiFlatLine) -> UpperHybrid<'_> {
        self.as_upper_hybrid_opts::<Default>(default_delim)
    }

    /// Returns a displayable type that converts the provided input to upper
    /// hybrid in plain form, over some provided boundary options.
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
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     UpperHybridIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .as_upper_hybrid_opts::<AllBoundaries>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "Abc123HttpDevServer" // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperHybridIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .as_upper_hybrid_opts::<Default>(AsciiFlatLine::LowLine)
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
    ///         .as_upper_hybrid_opts::<NoBoundaries>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "Http_Dev_Server" // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperHybridIdent::new("__HttpDEVServer__")?
    ///         .as_upper_hybrid_opts::<Default>(AsciiFlatLine::LowLine)
    ///         .to_string(),
    ///     "HttpDevServer" // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "formatting operations return displayable types, the original identifier is unmodified"]
    fn as_upper_hybrid_opts<O: Options>(&self, default_delim: AsciiFlatLine) -> UpperHybrid<'_>;

    /// Returns a displayable type that converts the provided input to upper
    /// hybrid in canonical form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`as_upper_hybrid_canonical_opts`].
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
    /// This form *WILL* attempt to validate the first character to ensure it
    /// could be considered a valid identifier (see [`as_upper_hybrid`] for a
    /// version that would *NOT* validate this).
    ///
    /// [`as_upper_hybrid`]: Self::as_upper_hybrid
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
    fn as_upper_hybrid_opts<O: Options>(&self, default_delim: AsciiFlatLine) -> UpperHybrid<'_> {
        UpperHybrid(Canonical::new::<B, D, P::CharProfile, Standard<O>>(
            self.as_str(),
            default_delim.as_char(),
            false,
        ))
    }
    #[inline]
    fn as_upper_hybrid_canonical_opts<O: Options>(
        &self,
        default_delim: AsciiFlatLine,
    ) -> UpperHybridCanonical<'_> {
        UpperHybridCanonical(Canonical::new::<B, D, P::CharProfile, Standard<O>>(
            self.as_str(),
            default_delim.as_char(),
            true,
        ))
    }
    #[inline]
    fn as_upper_hybrid_decorated_opts<O: Options>(
        &self,
        default_delim: AsciiFlatLine,
    ) -> UpperHybridDecorated<'_> {
        UpperHybridDecorated(Decorated::new::<B, D, P::CharProfile, Standard<O>>(
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
        UpperHybridDelimited(Delimited::new::<B, D, P::CharProfile, Standard<O>>(
            self.as_str(),
            default_delim.as_char(),
            Some(match default_delim {
                AsciiFlatLine::LowLine => '-',
                AsciiFlatLine::HyphenMinus => '_',
            }),
        ))
    }
}
