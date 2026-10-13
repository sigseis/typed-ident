// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::fmt::AsUpperKebab;
use crate::intermediate::FromIntermediate;
use crate::syntax::Profile;
use crate::{Error, Identifier};
use core::fmt::Write;
use std_alloc::string::String;

// =============================================================================
// TYPE ALIASES
// =============================================================================

// -----------------------------------------------------------------------------
type UpperKebabIdent<P> = crate::presets::generic::UpperKebabIdent<<P as Profile>::CharProfile>;

// =============================================================================
// TRAITS
// =============================================================================

/// Provides methods for converting an identifier to upper-kebab format, using
/// canonical, or decorated forms.
///
/// For explicit-delimited identifiers like this one, there's no difference
/// between the decorated and delimited forms. So a delimited method is not
/// provided.
///
/// See the [`convert`] module for more details.
///
/// [`convert`]: crate::convert
pub trait ToUpperKebab: Identifier {
    /// Returns a string of the provided input converted to upper kebab
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
    ///         .to_upper_kebab(),
    ///     Ok(UpperKebabIdent::new_boxed("LOWER-CAMEL-CASE")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_upper_kebab(),
    ///     Ok(UpperKebabIdent::new_boxed("LOWER-HYBRID-CASE")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_upper_kebab(),
    ///     Ok(UpperKebabIdent::new_boxed("LOWER-KEBAB-CASE")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_upper_kebab(),
    ///     Ok(UpperKebabIdent::new_boxed("LOWER-SNAKE-CASE")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_upper_kebab(),
    ///     Ok(UpperKebabIdent::new_boxed("UPPER-CAMEL-CASE")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_upper_kebab(),
    ///     Ok(UpperKebabIdent::new_boxed("UPPER-HYBRID-CASE")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_upper_kebab(),
    ///     Ok(UpperKebabIdent::new_boxed("UPPER-KEBAB-CASE")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_upper_kebab(),
    ///     Ok(UpperKebabIdent::new_boxed("UPPER-SNAKE-CASE")?),
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
    ///         .to_upper_kebab(),
    ///     Ok(UpperKebabIdent::new_boxed("-")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .to_upper_kebab(),
    ///     Ok(UpperKebabIdent::new_boxed("-2-EXAMPLE-CAMEL")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated result, the original identifier is unmodified"]
    fn to_upper_kebab<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperKebabIdent<Self::Profile>>;

    /// Returns a string of the provided input converted to upper kebab
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
    ///         .to_upper_kebab_decorated(),
    ///     Ok(UpperKebabIdent::new_boxed("--LOWER--CAMEL-CASE--")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_upper_kebab_decorated(),
    ///     Ok(UpperKebabIdent::new_boxed("--LOWER--HYBRID-CASE--")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_upper_kebab_decorated(),
    ///     Ok(UpperKebabIdent::new_boxed("--LOWER--KEBAB-CASE--")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_upper_kebab_decorated(),
    ///     Ok(UpperKebabIdent::new_boxed("--LOWER--SNAKE-CASE--")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_upper_kebab_decorated(),
    ///     Ok(UpperKebabIdent::new_boxed("--UPPER--CAMEL-CASE--")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_upper_kebab_decorated(),
    ///     Ok(UpperKebabIdent::new_boxed("--UPPER--HYBRID-CASE--")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_upper_kebab_decorated(),
    ///     Ok(UpperKebabIdent::new_boxed("--UPPER--KEBAB-CASE--")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_upper_kebab_decorated(),
    ///     Ok(UpperKebabIdent::new_boxed("--UPPER--SNAKE-CASE--")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated result, the original identifier is unmodified"]
    fn to_upper_kebab_decorated<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperKebabIdent<Self::Profile>>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<I: Identifier + ?Sized> ToUpperKebab for I {
    #[inline]
    fn to_upper_kebab<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperKebabIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_upper_kebab())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
    #[inline]
    fn to_upper_kebab_decorated<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperKebabIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_upper_kebab_decorated())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
}
