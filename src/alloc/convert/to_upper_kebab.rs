// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Ident;
use crate::core::fmt::AsUpperKebab;
use crate::syntax::boundary::Boundary;
use crate::syntax::delimiter::Delimiter;
use crate::syntax::profile::Profile;
use core::fmt::Write;
use std_alloc::string::String;

// =============================================================================
// TRAITS
// =============================================================================

/// Converts an identifier into a preset upper-kebab identifier string.
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
///     LowerCamelIdent::new("lowerCamel")?.to_upper_kebab_canonical(),
///     "LOWER-CAMEL"
/// );
/// assert_eq!(
///     LowerHybridIdent::new("lowerHybrid")?.to_upper_kebab_canonical(),
///     "LOWER-HYBRID"
/// );
/// assert_eq!(
///     LowerKebabIdent::new("lower-kebab")?.to_upper_kebab_canonical(),
///     "LOWER-KEBAB"
/// );
/// assert_eq!(
///     LowerSnakeIdent::new("lower_snake")?.to_upper_kebab_canonical(),
///     "LOWER-SNAKE"
/// );
/// assert_eq!(
///     UpperCamelIdent::new("UpperCamel")?.to_upper_kebab_canonical(),
///     "UPPER-CAMEL"
/// );
/// assert_eq!(
///     UpperHybridIdent::new("UpperHybrid")?.to_upper_kebab_canonical(),
///     "UPPER-HYBRID"
/// );
/// assert_eq!(
///     UpperKebabIdent::new("UPPER-KEBAB")?.to_upper_kebab_canonical(),
///     "UPPER-KEBAB"
/// );
/// assert_eq!(
///     UpperSnakeIdent::new("UPPER_SNAKE")?.to_upper_kebab_canonical(),
///     "UPPER-SNAKE"
/// );
/// # Ok::<(), typed_ident::Error>(())
/// ```
pub trait ToUpperKebab {
    /// Returns a displayable type that converts the provided input to upper
    /// kebab in canonical form.
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
    ///         .to_upper_kebab_canonical(),
    ///     "LOWER-CAMEL-CASE"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_upper_kebab_canonical(),
    ///     "LOWER-HYBRID-CASE"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_upper_kebab_canonical(),
    ///     "LOWER-KEBAB-CASE"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_upper_kebab_canonical(),
    ///     "LOWER-SNAKE-CASE"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_upper_kebab_canonical(),
    ///     "UPPER-CAMEL-CASE"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_upper_kebab_canonical(),
    ///     "UPPER-HYBRID-CASE"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_upper_kebab_canonical(),
    ///     "UPPER-KEBAB-CASE"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_upper_kebab_canonical(),
    ///     "UPPER-SNAKE-CASE"
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
    ///         .to_upper_kebab_canonical(),
    ///     "-"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .to_upper_kebab_canonical(),
    ///     "-2-EXAMPLE-CAMEL"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_kebab_canonical(&self) -> String;

    /// Returns a displayable type that converts the provided input to upper
    /// kebab in decorated form.
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
    ///         .to_upper_kebab_decorated(),
    ///     "--LOWER--CAMEL-CASE--"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_upper_kebab_decorated(),
    ///     "--LOWER--HYBRID-CASE--"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_upper_kebab_decorated(),
    ///     "--LOWER--KEBAB-CASE--"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_upper_kebab_decorated(),
    ///     "--LOWER--SNAKE-CASE--"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_upper_kebab_decorated(),
    ///     "--UPPER--CAMEL-CASE--"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_upper_kebab_decorated(),
    ///     "--UPPER--HYBRID-CASE--"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_upper_kebab_decorated(),
    ///     "--UPPER--KEBAB-CASE--"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_upper_kebab_decorated(),
    ///     "--UPPER--SNAKE-CASE--"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_upper_kebab_decorated(&self) -> String;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> ToUpperKebab for Ident<B, D, P> {
    #[inline]
    fn to_upper_kebab_canonical(&self) -> String {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_upper_kebab_canonical())
            .expect("failed to format an identifier into a string");
        string
    }
    #[inline]
    fn to_upper_kebab_decorated(&self) -> String {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_upper_kebab_decorated())
            .expect("failed to format an identifier into a string");
        string
    }
}
