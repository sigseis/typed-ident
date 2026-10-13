// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::fmt::AsUpperSnake;
use crate::intermediate::FromIntermediate;
use crate::syntax::Profile;
use crate::{Error, Identifier};
use core::fmt::Write;
use std_alloc::string::String;

// =============================================================================
// TYPE ALIASES
// =============================================================================

// -----------------------------------------------------------------------------
type UpperSnakeIdent<P> = crate::presets::generic::UpperSnakeIdent<<P as Profile>::CharProfile>;

// =============================================================================
// TRAITS
// =============================================================================

/// Provides methods for converting an identifier to upper-snake format, using
/// canonical, or decorated forms.
///
/// For explicit-delimited identifiers like this one, there's no difference
/// between the decorated and delimited forms. So a delimited method is not
/// provided.
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
pub trait ToUpperSnake: Identifier {
    /// Returns a string of the provided input converted to upper snake
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
    ///         .to_upper_snake(),
    ///     Ok(UpperSnakeIdent::new_boxed("LOWER_CAMEL_CASE")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_upper_snake(),
    ///     Ok(UpperSnakeIdent::new_boxed("LOWER_HYBRID_CASE")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_upper_snake(),
    ///     Ok(UpperSnakeIdent::new_boxed("LOWER_KEBAB_CASE")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_upper_snake(),
    ///     Ok(UpperSnakeIdent::new_boxed("LOWER_SNAKE_CASE")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_upper_snake(),
    ///     Ok(UpperSnakeIdent::new_boxed("UPPER_CAMEL_CASE")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_upper_snake(),
    ///     Ok(UpperSnakeIdent::new_boxed("UPPER_HYBRID_CASE")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_upper_snake(),
    ///     Ok(UpperSnakeIdent::new_boxed("UPPER_KEBAB_CASE")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_upper_snake(),
    ///     Ok(UpperSnakeIdent::new_boxed("UPPER_SNAKE_CASE")?),
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
    ///         .to_upper_snake(),
    ///     Ok(UpperSnakeIdent::new_boxed("_")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .to_upper_snake(),
    ///     Ok(UpperSnakeIdent::new_boxed("_2_EXAMPLE_CAMEL")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_snake<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperSnakeIdent<Self::Profile>>;

    /// Returns a string of the provided input converted to upper snake
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
    ///         .to_upper_snake_decorated(),
    ///     Ok(UpperSnakeIdent::new_boxed("__LOWER__CAMEL_CASE__")?),
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_upper_snake_decorated(),
    ///     Ok(UpperSnakeIdent::new_boxed("__LOWER__HYBRID_CASE__")?),
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_upper_snake_decorated(),
    ///     Ok(UpperSnakeIdent::new_boxed("__LOWER__KEBAB_CASE__")?),
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_upper_snake_decorated(),
    ///     Ok(UpperSnakeIdent::new_boxed("__LOWER__SNAKE_CASE__")?),
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_upper_snake_decorated(),
    ///     Ok(UpperSnakeIdent::new_boxed("__UPPER__CAMEL_CASE__")?),
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_upper_snake_decorated(),
    ///     Ok(UpperSnakeIdent::new_boxed("__UPPER__HYBRID_CASE__")?),
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_upper_snake_decorated(),
    ///     Ok(UpperSnakeIdent::new_boxed("__UPPER__KEBAB_CASE__")?),
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_upper_snake_decorated(),
    ///     Ok(UpperSnakeIdent::new_boxed("__UPPER__SNAKE_CASE__")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_snake_decorated<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperSnakeIdent<Self::Profile>>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<I: Identifier + ?Sized> ToUpperSnake for I {
    #[inline]
    fn to_upper_snake<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperSnakeIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_upper_snake())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
    #[inline]
    fn to_upper_snake_decorated<T>(&self) -> Result<T, Error>
    where
        T: FromIntermediate<UpperSnakeIdent<Self::Profile>>,
    {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_upper_snake_decorated())
            .expect("failed to format an identifier into a string");
        T::from_intermediate(string)
    }
}
