## Format Table

There are four predefined identifier formats, each available with different case restrictions.

| **Mixed**       | **Cased**            | **Lower** or **Upper**                      |
|-----------------|----------------------|---------------------------------------------|
| [`CamelIdent`]  | [`CasedCamelIdent`]  | [`LowerCamelIdent`]  / [`UpperCamelIdent`]  |
| [`HybridIdent`] | [`CasedHybridIdent`] | [`LowerHybridIdent`] / [`UpperHybridIdent`] |
| [`KebabIdent`]  | [`CasedKebabIdent`]  | [`LowerKebabIdent`]  / [`UpperKebabIdent`]  |
| [`SnakeIdent`]  | [`CasedSnakeIdent`]  | [`LowerSnakeIdent`]  / [`UpperSnakeIdent`]  |

## Selecting a Preset

To break down the selection process, you can first start by considering the general form of the identifier you want.

**Start by selecting one of...**

| **Format**      | **Delimiters used...** | **Prefers word separation by...** | **Case restrictions apply to...** |
|-----------------|------------------------|-----------------------------------|-----------------------------------|
| [`CamelIdent`]  | Low Line (`_`)         | ...alternating casing             | ...only chunk start characters    |
| [`HybridIdent`] | Flat Line (`_` / `-`)  | ...alternating casing             | ...only chunk start characters    |
| [`KebabIdent`]  | Hyphen-Minus (`-`)     | ...explicitly using delimiters    | ...all characters                 |
| [`SnakeIdent`]  | Low Line (`_`)         | ...explicitly using delimiters    | ...all characters                 |

> **NOTE:**
> For alternate-casing identifier formats, delimiters can be used if separation of words through casing is not possible (like with uncased letters, symbols, etc).

**Then, select a case profile based on the case restrictions you want...**

| **Casing**           | **Case Restrictions**                 |
|----------------------|---------------------------------------|
| Mixed *(Unprefixed)* | No specific case restrictions.        |
| Cased                | Either lower or upper, but not mixed. |
| Lower                | Must be lowercase or uncased.         |
| Upper                | Must be uppercase or uncased.         |

All predefined presets follow the naming pattern of `{CasingPrefix}{IdentFormat}` (e.g. [`UpperCamelIdent`], with the exception that "Mixed" has no prefix).

Select the strictest preset that matches your requirement.

```rust
# use typed_ident::presets::unicode::*;
// A basic camel identifier is the most permissive choice.
assert!(CamelIdent::new("lowerCamel").is_ok());
assert!(CamelIdent::new("UpperCamel").is_ok());
assert!(CamelIdent::new("lowerCamel_UpperCamel").is_ok());

// A cased-camel identifier must be consistently either lower- or upper-cased.
assert!(CasedCamelIdent::new("lowerCamel").is_ok());
assert!(CasedCamelIdent::new("UpperCamel").is_ok());
assert!(CasedCamelIdent::new("lowerCamel_UpperCamel").is_err());

// A lower-cased camel identifier requires that it's lowerCamelCase
assert!(LowerCamelIdent::new("lowerCamel").is_ok());
assert!(LowerCamelIdent::new("UpperCamel").is_err());
assert!(LowerCamelIdent::new("lowerCamel_UpperCamel").is_err());

// An upper-cased camel identifier requires that it's UpperCamelCase
assert!(UpperCamelIdent::new("lowerCamel").is_err());
assert!(UpperCamelIdent::new("UpperCamel").is_ok());
assert!(UpperCamelIdent::new("lowerCamel_UpperCamel").is_err());
```
