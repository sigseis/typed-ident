// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::fmt::AsUpperCamel;
use crate::intermediate::FromIntermediate;
use crate::syntax::Profile;
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
/// [`convert`]: crate::convert
pub trait ToUpperCamel: Identifier {
    /// Returns a string of the provided input converted to upper camel
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
    #[must_use = "format conversion returns a newly-allocated result, the original identifier is unmodified"]
    fn to_upper_camel<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperCamelIdent<Self::Profile>>;

    /// Returns a string of the provided input converted to upper camel
    /// decorated form.
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
    #[must_use = "format conversion returns a newly-allocated result, the original identifier is unmodified"]
    fn to_upper_camel_decorated<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperCamelIdent<Self::Profile>>;

    /// Returns a string of the provided input converted to upper camel
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
    #[must_use = "format conversion returns a newly-allocated result, the original identifier is unmodified"]
    fn to_upper_camel_delimited<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperCamelIdent<Self::Profile>>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<I: Identifier + ?Sized> ToUpperCamel for I {
    #[inline]
    fn to_upper_camel<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperCamelIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_upper_camel())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
    #[inline]
    fn to_upper_camel_decorated<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperCamelIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_upper_camel_decorated())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
    #[inline]
    fn to_upper_camel_delimited<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperCamelIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_upper_camel_delimited())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
}
