// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::fmt::AsLowerCamel;
use crate::intermediate::FromIntermediate;
use crate::syntax::Profile;
use crate::syntax::boundary::options::{Default, Options};
use crate::{Error, Identifier};
use core::fmt::Write;
use std_alloc::string::String;

// =============================================================================
// TYPE ALIASES
// =============================================================================

// -----------------------------------------------------------------------------
type LowerCamelIdent<P> = crate::presets::generic::LowerCamelIdent<<P as Profile>::CharProfile>;

// =============================================================================
// TRAITS
// =============================================================================

/// Provides methods for converting an identifier to lower-camel format, using
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
/// [`convert`]: crate::convert
pub trait ToLowerCamel: Identifier {
    /// Returns a string of the provided input converted to lower camel
    /// canonical form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_lower_camel_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`to_lower_camel_opts`]: Self::to_lower_camel_opts
    /// [`fmt`]: crate::fmt
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_lower_camel(),
    ///     Ok(LowerCamelIdent::new_boxed("lowerCamelCase")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_camel(),
    ///     Ok(LowerCamelIdent::new_boxed("lowerHybridCase")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_camel(),
    ///     Ok(LowerCamelIdent::new_boxed("lowerKebabCase")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_camel(),
    ///     Ok(LowerCamelIdent::new_boxed("lowerSnakeCase")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_camel(),
    ///     Ok(LowerCamelIdent::new_boxed("upperCamelCase")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_camel(),
    ///     Ok(LowerCamelIdent::new_boxed("upperHybridCase")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_camel(),
    ///     Ok(LowerCamelIdent::new_boxed("upperKebabCase")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_camel(),
    ///     Ok(LowerCamelIdent::new_boxed("upperSnakeCase")?),
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
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .to_lower_camel(),
    ///     Ok(LowerCamelIdent::new_boxed("_")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .to_lower_camel(),
    ///     Ok(LowerCamelIdent::new_boxed("_2ExampleCamel")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_camel<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerCamelIdent<Self::Profile>>,
    {
        self.to_lower_camel_opts::<T, Default>()
    }

    /// Returns a string of the provided input converted to lower camel
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
    /// assert_eq!(
    ///     LowerCamelIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_camel_opts::<_, AllBoundaries>(),
    ///     Ok(LowerCamelIdent::new_boxed("abc123HttpDevServer")?), // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerCamelIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_camel_opts::<_, Default>(),
    ///     Ok(LowerCamelIdent::new_boxed("abc_123HttpDevServer")?), // Digits aren't recognized as boundaries.
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
    /// assert_eq!(
    ///     LowerCamelIdent::new("__httpDEVServer__")?
    ///         .to_lower_camel_opts::<_, NoBoundaries>(),
    ///     Ok(LowerCamelIdent::new_boxed("http_dev_server")?), // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerCamelIdent::new("__httpDEVServer__")?
    ///         .to_lower_camel_opts::<_, Default>(),
    ///     Ok(LowerCamelIdent::new_boxed("httpDevServer")?), // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_camel_opts<T, O: Options>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerCamelIdent<Self::Profile>>;

    /// Returns a string of the provided input converted to lower camel
    /// decorated form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_lower_camel_decorated_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`to_lower_camel_decorated_opts`]: Self::to_lower_camel_decorated_opts
    /// [`fmt`]: crate::fmt
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_lower_camel_decorated(),
    ///     Ok(LowerCamelIdent::new_boxed("__lower__camelCase__")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_camel_decorated(),
    ///     Ok(LowerCamelIdent::new_boxed("__lower__hybridCase__")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_camel_decorated(),
    ///     Ok(LowerCamelIdent::new_boxed("__lower__kebabCase__")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_camel_decorated(),
    ///     Ok(LowerCamelIdent::new_boxed("__lower__snakeCase__")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_camel_decorated(),
    ///     Ok(LowerCamelIdent::new_boxed("__upper__camelCase__")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_camel_decorated(),
    ///     Ok(LowerCamelIdent::new_boxed("__upper__hybridCase__")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_camel_decorated(),
    ///     Ok(LowerCamelIdent::new_boxed("__upper__kebabCase__")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_camel_decorated(),
    ///     Ok(LowerCamelIdent::new_boxed("__upper__snakeCase__")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_camel_decorated<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerCamelIdent<Self::Profile>>,
    {
        self.to_lower_camel_decorated_opts::<T, Default>()
    }

