// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::fmt::AsLowerCamel;
use crate::intermediate::FromIntermediate;
use crate::syntax::Profile;
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
/// [`convert`]: crate::convert
pub trait ToLowerCamel: Identifier {
    /// Returns a string of the provided input converted to lower camel
    /// canonical form.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
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
    #[must_use = "format conversion returns a newly-allocated result, the original identifier is unmodified"]
    fn to_lower_camel<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerCamelIdent<Self::Profile>>;

    /// Returns a string of the provided input converted to lower camel
    /// decorated form.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
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
    #[must_use = "format conversion returns a newly-allocated result, the original identifier is unmodified"]
    fn to_lower_camel_decorated<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerCamelIdent<Self::Profile>>;

    /// Returns a string of the provided input converted to lower camel
    /// delimited form.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
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
    #[must_use = "format conversion returns a newly-allocated result, the original identifier is unmodified"]
    fn to_lower_camel_delimited<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerCamelIdent<Self::Profile>>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<I: Identifier + ?Sized> ToLowerCamel for I {
    #[inline]
    fn to_lower_camel<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerCamelIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_lower_camel())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
    #[inline]
    fn to_lower_camel_decorated<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerCamelIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_lower_camel_decorated())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
    #[inline]
    fn to_lower_camel_delimited<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerCamelIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_lower_camel_delimited())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
}
