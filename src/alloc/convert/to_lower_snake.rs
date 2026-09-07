// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Ident;
use crate::core::fmt::AsLowerSnake;
use crate::syntax::boundary::Boundary;
use crate::syntax::delimiter::Delimiter;
use crate::syntax::profile::Profile;
use core::fmt::Write;
use std_alloc::string::String;

// =============================================================================
// TRAITS
// =============================================================================

/// Converts an identifier into a preset lower-snake identifier string.
///
/// See the [`convert`] module for more details.
///
/// [`convert`]: crate::alloc::convert
///
/// # Examples
///
/// ```
/// # use typed_ident::alloc::convert::*;
/// # use typed_ident::presets::unicode::*;
/// assert_eq!(
///     LowerCamelIdent::new("lowerCamel")?.to_lower_snake_canonical(),
///     "lower_camel"
/// );
/// assert_eq!(
///     LowerHybridIdent::new("lowerHybrid")?.to_lower_snake_canonical(),
///     "lower_hybrid"
/// );
/// assert_eq!(
///     LowerKebabIdent::new("lower-kebab")?.to_lower_snake_canonical(),
///     "lower_kebab"
/// );
/// assert_eq!(
///     LowerSnakeIdent::new("lower_snake")?.to_lower_snake_canonical(),
///     "lower_snake"
/// );
/// assert_eq!(
///     UpperCamelIdent::new("UpperCamel")?.to_lower_snake_canonical(),
///     "upper_camel"
/// );
/// assert_eq!(
///     UpperHybridIdent::new("UpperHybrid")?.to_lower_snake_canonical(),
///     "upper_hybrid"
/// );
/// assert_eq!(
///     UpperKebabIdent::new("UPPER-KEBAB")?.to_lower_snake_canonical(),
///     "upper_kebab"
/// );
/// assert_eq!(
///     UpperSnakeIdent::new("UPPER_SNAKE")?.to_lower_snake_canonical(),
///     "upper_snake"
/// );
/// # Ok::<(), typed_ident::Error>(())
/// ```
pub trait ToLowerSnake {
    /// Returns a displayable type that converts the provided input to lower
    /// snake in canonical form.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_lower_snake_canonical(),
    ///     "lower_camel_case"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_snake_canonical(),
    ///     "lower_hybrid_case"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_snake_canonical(),
    ///     "lower_kebab_case"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_snake_canonical(),
    ///     "lower_snake_case"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_snake_canonical(),
    ///     "upper_camel_case"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_snake_canonical(),
    ///     "upper_hybrid_case"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_snake_canonical(),
    ///     "upper_kebab_case"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_snake_canonical(),
    ///     "upper_snake_case"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Notice that it will attempt to keep necessary prefix delimiters.
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .to_lower_snake_canonical(),
    ///     "_"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .to_lower_snake_canonical(),
    ///     "_2_example_camel"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_snake_canonical(&self) -> String;

    /// Returns a displayable type that converts the provided input to lower
    /// snake in decorated form.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_lower_snake_decorated(),
    ///     "__lower__camel_case__"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_snake_decorated(),
    ///     "__lower__hybrid_case__"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_snake_decorated(),
    ///     "__lower__kebab_case__"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_snake_decorated(),
    ///     "__lower__snake_case__"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_snake_decorated(),
    ///     "__upper__camel_case__"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_snake_decorated(),
    ///     "__upper__hybrid_case__"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_snake_decorated(),
    ///     "__upper__kebab_case__"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_snake_decorated(),
    ///     "__upper__snake_case__"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_snake_decorated(&self) -> String;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> ToLowerSnake for Ident<B, D, P> {
    #[inline]
    fn to_lower_snake_canonical(&self) -> String {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_lower_snake_canonical())
            .expect("failed to format an identifier into a string");
        string
    }
    #[inline]
    fn to_lower_snake_decorated(&self) -> String {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_lower_snake_decorated())
            .expect("failed to format an identifier into a string");
        string
    }
}
