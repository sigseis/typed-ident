// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::fmt::AsLowerSnake;
use crate::intermediate::FromIntermediate;
use crate::syntax::Profile;
use crate::{Error, Identifier};
use core::fmt::Write;
use std_alloc::string::String;

// =============================================================================
// TYPE ALIASES
// =============================================================================

// -----------------------------------------------------------------------------
type LowerSnakeIdent<P> = crate::presets::generic::LowerSnakeIdent<<P as Profile>::CharProfile>;

// =============================================================================
// TRAITS
// =============================================================================

/// Provides methods for converting an identifier to lower-snake format, using
/// canonical, or decorated forms.
///
/// For explicit-delimited identifiers like this one, there's no difference
/// between the decorated and delimited forms. So a delimited method is not
/// provided.
///
/// See the [`convert`] module for more details.
///
/// [`convert`]: crate::convert
pub trait ToLowerSnake: Identifier {
    /// Returns a string of the provided input converted to lower snake
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
    ///         .to_lower_snake(),
    ///     Ok(LowerSnakeIdent::new_boxed("lower_camel_case")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_snake(),
    ///     Ok(LowerSnakeIdent::new_boxed("lower_hybrid_case")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_snake(),
    ///     Ok(LowerSnakeIdent::new_boxed("lower_kebab_case")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_snake(),
    ///     Ok(LowerSnakeIdent::new_boxed("lower_snake_case")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_snake(),
    ///     Ok(LowerSnakeIdent::new_boxed("upper_camel_case")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_snake(),
    ///     Ok(LowerSnakeIdent::new_boxed("upper_hybrid_case")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_snake(),
    ///     Ok(LowerSnakeIdent::new_boxed("upper_kebab_case")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_snake(),
    ///     Ok(LowerSnakeIdent::new_boxed("upper_snake_case")?),
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
    ///         .to_lower_snake(),
    ///     Ok(LowerSnakeIdent::new_boxed("_")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .to_lower_snake(),
    ///     Ok(LowerSnakeIdent::new_boxed("_2_example_camel")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated result, the original identifier is unmodified"]
    fn to_lower_snake<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerSnakeIdent<Self::Profile>>;

    /// Returns a string of the provided input converted to lower snake
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
    ///         .to_lower_snake_decorated(),
    ///     Ok(LowerSnakeIdent::new_boxed("__lower__camel_case__")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_snake_decorated(),
    ///     Ok(LowerSnakeIdent::new_boxed("__lower__hybrid_case__")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_snake_decorated(),
    ///     Ok(LowerSnakeIdent::new_boxed("__lower__kebab_case__")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_snake_decorated(),
    ///     Ok(LowerSnakeIdent::new_boxed("__lower__snake_case__")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_snake_decorated(),
    ///     Ok(LowerSnakeIdent::new_boxed("__upper__camel_case__")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_snake_decorated(),
    ///     Ok(LowerSnakeIdent::new_boxed("__upper__hybrid_case__")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_snake_decorated(),
    ///     Ok(LowerSnakeIdent::new_boxed("__upper__kebab_case__")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_snake_decorated(),
    ///     Ok(LowerSnakeIdent::new_boxed("__upper__snake_case__")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated result, the original identifier is unmodified"]
    fn to_lower_snake_decorated<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerSnakeIdent<Self::Profile>>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<I: Identifier + ?Sized> ToLowerSnake for I {
    #[inline]
    fn to_lower_snake<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerSnakeIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_lower_snake())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
    #[inline]
    fn to_lower_snake_decorated<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<LowerSnakeIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_lower_snake_decorated())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
}
