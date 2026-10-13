// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::fmt::AsLowerKebab;
use crate::intermediate::FromIntermediate;
use crate::syntax::Profile;
use crate::{Error, Identifier};
use core::fmt::Write;
use std_alloc::string::String;

// =============================================================================
// TYPE ALIASES
// =============================================================================

// -----------------------------------------------------------------------------
type LowerKebabIdent<P> = crate::presets::generic::LowerKebabIdent<<P as Profile>::CharProfile>;

// =============================================================================
// TRAITS
// =============================================================================

/// Provides methods for converting an identifier to lower-kebab format, using
/// canonical, or decorated forms.
///
/// For explicit-delimited identifiers like this one, there's no difference
/// between the decorated and delimited forms. So a delimited method is not
/// provided.
///
/// See the [`convert`] module for more details.
///
/// [`convert`]: crate::convert
pub trait ToLowerKebab: Identifier {
    /// Returns a string of the provided input converted to lower kebab
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
    ///         .to_lower_kebab(),
    ///     Ok(LowerKebabIdent::new_boxed("lower-camel-case")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_kebab(),
    ///     Ok(LowerKebabIdent::new_boxed("lower-hybrid-case")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_kebab(),
    ///     Ok(LowerKebabIdent::new_boxed("lower-kebab-case")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_kebab(),
    ///     Ok(LowerKebabIdent::new_boxed("lower-snake-case")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_kebab(),
    ///     Ok(LowerKebabIdent::new_boxed("upper-camel-case")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_kebab(),
    ///     Ok(LowerKebabIdent::new_boxed("upper-hybrid-case")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_kebab(),
    ///     Ok(LowerKebabIdent::new_boxed("upper-kebab-case")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_kebab(),
    ///     Ok(LowerKebabIdent::new_boxed("upper-snake-case")?),
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
    ///         .to_lower_kebab(),
    ///     Ok(LowerKebabIdent::new_boxed("-")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .to_lower_kebab(),
    ///     Ok(LowerKebabIdent::new_boxed("-2-example-camel")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated result, the original identifier is unmodified"]
    fn to_lower_kebab<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerKebabIdent<Self::Profile>>;

    /// Returns a string of the provided input converted to lower kebab
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
    ///         .to_lower_kebab_decorated(),
    ///     Ok(LowerKebabIdent::new_boxed("--lower--camel-case--")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_kebab_decorated(),
    ///     Ok(LowerKebabIdent::new_boxed("--lower--hybrid-case--")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_kebab_decorated(),
    ///     Ok(LowerKebabIdent::new_boxed("--lower--kebab-case--")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_kebab_decorated(),
    ///     Ok(LowerKebabIdent::new_boxed("--lower--snake-case--")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_kebab_decorated(),
    ///     Ok(LowerKebabIdent::new_boxed("--upper--camel-case--")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_kebab_decorated(),
    ///     Ok(LowerKebabIdent::new_boxed("--upper--hybrid-case--")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_kebab_decorated(),
    ///     Ok(LowerKebabIdent::new_boxed("--upper--kebab-case--")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_kebab_decorated(),
    ///     Ok(LowerKebabIdent::new_boxed("--upper--snake-case--")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated result, the original identifier is unmodified"]
    fn to_lower_kebab_decorated<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerKebabIdent<Self::Profile>>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<I: Identifier + ?Sized> ToLowerKebab for I {
    #[inline]
    fn to_lower_kebab<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerKebabIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_lower_kebab())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
    #[inline]
    fn to_lower_kebab_decorated<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerKebabIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_lower_kebab_decorated())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
}
