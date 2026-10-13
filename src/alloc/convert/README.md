String conversion to another cased format.

The main conversions traits defined by this module are:

* [`ToLowerCamel`] - for identifiers of the format `lowerCamelCase`.
* [`ToLowerHybrid`] - for identifiers of the format `-_lowerHybridCase`.
* [`ToLowerKebab`] - for identifiers of the format `lower-kebab-case`.
* [`ToLowerSnake`] - for identifiers of the format `lower_snake_case`.
* [`ToUpperCamel`] - for identifiers of the format `UpperCamelCase`.
* [`ToUpperHybrid`] - for identifiers of the format `-_UpperHybridCase`.
* [`ToUpperKebab`] - for identifiers of the format `UPPER-KEBAB-CASE`.
* [`ToUpperSnake`] - for identifiers of the format `UPPER_SNAKE_CASE`.

If you include the trait, then you should be able to use any of the defined methods, as all valid identifiers support these conversion operations.

# How Does Conversion Work?

It's basically a shortcut for calling the equivalent formatting traits from the [`fmt`] module over a string destination buffer, then returning the string to you. Failure to push into the string buffer will panic.

You should see the [`fmt`] modules for more information (especially regarding the different forms; canonical, decorated, and delimited).

If you don't need to own the final result, but only need something displayable, then you should prefer the [`fmt`] variants.

[`fmt`]: crate::fmt

# Type Assignments

The type produced by a conversion function is generic, so you'll need to suggest the final resulting type, similar to what you must do for something like the `collect` function on an iterator.

All types will support binding to `String` and `Box<str>`.

If you are binding to a typed identifier, then the type in question needs to be one that can be considered a superset of the target type. So for instance, if you were using the `ToLowerKebab` trait, then you could assign the output of those methods to any one of the following boxed identifier types:

- `LowerKebabIdent`
- `CasedKebabIdent` (superset of `LowerKebabIdent`)
- `KebabIdent` (superset of `CasedKebabIdent`)
- `LowerHybridIdent` (superset of `LowerKebabIdent`)
- `CasedHybridIdent` (superset of `CasedKebabIdent`)
- `HybridIdent` (superset of `KebabIdent`)


```rust
# use typed_ident::convert::*;
# use typed_ident::presets::unicode::*;
let ident = LowerSnakeIdent::new("example_ident")?;
let _converted: String = ident.to_lower_kebab()?;
let _converted: Box<str> = ident.to_lower_kebab()?;
let _converted: Box<LowerKebabIdent> = ident.to_lower_kebab()?;
let _converted: Box<CasedKebabIdent> = ident.to_lower_kebab()?;
let _converted: Box<KebabIdent> = ident.to_lower_kebab()?;
let _converted: Box<LowerHybridIdent> = ident.to_lower_kebab()?;
let _converted: Box<CasedHybridIdent> = ident.to_lower_kebab()?;
let _converted: Box<HybridIdent> = ident.to_lower_kebab()?;
# Ok::<(), typed_ident::Error>(())
```

But note that assignment to something completely of a different format would not be allowed. We cannot assign to `CamelIdent` or `SnakeIdent` formats.

```rust,compile_fail
# use typed_ident::convert::*;
# use typed_ident::presets::unicode::*;
let ident = LowerSnakeIdent::new("example_ident")?;
let _converted: Box<LowerCamelIdent> = ident.to_lower_kebab()?;
let _converted: Box<CasedCamelIdent> = ident.to_lower_kebab()?;
let _converted: Box<CamelIdent> = ident.to_lower_kebab()?;
let _converted: Box<LowerSnakeIdent> = ident.to_lower_kebab()?;
let _converted: Box<CasedSnakeIdent> = ident.to_lower_kebab()?;
let _converted: Box<SnakeIdent> = ident.to_lower_kebab()?;
# Ok::<(), typed_ident::Error>(())
```

# Warning: Case Mappings

Some cased Unicode characters don't have alternate case mappings.

This could be either because the letter has no alternate-cased letter, *or* because the code point is more of a symbol than a letter so case mapping should not alter it.

This means you *could* have a situation where conversion fails if you are too strict on the target type.

```rust
# use typed_ident::convert::*;
# use typed_ident::presets::unicode::*;
let ident = LowerSnakeIdent::new("ident_ᴨ")?;
let upper_camel_ish: String = ident.to_upper_camel()?;

// Chunk boundary preserved, but `ᴨ` has no uppercase mapping.
assert_eq!(upper_camel_ish, "Ident_ᴨ");

// The result therefore does not satisfy the strict UpperCamel profile.
assert!(UpperCamelIdent::new(&upper_camel_ish).is_err());
assert!(ident.to_upper_camel::<Box<UpperCamelIdent>>().is_err());

// A profile that does not require uppercase chunk starts accepts it.
assert!(CamelIdent::new(&upper_camel_ish).is_ok());
assert!(ident.to_upper_camel::<Box<CamelIdent>>().is_ok());
# Ok::<(), typed_ident::Error>(())
```

Regardless of this, the user has to attempt to construct this situation, as such characters are not common.

But if you don't have specific requirements for the target type of something post-conversion, it could be good idea to select a looser type post-conversion (like an unprefixed variant, `CamelIdent`, `KebabIdent`, etc). Or, if you just want to recall the formatting but don't need it to be typed, assign it to something completely lacking of format checks (`String`, `Box<str>`).

> **NOTE:** If you are really curious to see more examples of such characters, check the code repository for this project.
>
> Specifically, there's a helper project named `typed-ident-generator`, under the `generator` directory. Under that project, there's some code at `generator/src/commands/analyze_tables.rs`, where a complete list of such characters exist, as well as the code for analyzing the updated unicode tables to find if more such characters are added.
>
> An example of such oddities are mathematic symbols. We don't want to change what they are because, while they appear to be lowercase/uppercase, they are more symbolic in nature.
