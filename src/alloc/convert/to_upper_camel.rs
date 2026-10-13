// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::fmt::AsUpperCamel;
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
type UpperCamelIdent<P> = crate::presets::generic::UpperCamelIdent<<P as Profile>::CharProfile>;

// =============================================================================
// TRAITS
// =============================================================================

/// Provides methods for converting an identifier to upper-camel format, using
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
pub trait ToUpperCamel: Identifier {
    /// Returns a string of the provided input converted to upper camel
    /// canonical form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_upper_camel_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`to_upper_camel_opts`]: Self::to_upper_camel_opts
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
    ///         .to_upper_camel(),
    ///     Ok(UpperCamelIdent::new_boxed("LowerCamelCase")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_upper_camel(),
    ///     Ok(UpperCamelIdent::new_boxed("LowerHybridCase")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_upper_camel(),
    ///     Ok(UpperCamelIdent::new_boxed("LowerKebabCase")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_upper_camel(),
    ///     Ok(UpperCamelIdent::new_boxed("LowerSnakeCase")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_upper_camel(),
    ///     Ok(UpperCamelIdent::new_boxed("UpperCamelCase")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_upper_camel(),
    ///     Ok(UpperCamelIdent::new_boxed("UpperHybridCase")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_upper_camel(),
    ///     Ok(UpperCamelIdent::new_boxed("UpperKebabCase")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_upper_camel(),
    ///     Ok(UpperCamelIdent::new_boxed("UpperSnakeCase")?),
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
    ///         .to_upper_camel(),
    ///     Ok(UpperCamelIdent::new_boxed("_")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .to_upper_camel(),
    ///     Ok(UpperCamelIdent::new_boxed("_2ExampleCamel")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_camel<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperCamelIdent<Self::Profile>>,
    {
        self.to_upper_camel_opts::<T, Default>()
    }

    /// Returns a string of the provided input converted to upper camel
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
    ///     UpperCamelIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .to_upper_camel_opts::<_, AllBoundaries>(),
    ///     Ok(UpperCamelIdent::new_boxed("Abc123HttpDevServer")?), // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .to_upper_camel_opts::<_, Default>(),
    ///     Ok(UpperCamelIdent::new_boxed("Abc_123HttpDevServer")?), // Digits aren't recognized as boundaries.
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
    ///     UpperCamelIdent::new("__HttpDEVServer__")?
    ///         .to_upper_camel_opts::<_, NoBoundaries>(),
    ///     Ok(UpperCamelIdent::new_boxed("Http_Dev_Server")?), // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperCamelIdent::new("__HttpDEVServer__")?
    ///         .to_upper_camel_opts::<_, Default>(),
    ///     Ok(UpperCamelIdent::new_boxed("HttpDevServer")?), // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_camel_opts<T, O: Options>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperCamelIdent<Self::Profile>>;

    /// Returns a string of the provided input converted to upper camel
    /// decorated form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_upper_camel_decorated_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`to_upper_camel_decorated_opts`]: Self::to_upper_camel_decorated_opts
    /// [`fmt`]: crate::fmt
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_upper_camel_decorated(),
    ///     Ok(UpperCamelIdent::new_boxed("__Lower__CamelCase__")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_upper_camel_decorated(),
    ///     Ok(UpperCamelIdent::new_boxed("__Lower__HybridCase__")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_upper_camel_decorated(),
    ///     Ok(UpperCamelIdent::new_boxed("__Lower__KebabCase__")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_upper_camel_decorated(),
    ///     Ok(UpperCamelIdent::new_boxed("__Lower__SnakeCase__")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_upper_camel_decorated(),
    ///     Ok(UpperCamelIdent::new_boxed("__Upper__CamelCase__")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_upper_camel_decorated(),
    ///     Ok(UpperCamelIdent::new_boxed("__Upper__HybridCase__")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_upper_camel_decorated(),
    ///     Ok(UpperCamelIdent::new_boxed("__Upper__KebabCase__")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_upper_camel_decorated(),
    ///     Ok(UpperCamelIdent::new_boxed("__Upper__SnakeCase__")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_camel_decorated<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperCamelIdent<Self::Profile>>,
    {
        self.to_upper_camel_decorated_opts::<T, Default>()
    }

