// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::fmt::AsUpperHybrid;
use crate::intermediate::FromIntermediate;
use crate::syntax::Profile;
use crate::syntax::delimiter::AsciiFlatLine;
use crate::{Error, Identifier};
use core::fmt::Write;
use std_alloc::string::String;

// =============================================================================
// TYPE ALIASES
// =============================================================================

// -----------------------------------------------------------------------------
type UpperHybridIdent<P> = crate::presets::generic::UpperHybridIdent<<P as Profile>::CharProfile>;

// =============================================================================
// TRAITS
// =============================================================================

/// Provides methods for converting an identifier to upper-hybrid format, using
/// canonical, decorated, or delimited forms.
///
/// See the [`convert`] module for more details.
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
pub trait ToUpperHybrid: Identifier {
    /// Returns a string of the provided input converted to upper hybrid
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
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("LowerCamelCase")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("LowerHybridCase")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("LowerKebabCase")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("LowerSnakeCase")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("UpperCamelCase")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("UpperHybridCase")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("UpperKebabCase")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("UpperSnakeCase")?),
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
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("_")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .to_upper_hybrid(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("_2ExampleCamel")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated result, the original identifier is unmodified"]
    fn to_upper_hybrid<T>(&self, default_delim: AsciiFlatLine) -> Result<T, Error>
    where
        T: FromIntermediate<UpperHybridIdent<Self::Profile>>;

    /// Returns a string of the provided input converted to upper hybrid
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
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_upper_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("__Lower__CamelCase__")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_upper_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("_-Lower-_HybridCase-_")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_upper_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("--Lower--KebabCase--")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_upper_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("__Lower__SnakeCase__")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_upper_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("__Upper__CamelCase__")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_upper_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("-_Upper_-HybridCase_-")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_upper_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("--Upper--KebabCase--")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_upper_hybrid_decorated(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("__Upper__SnakeCase__")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated result, the original identifier is unmodified"]
    fn to_upper_hybrid_decorated<T>(&self, default_delim: AsciiFlatLine) -> Result<T, Error>
    where
        T: FromIntermediate<UpperHybridIdent<Self::Profile>>;

    /// Returns a string of the provided input converted to upper hybrid
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
    /// # use typed_ident::syntax::delimiter::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_upper_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("__Lower__Camel_Case__")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_upper_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("_-Lower-_Hybrid-Case-_")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_upper_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("--Lower--Kebab-Case--")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_upper_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("__Lower__Snake_Case__")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_upper_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("__Upper__Camel_Case__")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_upper_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("-_Upper_-Hybrid_Case_-")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_upper_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("--Upper--Kebab-Case--")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_upper_hybrid_delimited(AsciiFlatLine::LowLine),
    ///     Ok(UpperHybridIdent::new_boxed("__Upper__Snake_Case__")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated result, the original identifier is unmodified"]
    fn to_upper_hybrid_delimited<T>(&self, default_delim: AsciiFlatLine) -> Result<T, Error>
    where
        T: FromIntermediate<UpperHybridIdent<Self::Profile>>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<I: Identifier + ?Sized> ToUpperHybrid for I {
    #[inline]
    fn to_upper_hybrid<T>(&self, default_delim: AsciiFlatLine) -> Result<T, Error>
    where
        T: FromIntermediate<UpperHybridIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_upper_hybrid(default_delim))
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
    #[inline]
    fn to_upper_hybrid_decorated<T>(&self, default_delim: AsciiFlatLine) -> Result<T, Error>
    where
        T: FromIntermediate<UpperHybridIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_upper_hybrid_decorated(default_delim))
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
    #[inline]
    fn to_upper_hybrid_delimited<T>(&self, default_delim: AsciiFlatLine) -> Result<T, Error>
    where
        T: FromIntermediate<UpperHybridIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_upper_hybrid_delimited(default_delim))
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
}
