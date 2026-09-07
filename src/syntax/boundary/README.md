Syntax definitions for how chunks (runs of non-delimiter characters) should be broken up during segmentation.

There is only one provided boundary implementation, but it is configurable.

* [`Standard`], the provided standard boundary implementation.
* [`Options`], options for configuring the provided boundary implementation.

## Customizing Boundary Rules

There's two primary ways to customize how identifying chunk boundaries works.

1. Provide a custom [`Options`] type to the [`Standard`] type.
2. Create your own type which implements the [`Boundary`] trait.

For most use-cases, you will not need to implement your own boundary definition.

### Options

The [`Options`] trait defines the following configurations:

* `CAMEL`: Break based on camel-casing rules ([Unicode Technical Standard #55](https://www.unicode.org/reports/tr55/#Identifier-Chunks))
  * *An identifier word boundary exists after a lowercase or non-Greek titlecase letter followed by an uppercase or titlecase letter*
* `HAT`: Break based on hat rules ([Unicode Technical Standard #55](https://www.unicode.org/reports/tr55/#Identifier-Chunks))
  * *An identifier word boundary exists before an uppercase or titlecase letter followed by a lowercase letter, or before a non-Greek titlecase letter*
* `DIGIT_TO_LOWER`: Break if we go from an ASCII digit to a lowercase letter.
* `DIGIT_TO_UPPER`: Break if we go from an ASCII digit to an uppercase or titlecase letter.
* `LOWER_TO_DIGIT`: Break if we go from a lowercase or non-greek titlecase letter to an ASCII digit.
* `UPPER_TO_DIGIT`: Break if we go from an uppercase or greek titlecase letter to an ASCII digit.

Usually, the provided implementations ([`Default`], [`AllBoundaries`], [`NoBoundaries`]) are enough. However, if you want your own custom combination of options, you can get that by implementing this trait on some new type you make.

```rust
# use typed_ident::*;
# use typed_ident::syntax::*;
# use typed_ident::syntax::boundary::*;
pub struct DigitsOnly(());
impl Options for DigitsOnly {
    const DIGIT_TO_LOWER: bool = true;
    const DIGIT_TO_UPPER: bool = true;
    const LOWER_TO_DIGIT: bool = true;
    const UPPER_TO_DIGIT: bool = true;
}

// Using the custom boundary:
type CustomIdent = Ident<
    boundary::Standard<DigitsOnly>,
    delimiter::LowLine,
    profile::Unicode,
>;
```

Note that any constant you *don't* provide will default to `false`.

[`AllBoundaries`]: crate::syntax::boundary::options::AllBoundaries
[`Boundary`]: crate::syntax::boundary::Boundary
[`Default`]: crate::syntax::boundary::options::Default
[`NoBoundaries`]: crate::syntax::boundary::options::NoBoundaries
[`Options`]: crate::syntax::boundary::options::Options
[`Standard`]: crate::syntax::boundary::Standard
