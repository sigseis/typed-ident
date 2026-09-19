Syntax definitions for how chunks (runs of non-delimiter characters) should be broken up during segmentation.

There is only one provided boundary implementation, but it is configurable.

* [`Standard`], the provided standard boundary implementation
* [`Options`], options for configuring the provided boundary implementation

## Customizing Boundary Rules

There's a few ways to customize how identifying chunk boundaries works.

1. Provide a different [`Options`] type to the [`Standard`] type.
2. Create your own type which implements the [`Boundary`] trait.

For most use-cases, you will not need to implement your own boundary definition.

### Configuring the `Standard` Implementation

The [`Options`] trait defines the following configurations (based on [Unicode Technical Standard #55](https://www.unicode.org/reports/tr55/#Identifier-Chunks)):

* `CAMEL`: Break based on camel-casing rules
  * *An identifier word boundary exists after a lowercase or non-Greek titlecase letter followed by an uppercase or titlecase letter*
* `HAT`: Break based on hat rules ([Unicode Technical Standard #55](https://www.unicode.org/reports/tr55/#Identifier-Chunks))
  * *An identifier word boundary exists before an uppercase or titlecase letter followed by a lowercase letter, or before a non-Greek titlecase letter*
* `DIGIT_TO_LOWER`: Break if we go from an ASCII digit to a lowercase letter.
* `DIGIT_TO_UPPER`: Break if we go from an ASCII digit to an uppercase or titlecase letter.
* `LOWER_TO_DIGIT`: Break if we go from a lowercase or non-greek titlecase letter to an ASCII digit.
* `UPPER_TO_DIGIT`: Break if we go from an uppercase or greek titlecase letter to an ASCII digit.

Given these options, there are three provided options implementations:

* [`Default`]: Only `CAMEL` and `HAT` boundaries.
* [`AllBoundaries`]: All boundary options are enabled.
* [`NoBoundaries`]: No boundary options are enabled.

However, if you want an obscure combination of these options, you can implement your own options type:

```rust
# use typed_ident::*;
# use typed_ident::syntax::*;
# use typed_ident::syntax::boundary::*;
pub struct DigitsOnly(());

// Any constant you *don't* provide will default to `false`.
impl Options for DigitsOnly {
    const DIGIT_TO_LOWER: bool = true;
    const DIGIT_TO_UPPER: bool = true;
    const LOWER_TO_DIGIT: bool = true;
    const UPPER_TO_DIGIT: bool = true;
}

// Use the custom boundary with the generic presets.
use typed_ident::presets::generic::*;
type LowerCamelIdentFromPresets = LowerCamelIdent<
    profile::Ascii, // Select a character profile.
    DigitsOnly,     // Select a boundary implementation.
>;

// Or you can use it with a custom combination of syntax rules.
type CustomizedLowerCamelIdent = Ident<
    boundary::Standard<DigitsOnly>,
    delimiter::HyphenMinus,
    profile::LowerCamel<profile::Ascii>,
>;
```

[`AllBoundaries`]: crate::syntax::boundary::options::AllBoundaries
[`Boundary`]: crate::syntax::boundary::Boundary
[`Default`]: crate::syntax::boundary::options::Default
[`NoBoundaries`]: crate::syntax::boundary::options::NoBoundaries
[`Options`]: crate::syntax::boundary::options::Options
[`Standard`]: crate::syntax::boundary::Standard

### Creating a Custom Boundary Implementation

You can create your own boundary implementation by creating a new type, and then implementing the [`Boundary`] trait on it. It is recommended that this type is
distinct from the other syntax definition types that you have implemented.

You only need to provide three functions:

* [`Boundary::find_boundary`] returns the next boundary from the start of the string
* [`Boundary::rfind_boundary`] returns the next boundary from the end of the string
* [`Boundary::has_boundary_at`] returns whether or not there is a boundary for some provided chunk string, at some provided byte offset

```rust
# use typed_ident::*;
# use typed_ident::syntax::*;
# use typed_ident::syntax::boundary::*;
# use std::num::NonZero;
// Let's define an example boundary implementation, which inserts a boundary
// before every ASCII digit character (not useful, but simple to implement).
//
// This is only a simple example, a more complete implementation should use the
// `Segmentation` type to ensure that it iterates over graphemes properly.
struct CustomBoundary;
impl Boundary for CustomBoundary {
  fn find_boundary<S: Segmentation>(chunk: &str) -> Option<NonZero<usize>> {
    for (idx, c) in chunk.char_indices().skip(1) {
      if c.is_ascii_digit() {
        return NonZero::new(idx);
      }
    }
    None
  }
  fn rfind_boundary<S: Segmentation>(chunk: &str) -> Option<NonZero<usize>> {
    for (idx, c) in chunk.char_indices().rev() {
      if c.is_ascii_digit() {
        return NonZero::new(idx);
      }
    }
    None
  }
  fn has_boundary_at<S: Segmentation>(chunk: &str, idx: usize) -> bool {
    if idx == 0 {
      return false;
    }
    chunk[idx..].chars().next().is_some_and(|c| c.is_ascii_digit())
  }
}

// You must define a custom type alias when using non-standard boundaries.
type CustomizedLowerCamelIdent = Ident<
    CustomBoundary,
    delimiter::HyphenMinus,
    profile::LowerCamel<profile::Ascii>,
>;
```