    /// Returns a string of the provided input converted to lower camel
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
    /// # Examples
    ///
    /// Transforming to a more-bounded policy:
    ///
    /// ```
    /// # use typed_ident::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// # use typed_ident::syntax::boundary::options::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_camel_decorated_opts::<_, AllBoundaries>(),
    ///     Ok(LowerCamelIdent::new_boxed("__abc123HttpDevServer__")?), // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerCamelIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_camel_decorated_opts::<_, Default>(),
    ///     Ok(LowerCamelIdent::new_boxed("__abc_123HttpDevServer__")?), // Digits aren't recognized as boundaries.
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
    /// assert_eq!(
    ///     LowerCamelIdent::new("__httpDEVServer__")?
    ///         .to_lower_camel_decorated_opts::<_, NoBoundaries>(),
    ///     Ok(LowerCamelIdent::new_boxed("__http_dev_server__")?), // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerCamelIdent::new("__httpDEVServer__")?
    ///         .to_lower_camel_decorated_opts::<_, Default>(),
    ///     Ok(LowerCamelIdent::new_boxed("__httpDevServer__")?), // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_camel_decorated_opts<T, O: Options>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerCamelIdent<Self::Profile>>;

    /// Returns a string of the provided input converted to lower camel
    /// delimited form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_lower_camel_delimited_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`to_lower_camel_delimited_opts`]: Self::to_lower_camel_delimited_opts
    /// [`fmt`]: crate::fmt
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_lower_camel_delimited(),
    ///     Ok(LowerCamelIdent::new_boxed("__lower__camel_case__")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_camel_delimited(),
    ///     Ok(LowerCamelIdent::new_boxed("__lower__hybrid_case__")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_camel_delimited(),
    ///     Ok(LowerCamelIdent::new_boxed("__lower__kebab_case__")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_camel_delimited(),
    ///     Ok(LowerCamelIdent::new_boxed("__lower__snake_case__")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_camel_delimited(),
    ///     Ok(LowerCamelIdent::new_boxed("__upper__camel_case__")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_camel_delimited(),
    ///     Ok(LowerCamelIdent::new_boxed("__upper__hybrid_case__")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_camel_delimited(),
    ///     Ok(LowerCamelIdent::new_boxed("__upper__kebab_case__")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_camel_delimited(),
    ///     Ok(LowerCamelIdent::new_boxed("__upper__snake_case__")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_camel_delimited<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerCamelIdent<Self::Profile>>,
    {
        self.to_lower_camel_delimited_opts::<T, Default>()
    }

    /// Returns a string of the provided input converted to lower camel
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
    /// assert_eq!(
    ///     LowerCamelIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_camel_delimited_opts::<_, AllBoundaries>(),
    ///     Ok(LowerCamelIdent::new_boxed("__abc_123_httpDevServer__")?), // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to (in this case identical)...
    /// assert_eq!(
    ///     LowerCamelIdent::new("__abc_123_httpDEVServer__")?
    ///         .to_lower_camel_delimited_opts::<_, Default>(),
    ///     Ok(LowerCamelIdent::new_boxed("__abc_123_httpDevServer__")?), // Digits aren't recognized as boundaries.
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
    /// assert_eq!(
    ///     LowerCamelIdent::new("__httpDEVServer__")?
    ///         .to_lower_camel_delimited_opts::<_, NoBoundaries>(),
    ///     Ok(LowerCamelIdent::new_boxed("__http_dev_server__")?), // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     LowerCamelIdent::new("__httpDEVServer__")?
    ///         .to_lower_camel_delimited_opts::<_, Default>(),
    ///     Ok(LowerCamelIdent::new_boxed("__httpDevServer__")?), // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_camel_delimited_opts<T, O: Options>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerCamelIdent<Self::Profile>>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<I: Identifier + ?Sized> ToLowerCamel for I {
    #[inline]
    fn to_lower_camel_opts<T, O: Options>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerCamelIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_lower_camel_opts::<O>())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
    #[inline]
    fn to_lower_camel_decorated_opts<T, O: Options>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerCamelIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_lower_camel_decorated_opts::<O>())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
    #[inline]
    fn to_lower_camel_delimited_opts<T, O: Options>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerCamelIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_lower_camel_delimited_opts::<O>())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
}
