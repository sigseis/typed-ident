// =============================================================================
// TRAIT
// =============================================================================

/// Configures how the default chunk splitting algorithm identifies boundaries.
///
/// For more information, see the [`boundary`](crate::syntax::boundary#options)
/// module.
pub trait Options {
    /// Introduce a boundary based on camel-casing rules
    /// ([Unicode Technical Standard #55](https://www.unicode.org/reports/tr55/#Identifier-Chunks)).
    ///
    /// Specifically: *An identifier word boundary exists after a lowercase or
    /// non-Greek titlecase letter followed by an uppercase or titlecase letter*
    ///
    /// # Example
    ///
    /// ```
    /// # use typed_ident::syntax::{boundary, delimiter, profile};
    /// # use typed_ident::syntax::boundary::Options;
    /// # struct SpecificOption;
    /// # impl Options for SpecificOption {
    /// #     const CAMEL: bool = true;
    /// # }
    /// # type ExampleChunk = typed_ident::Chunk<
    /// #     boundary::Standard<SpecificOption>,
    /// #     delimiter::NotDelimited,
    /// #     profile::Ascii
    /// # >;
    /// let chunk = ExampleChunk::new("camelBoundary")?;
    /// let mut words = chunk.words().type_erased();
    /// assert_eq!(words.next(), Some("camel"));
    /// assert_eq!(words.next(), Some("Boundary"));
    /// assert_eq!(words.next(), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    const CAMEL: bool = false;

    /// Introduces a boundary based on hat rules
    /// ([Unicode Technical Standard #55](https://www.unicode.org/reports/tr55/#Identifier-Chunks)).
    ///
    /// Specifically: *An identifier word boundary exists before an uppercase or
    /// titlecase letter followed by a lowercase letter, or before a non-Greek
    /// titlecase letter*
    ///
    /// # Example
    ///
    /// ```
    /// # use typed_ident::syntax::{boundary, delimiter, profile};
    /// # use typed_ident::syntax::boundary::Options;
    /// # struct SpecificOption;
    /// # impl Options for SpecificOption {
    /// #     const HAT: bool = true;
    /// # }
    /// # type ExampleChunk = typed_ident::Chunk<
    /// #     boundary::Standard<SpecificOption>,
    /// #     delimiter::NotDelimited,
    /// #     profile::Ascii
    /// # >;
    /// let chunk = ExampleChunk::new("HATBoundary")?;
    /// let mut words = chunk.words().type_erased();
    /// assert_eq!(words.next(), Some("HAT"));
    /// assert_eq!(words.next(), Some("Boundary"));
    /// assert_eq!(words.next(), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    const HAT: bool = false;

    /// Introduces a boundary on transition from an ASCII digit to a lowercase
    /// letter.
    ///
    /// # Example
    ///
    /// ```
    /// # use typed_ident::syntax::{boundary, delimiter, profile};
    /// # use typed_ident::syntax::boundary::Options;
    /// # struct SpecificOption;
    /// # impl Options for SpecificOption {
    /// #     const DIGIT_TO_LOWER: bool = true;
    /// # }
    /// # type ExampleChunk = typed_ident::Chunk<
    /// #     boundary::Standard<SpecificOption>,
    /// #     delimiter::NotDelimited,
    /// #     profile::Ascii
    /// # >;
    /// let chunk = ExampleChunk::new("FromDigit123toLower")?;
    /// let mut words = chunk.words().type_erased();
    /// assert_eq!(words.next(), Some("FromDigit123"));
    /// assert_eq!(words.next(), Some("toLower"));
    /// assert_eq!(words.next(), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    const DIGIT_TO_LOWER: bool = false;

    /// Introduces a boundary on transition from an ASCII digit to an uppercase
    /// letter.
    ///
    /// # Example
    ///
    /// ```
    /// # use typed_ident::syntax::{boundary, delimiter, profile};
    /// # use typed_ident::syntax::boundary::Options;
    /// # struct SpecificOption;
    /// # impl Options for SpecificOption {
    /// #     const DIGIT_TO_UPPER: bool = true;
    /// # }
    /// # type ExampleChunk = typed_ident::Chunk<
    /// #     boundary::Standard<SpecificOption>,
    /// #     delimiter::NotDelimited,
    /// #     profile::Ascii
    /// # >;
    /// let chunk = ExampleChunk::new("FromDigit123ToUpper")?;
    /// let mut words = chunk.words().type_erased();
    /// assert_eq!(words.next(), Some("FromDigit123"));
    /// assert_eq!(words.next(), Some("ToUpper"));
    /// assert_eq!(words.next(), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    const DIGIT_TO_UPPER: bool = false;

    /// Introduces a boundary on transition from a lowercase letter to an ASCII
    /// digit.
    ///
    /// # Example
    ///
    /// ```
    /// # use typed_ident::syntax::{boundary, delimiter, profile};
    /// # use typed_ident::syntax::boundary::Options;
    /// # struct SpecificOption;
    /// # impl Options for SpecificOption {
    /// #     const LOWER_TO_DIGIT: bool = true;
    /// # }
    /// # type ExampleChunk = typed_ident::Chunk<
    /// #     boundary::Standard<SpecificOption>,
    /// #     delimiter::NotDelimited,
    /// #     profile::Ascii
    /// # >;
    /// let chunk = ExampleChunk::new("FromLower123ToDigit")?;
    /// let mut words = chunk.words().type_erased();
    /// assert_eq!(words.next(), Some("FromLower"));
    /// assert_eq!(words.next(), Some("123ToDigit"));
    /// assert_eq!(words.next(), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    const LOWER_TO_DIGIT: bool = false;

    /// Introduces a boundary on transition from an uppercase letter to an ASCII
    /// digit.
    ///
    /// # Example
    ///
    /// ```
    /// # use typed_ident::syntax::{boundary, delimiter, profile};
    /// # use typed_ident::syntax::boundary::Options;
    /// # struct SpecificOption;
    /// # impl Options for SpecificOption {
    /// #     const UPPER_TO_DIGIT: bool = true;
    /// # }
    /// # type ExampleChunk = typed_ident::Chunk<
    /// #     boundary::Standard<SpecificOption>,
    /// #     delimiter::NotDelimited,
    /// #     profile::Ascii
    /// # >;
    /// let chunk = ExampleChunk::new("FromUPPER123ToDigit")?;
    /// let mut words = chunk.words().type_erased();
    /// assert_eq!(words.next(), Some("FromUPPER"));
    /// assert_eq!(words.next(), Some("123ToDigit"));
    /// assert_eq!(words.next(), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    const UPPER_TO_DIGIT: bool = false;
}
