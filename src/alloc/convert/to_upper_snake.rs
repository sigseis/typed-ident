// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Ident;
use crate::core::fmt::AsUpperSnake;
use crate::syntax::boundary::Boundary;
use crate::syntax::delimiter::Delimiter;
use crate::syntax::profile::Profile;
use core::fmt::Write;
use std_alloc::string::String;

// =============================================================================
// TRAITS
// =============================================================================

/// Converts an identifier into a preset upper-snake identifier string.
///
/// See the [`convert`] module for more details.
///
/// [`convert`]: crate::alloc::convert
pub trait ToUpperSnake {
    /// Returns a displayable type that converts the provided input to upper
    /// snake in plain form.
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
    ///         .to_upper_snake(),
    ///     "LOWER_CAMEL_CASE"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_upper_snake(),
    ///     "LOWER_HYBRID_CASE"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_upper_snake(),
    ///     "LOWER_KEBAB_CASE"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_upper_snake(),
    ///     "LOWER_SNAKE_CASE"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_upper_snake(),
    ///     "UPPER_CAMEL_CASE"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_upper_snake(),
    ///     "UPPER_HYBRID_CASE"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_upper_snake(),
    ///     "UPPER_KEBAB_CASE"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_upper_snake(),
    ///     "UPPER_SNAKE_CASE"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// This form will *NOT* attempt to validate the first character to ensure
    /// it could be considered a valid identifier (see
    /// [`to_upper_snake_canonical`] for a version that would validate this).
    ///
    /// [`to_upper_snake_canonical`]: Self::to_upper_snake_canonical
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .to_upper_snake(),
    ///     ""
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .to_upper_snake(),
    ///     "2_EXAMPLE_CAMEL"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_snake(&self) -> String;

    /// Returns a displayable type that converts the provided input to upper
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
    ///         .to_upper_snake_canonical(),
    ///     "LOWER_CAMEL_CASE"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_upper_snake_canonical(),
    ///     "LOWER_HYBRID_CASE"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_upper_snake_canonical(),
    ///     "LOWER_KEBAB_CASE"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_upper_snake_canonical(),
    ///     "LOWER_SNAKE_CASE"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_upper_snake_canonical(),
    ///     "UPPER_CAMEL_CASE"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_upper_snake_canonical(),
    ///     "UPPER_HYBRID_CASE"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_upper_snake_canonical(),
    ///     "UPPER_KEBAB_CASE"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_upper_snake_canonical(),
    ///     "UPPER_SNAKE_CASE"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// This form *WILL* attempt to validate the first character to ensure it
    /// could be considered a valid identifier (see [`to_upper_snake`] for a
    /// version that would *NOT* validate this).
    ///
    /// [`to_upper_snake`]: Self::to_upper_snake
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     UpperCamelIdent::new("_____")?
    ///         .to_upper_snake_canonical(),
    ///     "_"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .to_upper_snake_canonical(),
    ///     "_2_EXAMPLE_CAMEL"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_snake_canonical(&self) -> String;

    /// Returns a displayable type that converts the provided input to upper
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
    ///         .to_upper_snake_decorated(),
    ///     "__LOWER__CAMEL_CASE__"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_upper_snake_decorated(),
    ///     "__LOWER__HYBRID_CASE__"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_upper_snake_decorated(),
    ///     "__LOWER__KEBAB_CASE__"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_upper_snake_decorated(),
    ///     "__LOWER__SNAKE_CASE__"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_upper_snake_decorated(),
    ///     "__UPPER__CAMEL_CASE__"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_upper_snake_decorated(),
    ///     "__UPPER__HYBRID_CASE__"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_upper_snake_decorated(),
    ///     "__UPPER__KEBAB_CASE__"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_upper_snake_decorated(),
    ///     "__UPPER__SNAKE_CASE__"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_snake_decorated(&self) -> String;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> ToUpperSnake for Ident<B, D, P> {
    #[inline]
    fn to_upper_snake(&self) -> String {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_upper_snake())
            .expect("failed to format an identifier into a string");
        string
    }
    #[inline]
    fn to_upper_snake_canonical(&self) -> String {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_upper_snake_canonical())
            .expect("failed to format an identifier into a string");
        string
    }
    #[inline]
    fn to_upper_snake_decorated(&self) -> String {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_upper_snake_decorated())
            .expect("failed to format an identifier into a string");
        string
    }
}
