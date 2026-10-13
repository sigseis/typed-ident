// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::fmt::AsLowerHybrid;
use crate::intermediate::FromIntermediate;
use crate::syntax::Profile;
use crate::syntax::boundary::options::{Default, Options};
use crate::syntax::delimiter::AsciiFlatLine;
use crate::{Error, Identifier};
use core::fmt::Write;
use std_alloc::string::String;

// =============================================================================
// TYPE ALIASES
// =============================================================================

// -----------------------------------------------------------------------------
type LowerHybridIdent<P> = crate::presets::generic::LowerHybridIdent<<P as Profile>::CharProfile>;

// =============================================================================
// TRAITS
// =============================================================================

/// Provides methods for converting an identifier to lower-hybrid format, using
/// canonical, decorated, or delimited forms.
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
/// [`convert`]: crate::convert
pub trait ToLowerHybrid: Identifier {
    /// Returns a string of the provided input converted to lower hybrid
    /// canonical form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_lower_hybrid_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`to_lower_hybrid_opts`]: Self::to_lower_hybrid_opts
    /// [`fmt`]: crate::fmt
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("lowerCamelCase")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("lowerHybridCase")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("lowerKebabCase")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("lowerSnakeCase")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("upperCamelCase")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("upperHybridCase")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("upperKebabCase")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("upperSnakeCase")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// This form *WILL* attempt to validate the first character to ensure it
    /// could be considered a valid identifier.
    ///
    /// ```
    /// # use typed_ident::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("_")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .to_lower_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("_2ExampleCamel")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_hybrid<T>(&self, default_delim: AsciiFlatLine) -> Result<T, Error>
    where
        T: FromIntermediate<LowerHybridIdent<Self::Profile>>,
    {
        self.to_lower_hybrid_opts::<T, Default>(default_delim)
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
    /// [`fmt`]: crate::fmt
    ///
    /// # Examples
    ///
    /// Transforming to a more-bounded policy:
    ///
    /// ```
    /// # use typed_ident::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_hybrid_opts::<_, AllBoundaries>(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("abc123HttpDevServer")?), // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_hybrid_opts::<_, Default>(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("abc_123HttpDevServer")?), // Digits aren't recognized as boundaries.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Transforming to a less-bounded policy:
    ///
    /// ```
    /// # use typed_ident::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .to_lower_hybrid_opts::<_, NoBoundaries>(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("http_dev_server")?), // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .to_lower_hybrid_opts::<_, Default>(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("httpDevServer")?), // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_hybrid_opts<T, O: Options>(&self, default_delim: AsciiFlatLine) -> Result<T, Error>
    where
        T: FromIntermediate<LowerHybridIdent<Self::Profile>>;

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
    /// [`fmt`]: crate::fmt
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_lower_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("__lower__camelCase__")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("_-lower-_hybridCase-_")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("--lower--kebabCase--")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("__lower__snakeCase__")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("__upper__camelCase__")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("-_upper_-hybridCase_-")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("--upper--kebabCase--")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("__upper__snakeCase__")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_hybrid_decorated<T>(&self, default_delim: AsciiFlatLine) -> Result<T, Error>
    where
        T: FromIntermediate<LowerHybridIdent<Self::Profile>>,
    {
        self.to_lower_hybrid_decorated_opts::<T, Default>(default_delim)
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
    /// [`fmt`]: crate::fmt
    ///
    /// Transforming to a more-bounded policy:
    ///
    /// ```
    /// # use typed_ident::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_hybrid_decorated_opts::<_, AllBoundaries>(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("__abc123HttpDevServer__")?), // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_hybrid_decorated_opts::<_, Default>(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("__abc_123HttpDevServer__")?), // Digits aren't recognized as boundaries.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Transforming to a less-bounded policy:
    ///
    /// ```
    /// # use typed_ident::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .to_lower_hybrid_decorated_opts::<_, NoBoundaries>(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("__http_dev_server__")?), // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .to_lower_hybrid_decorated_opts::<_, Default>(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("__httpDevServer__")?), // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_hybrid_decorated_opts<T, O: Options>(
        &self,
        default_delim: AsciiFlatLine,
    ) -> Result<T, Error>
    where
        T: FromIntermediate<LowerHybridIdent<Self::Profile>>;

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
    /// [`fmt`]: crate::fmt
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_lower_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("__lower__camel_case__")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("_-lower-_hybrid-case-_")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("--lower--kebab-case--")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("__lower__snake_case__")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("__upper__camel_case__")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("-_upper_-hybrid_case_-")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("--upper--kebab-case--")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("__upper__snake_case__")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_hybrid_delimited<T>(&self, default_delim: AsciiFlatLine) -> Result<T, Error>
    where
        T: FromIntermediate<LowerHybridIdent<Self::Profile>>,
    {
        self.to_lower_hybrid_delimited_opts::<T, Default>(default_delim)
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
    /// [`fmt`]: crate::fmt
    ///
    /// # Examples
    ///
    /// Transforming to a more-bounded policy:
    ///
    /// ```
    /// # use typed_ident::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_hybrid_delimited_opts::<_, AllBoundaries>(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("__abc_123_httpDevServer__")?), // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to (in this case identical)...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_hybrid_delimited_opts::<_, Default>(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("__abc_123_httpDevServer__")?), // Digits aren't recognized as boundaries.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Transforming to a less-bounded policy:
    ///
    /// ```
    /// # use typed_ident::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .to_lower_hybrid_delimited_opts::<_, NoBoundaries>(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("__http_dev_server__")?), // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerHybridIdent::new("__httpDEVServer__")?
    ///         .to_lower_hybrid_delimited_opts::<_, Default>(AsciiFlatLine::LowLine),
    ///     Ok(LowerHybridIdent::new_boxed("__httpDevServer__")?), // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_hybrid_delimited_opts<T, O: Options>(
        &self,
        default_delim: AsciiFlatLine,
    ) -> Result<T, Error>
    where
        T: FromIntermediate<LowerHybridIdent<Self::Profile>>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<I: Identifier + ?Sized> ToLowerHybrid for I {
    #[inline]
    fn to_lower_hybrid_opts<T, O: Options>(&self, default_delim: AsciiFlatLine) -> Result<T, Error>
    where
        T: FromIntermediate<LowerHybridIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_lower_hybrid_opts::<O>(default_delim))
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
    #[inline]
    fn to_lower_hybrid_decorated_opts<T, O: Options>(
        &self,
        default_delim: AsciiFlatLine,
    ) -> Result<T, Error>
    where
        T: FromIntermediate<LowerHybridIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(
            string,
            "{}",
            self.as_lower_hybrid_decorated_opts::<O>(default_delim)
        )
        .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
    #[inline]
    fn to_lower_hybrid_delimited_opts<T, O: Options>(
        &self,
        default_delim: AsciiFlatLine,
    ) -> Result<T, Error>
    where
        T: FromIntermediate<LowerHybridIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(
            string,
            "{}",
            self.as_lower_hybrid_delimited_opts::<O>(default_delim)
        )
        .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
}
