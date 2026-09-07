// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Ident;
use crate::core::fmt::AsLowerKebab;
use crate::syntax::boundary::Boundary;
use crate::syntax::delimiter::Delimiter;
use crate::syntax::profile::Profile;
use core::fmt::Write;
use std_alloc::string::String;

// =============================================================================
// TRAITS
// =============================================================================

/// Converts an identifier into a preset lower-kebab identifier string.
///
/// See the [`convert`] module for more details.
///
/// [`convert`]: crate::alloc::convert
pub trait ToLowerKebab {
    /// Returns a displayable type that converts the provided input to lower
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
    ///         .to_lower_kebab_canonical(),
    ///     "lower-camel-case"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_kebab_canonical(),
    ///     "lower-hybrid-case"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_kebab_canonical(),
    ///     "lower-kebab-case"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_kebab_canonical(),
    ///     "lower-snake-case"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_kebab_canonical(),
    ///     "upper-camel-case"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_kebab_canonical(),
    ///     "upper-hybrid-case"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_kebab_canonical(),
    ///     "upper-kebab-case"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_kebab_canonical(),
    ///     "upper-snake-case"
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
    ///         .to_lower_kebab_canonical(),
    ///     "-"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__2__Example__Camel__")?
    ///         .to_lower_kebab_canonical(),
    ///     "-2-example-camel"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_kebab_canonical(&self) -> String;

    /// Returns a displayable type that converts the provided input to lower
    /// kebab in decorated form.
    ///
    /// See the [`fmt`] module documentation for details on different forms.
    ///
    /// [`fmt`]: crate::core::fmt
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::alloc::convert::*;
    /// # use typed_ident::presets::unicode::*;
    /// assert_eq!(
    ///     LowerCamelIdent::new("__lower__camel_case__")?
    ///         .to_lower_kebab_decorated(),
    ///     "--lower--camel-case--"
    /// );
    /// assert_eq!(
    ///     LowerHybridIdent::new("_-lower-_hybrid-case-_")?
    ///         .to_lower_kebab_decorated(),
    ///     "--lower--hybrid-case--"
    /// );
    /// assert_eq!(
    ///     LowerKebabIdent::new("--lower--kebab-case--")?
    ///         .to_lower_kebab_decorated(),
    ///     "--lower--kebab-case--"
    /// );
    /// assert_eq!(
    ///     LowerSnakeIdent::new("__lower__snake_case__")?
    ///         .to_lower_kebab_decorated(),
    ///     "--lower--snake-case--"
    /// );
    /// assert_eq!(
    ///     UpperCamelIdent::new("__Upper__Camel_Case__")?
    ///         .to_lower_kebab_decorated(),
    ///     "--upper--camel-case--"
    /// );
    /// assert_eq!(
    ///     UpperHybridIdent::new("-_Upper_-Hybrid_Case_-")?
    ///         .to_lower_kebab_decorated(),
    ///     "--upper--hybrid-case--"
    /// );
    /// assert_eq!(
    ///     UpperKebabIdent::new("--UPPER--KEBAB-CASE--")?
    ///         .to_lower_kebab_decorated(),
    ///     "--upper--kebab-case--"
    /// );
    /// assert_eq!(
    ///     UpperSnakeIdent::new("__UPPER__SNAKE_CASE__")?
    ///         .to_lower_kebab_decorated(),
    ///     "--upper--snake-case--"
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "format conversion returns a newly-allocated string, the original identifier is unmodified"]
    fn to_lower_kebab_decorated(&self) -> String;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> ToLowerKebab for Ident<B, D, P> {
    #[inline]
    fn to_lower_kebab_canonical(&self) -> String {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_lower_kebab_canonical())
            .expect("failed to format an identifier into a string");
        string
    }
    #[inline]
    fn to_lower_kebab_decorated(&self) -> String {
        let mut string = String::with_capacity(self.len());
        write!(string, "{}", self.as_lower_kebab_decorated())
            .expect("failed to format an identifier into a string");
        string
    }
}