    /// Returns a string of the provided input converted to upper camel
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
    ///     UpperCamelIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .to_upper_camel_decorated_opts::<_, AllBoundaries>(),
    ///     Ok(UpperCamelIdent::new_boxed("__Abc123HttpDevServer__")?), // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .to_upper_camel_decorated_opts::<_, Default>(),
    ///     Ok(UpperCamelIdent::new_boxed("__Abc_123HttpDevServer__")?), // Digits aren't recognized as boundaries.
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
    ///     UpperCamelIdent::new("__HttpDEVServer__")?
    ///         .to_upper_camel_decorated_opts::<_, NoBoundaries>(),
    ///     Ok(UpperCamelIdent::new_boxed("__Http_Dev_Server__")?), // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperCamelIdent::new("__HttpDEVServer__")?
    ///         .to_upper_camel_decorated_opts::<_, Default>(),
    ///     Ok(UpperCamelIdent::new_boxed("__HttpDevServer__")?), // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_camel_decorated_opts<T, O: Options>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperCamelIdent<Self::Profile>>;

    /// Returns a string of the provided input converted to upper camel
    /// delimited form.
    ///
    /// This method uses the default boundary options. If you have customized
    /// your boundary definitions, you almost certainly want to use the method
    /// [`to_upper_camel_delimited_opts`].
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`to_upper_camel_delimited_opts`]: Self::to_upper_camel_delimited_opts
    /// [`fmt`]: crate::fmt
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_upper_camel_delimited(),
    ///     Ok(UpperCamelIdent::new_boxed("__Lower__Camel_Case__")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_upper_camel_delimited(),
    ///     Ok(UpperCamelIdent::new_boxed("__Lower__Hybrid_Case__")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_upper_camel_delimited(),
    ///     Ok(UpperCamelIdent::new_boxed("__Lower__Kebab_Case__")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_upper_camel_delimited(),
    ///     Ok(UpperCamelIdent::new_boxed("__Lower__Snake_Case__")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_upper_camel_delimited(),
    ///     Ok(UpperCamelIdent::new_boxed("__Upper__Camel_Case__")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_upper_camel_delimited(),
    ///     Ok(UpperCamelIdent::new_boxed("__Upper__Hybrid_Case__")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_upper_camel_delimited(),
    ///     Ok(UpperCamelIdent::new_boxed("__Upper__Kebab_Case__")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_upper_camel_delimited(),
    ///     Ok(UpperCamelIdent::new_boxed("__Upper__Snake_Case__")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_camel_delimited<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperCamelIdent<Self::Profile>>,
    {
        self.to_upper_camel_delimited_opts::<T, Default>()
    }

    /// Returns a string of the provided input converted to upper camel
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
    ///     UpperCamelIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .to_upper_camel_delimited_opts::<_, AllBoundaries>(),
    ///     Ok(UpperCamelIdent::new_boxed("__Abc_123_HttpDevServer__")?), // Digits recognized as proper boundaries.
    /// );
    /// // Compare this to (in this case identical)...
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Abc_123_HttpDEVServer__")?
    ///         .to_upper_camel_delimited_opts::<_, Default>(),
    ///     Ok(UpperCamelIdent::new_boxed("__Abc_123_HttpDevServer__")?), // Digits aren't recognized as boundaries.
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
    ///     UpperCamelIdent::new("__HttpDEVServer__")?
    ///         .to_upper_camel_delimited_opts::<_, NoBoundaries>(),
    ///     Ok(UpperCamelIdent::new_boxed("__Http_Dev_Server__")?), // Nothing recognized as a boundary.
    /// );
    /// // Compare this to...
    /// assert_eq!(
    ///     UpperCamelIdent::new("__HttpDEVServer__")?
    ///         .to_upper_camel_delimited_opts::<_, Default>(),
    ///     Ok(UpperCamelIdent::new_boxed("__HttpDevServer__")?), // Normal boundaries are recognized.
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_camel_delimited_opts<T, O: Options>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperCamelIdent<Self::Profile>>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<I: Identifier + ?Sized> ToUpperCamel for I {
    #[inline]
    fn to_upper_camel_opts<T, O: Options>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperCamelIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_upper_camel_opts::<O>())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
    #[inline]
    fn to_upper_camel_decorated_opts<T, O: Options>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperCamelIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_upper_camel_decorated_opts::<O>())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
    #[inline]
    fn to_upper_camel_delimited_opts<T, O: Options>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperCamelIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_upper_camel_delimited_opts::<O>())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
}
